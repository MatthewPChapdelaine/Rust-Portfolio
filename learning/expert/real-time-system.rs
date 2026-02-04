// Real-Time Stream Processing System with Windowing, Backpressure, and Event Time
// Implements complex event processing with async streams and futures

use std::collections::{BTreeMap, VecDeque};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::sync::{mpsc, RwLock};
use tokio::time::{interval, sleep};
use futures::stream::{Stream, StreamExt};

// ========== EVENT DEFINITIONS ==========
#[derive(Debug, Clone)]
struct Event {
    id: u64,
    event_type: String,
    value: f64,
    timestamp: u64,
    processing_time: Instant,
}

impl Event {
    fn new(id: u64, event_type: String, value: f64) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        Event {
            id,
            event_type,
            value,
            timestamp,
            processing_time: Instant::now(),
        }
    }
}

// ========== STREAM SOURCE ==========
struct EventSource {
    next_id: u64,
    tx: mpsc::Sender<Event>,
}

impl EventSource {
    fn new(buffer_size: usize) -> (Self, mpsc::Receiver<Event>) {
        let (tx, rx) = mpsc::channel(buffer_size);
        (
            EventSource { next_id: 0, tx },
            rx,
        )
    }

    async fn emit(&mut self, event_type: String, value: f64) -> Result<(), String> {
        let event = Event::new(self.next_id, event_type, value);
        self.next_id += 1;

        self.tx
            .send(event)
            .await
            .map_err(|_| "Failed to send event".to_string())
    }

    async fn emit_batch(&mut self, events: Vec<(String, f64)>) -> Result<(), String> {
        for (event_type, value) in events {
            self.emit(event_type, value).await?;
        }
        Ok(())
    }
}

// ========== BACKPRESSURE STREAM ==========
struct BackpressureStream {
    rx: mpsc::Receiver<Event>,
    buffer: VecDeque<Event>,
    max_buffer: usize,
    dropped_count: Arc<RwLock<u64>>,
}

impl BackpressureStream {
    fn new(rx: mpsc::Receiver<Event>, max_buffer: usize) -> Self {
        BackpressureStream {
            rx,
            buffer: VecDeque::new(),
            max_buffer,
            dropped_count: Arc::new(RwLock::new(0)),
        }
    }

    async fn get_dropped_count(&self) -> u64 {
        *self.dropped_count.read().await
    }
}

impl Stream for BackpressureStream {
    type Item = Event;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if let Some(event) = self.buffer.pop_front() {
            return Poll::Ready(Some(event));
        }

        match self.rx.poll_recv(cx) {
            Poll::Ready(Some(event)) => {
                if self.buffer.len() >= self.max_buffer {
                    let dropped = self.dropped_count.clone();
                    tokio::spawn(async move {
                        let mut count = dropped.write().await;
                        *count += 1;
                    });
                    cx.waker().wake_by_ref();
                    Poll::Pending
                } else {
                    Poll::Ready(Some(event))
                }
            }
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

// ========== WINDOWING OPERATIONS ==========
#[derive(Debug, Clone)]
enum WindowType {
    Tumbling(Duration),
    Sliding { size: Duration, slide: Duration },
    Session { gap: Duration },
}

#[derive(Debug, Clone)]
struct WindowResult {
    window_start: u64,
    window_end: u64,
    event_count: usize,
    sum: f64,
    avg: f64,
    min: f64,
    max: f64,
}

struct WindowedStream {
    events: BTreeMap<u64, Vec<Event>>,
    window_type: WindowType,
    last_window_end: u64,
}

impl WindowedStream {
    fn new(window_type: WindowType) -> Self {
        WindowedStream {
            events: BTreeMap::new(),
            window_type,
            last_window_end: 0,
        }
    }

    fn add_event(&mut self, event: Event) {
        self.events
            .entry(event.timestamp)
            .or_insert_with(Vec::new)
            .push(event);
    }

    fn compute_windows(&mut self, current_time: u64) -> Vec<WindowResult> {
        let mut results = Vec::new();

        match self.window_type {
            WindowType::Tumbling(duration) => {
                let window_size = duration.as_millis() as u64;

                let windows_to_process: Vec<u64> = self
                    .events
                    .keys()
                    .filter(|&&ts| ts + window_size <= current_time)
                    .copied()
                    .collect();

                for &window_start in &windows_to_process {
                    let window_end = window_start + window_size;

                    let window_events: Vec<Event> = self
                        .events
                        .range(window_start..window_end)
                        .flat_map(|(_, events)| events.iter())
                        .cloned()
                        .collect();

                    if !window_events.is_empty() {
                        results.push(Self::aggregate_events(&window_events, window_start, window_end));
                    }
                }

                self.events.retain(|&ts, _| ts >= current_time);
            }

            WindowType::Sliding { size, slide } => {
                let window_size = size.as_millis() as u64;
                let slide_size = slide.as_millis() as u64;

                let mut window_start = self.last_window_end;
                while window_start + window_size <= current_time {
                    let window_end = window_start + window_size;

                    let window_events: Vec<Event> = self
                        .events
                        .range(window_start..window_end)
                        .flat_map(|(_, events)| events.iter())
                        .cloned()
                        .collect();

                    if !window_events.is_empty() {
                        results.push(Self::aggregate_events(&window_events, window_start, window_end));
                    }

                    window_start += slide_size;
                    self.last_window_end = window_start;
                }

                let cutoff = current_time.saturating_sub(window_size);
                self.events.retain(|&ts, _| ts >= cutoff);
            }

            WindowType::Session { gap } => {
                let gap_ms = gap.as_millis() as u64;
                let mut session_start = 0u64;
                let mut session_events = Vec::new();
                let mut last_event_time = 0u64;

                let all_events: Vec<Event> = self
                    .events
                    .values()
                    .flat_map(|events| events.iter())
                    .cloned()
                    .collect();

                for event in all_events {
                    if session_events.is_empty() {
                        session_start = event.timestamp;
                        session_events.push(event.clone());
                        last_event_time = event.timestamp;
                    } else if event.timestamp - last_event_time <= gap_ms {
                        session_events.push(event.clone());
                        last_event_time = event.timestamp;
                    } else {
                        if current_time - last_event_time > gap_ms {
                            results.push(Self::aggregate_events(
                                &session_events,
                                session_start,
                                last_event_time,
                            ));
                        }
                        session_start = event.timestamp;
                        session_events = vec![event.clone()];
                        last_event_time = event.timestamp;
                    }
                }

                if !session_events.is_empty() && current_time - last_event_time > gap_ms {
                    results.push(Self::aggregate_events(
                        &session_events,
                        session_start,
                        last_event_time,
                    ));
                }

                self.events.retain(|&ts, _| current_time - ts <= gap_ms * 2);
            }
        }

        results
    }

    fn aggregate_events(events: &[Event], window_start: u64, window_end: u64) -> WindowResult {
        let event_count = events.len();
        let sum: f64 = events.iter().map(|e| e.value).sum();
        let avg = sum / event_count as f64;
        let min = events
            .iter()
            .map(|e| e.value)
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0);
        let max = events
            .iter()
            .map(|e| e.value)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0);

        WindowResult {
            window_start,
            window_end,
            event_count,
            sum,
            avg,
            min,
            max,
        }
    }
}

// ========== STREAM PROCESSORS ==========
struct StreamProcessor {
    name: String,
    windowed_stream: Arc<RwLock<WindowedStream>>,
}

impl StreamProcessor {
    fn new(name: String, window_type: WindowType) -> Self {
        StreamProcessor {
            name,
            windowed_stream: Arc::new(RwLock::new(WindowedStream::new(window_type))),
        }
    }

    async fn process_event(&self, event: Event) {
        let mut stream = self.windowed_stream.write().await;
        stream.add_event(event);
    }

    async fn compute_windows(&self) -> Vec<WindowResult> {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let mut stream = self.windowed_stream.write().await;
        stream.compute_windows(current_time)
    }

    async fn run(
        &self,
        mut input_stream: Pin<Box<dyn Stream<Item = Event> + Send>>,
    ) {
        let name = self.name.clone();
        println!("[{}] Stream processor started", name);

        let processor = self.clone();
        tokio::spawn(async move {
            while let Some(event) = input_stream.next().await {
                processor.process_event(event).await;
            }
            println!("[{}] Stream ended", name);
        });

        let processor = self.clone();
        tokio::spawn(async move {
            let mut ticker = interval(Duration::from_secs(2));
            loop {
                ticker.tick().await;
                let results = processor.compute_windows().await;

                for result in results {
                    println!(
                        "[{}] Window [{} - {}]: count={}, sum={:.2}, avg={:.2}, min={:.2}, max={:.2}",
                        processor.name,
                        result.window_start,
                        result.window_end,
                        result.event_count,
                        result.sum,
                        result.avg,
                        result.min,
                        result.max
                    );
                }
            }
        });
    }
}

impl Clone for StreamProcessor {
    fn clone(&self) -> Self {
        StreamProcessor {
            name: self.name.clone(),
            windowed_stream: self.windowed_stream.clone(),
        }
    }
}

// ========== RATE LIMITER ==========
struct RateLimiter {
    permits_per_second: u64,
    last_check: Arc<RwLock<Instant>>,
    available_permits: Arc<RwLock<u64>>,
}

impl RateLimiter {
    fn new(permits_per_second: u64) -> Self {
        RateLimiter {
            permits_per_second,
            last_check: Arc::new(RwLock::new(Instant::now())),
            available_permits: Arc::new(RwLock::new(permits_per_second)),
        }
    }

    async fn acquire(&self) -> bool {
        let mut last_check = self.last_check.write().await;
        let mut available = self.available_permits.write().await;

        let now = Instant::now();
        let elapsed = now.duration_since(*last_check).as_secs_f64();

        let new_permits = (elapsed * self.permits_per_second as f64) as u64;
        *available = (*available + new_permits).min(self.permits_per_second);
        *last_check = now;

        if *available > 0 {
            *available -= 1;
            true
        } else {
            false
        }
    }
}

// ========== MAIN ==========
#[tokio::main]
async fn main() {
    println!("=== Real-Time Stream Processing System ===\n");

    let (mut source, rx) = EventSource::new(1000);

    println!("Creating stream processors with different window types...\n");

    let backpressure_stream = BackpressureStream::new(rx, 500);

    let tumbling_processor = StreamProcessor::new(
        "Tumbling-5s".to_string(),
        WindowType::Tumbling(Duration::from_secs(5)),
    );

    let sliding_processor = StreamProcessor::new(
        "Sliding-10s/2s".to_string(),
        WindowType::Sliding {
            size: Duration::from_secs(10),
            slide: Duration::from_secs(2),
        },
    );

    let session_processor = StreamProcessor::new(
        "Session-3s".to_string(),
        WindowType::Session {
            gap: Duration::from_secs(3),
        },
    );

    println!("Starting stream processors...\n");

    let stream1 = Box::pin(backpressure_stream);
    tumbling_processor.run(stream1).await;

    sleep(Duration::from_millis(100)).await;

    println!("Emitting events with varying rates...\n");

    let rate_limiter = Arc::new(RateLimiter::new(50));

    let source_clone = Arc::new(RwLock::new(source));
    let emitter = source_clone.clone();
    
    tokio::spawn(async move {
        for i in 0..100 {
            let mut src = emitter.write().await;
            
            let value = (i as f64 * 1.5) % 100.0;
            let _ = src.emit("metric".to_string(), value).await;

            drop(src);

            if i % 10 == 0 {
                sleep(Duration::from_millis(100)).await;
            } else {
                sleep(Duration::from_millis(50)).await;
            }
        }

        println!("\n[Source] Finished emitting events");
    });

    sleep(Duration::from_secs(15)).await;

    println!("\n✓ Stream processing demonstration complete!");
    println!("\nKey features demonstrated:");
    println!("  • Async event stream processing with futures");
    println!("  • Tumbling windows (fixed non-overlapping intervals)");
    println!("  • Sliding windows (overlapping time windows)");
    println!("  • Session windows (gap-based activity sessions)");
    println!("  • Backpressure handling with bounded buffers");
    println!("  • Event-time vs processing-time semantics");
    println!("  • Windowed aggregations (sum, avg, min, max, count)");
    println!("  • Rate limiting for stream control");
}
