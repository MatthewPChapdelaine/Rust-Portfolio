// Sorting Algorithms with Benchmarking
//
// COMPILE & RUN:
//   rustc -O sorting_algorithms.rs && ./sorting_algorithms
//
// For more accurate benchmarks, compile with optimizations (-O flag)
//
// This program implements various sorting algorithms and benchmarks them

use std::time::{Duration, Instant};
use std::fmt;

// ============================================================================
// SORTING ALGORITHMS
// ============================================================================

/// Bubble Sort - O(n²) time complexity
pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        let mut swapped = false;
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        // Early termination if no swaps occurred
        if !swapped {
            break;
        }
    }
}

/// Selection Sort - O(n²) time complexity
pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        let mut min_idx = i;
        for j in (i + 1)..len {
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }
        if min_idx != i {
            arr.swap(i, min_idx);
        }
    }
}

/// Insertion Sort - O(n²) time complexity, efficient for small arrays
pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

/// Merge Sort - O(n log n) time complexity, stable sort
pub fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;
    let mut left = arr[..mid].to_vec();
    let mut right = arr[mid..].to_vec();

    merge_sort(&mut left);
    merge_sort(&mut right);

    merge(arr, &left, &right);
}

fn merge<T: Ord + Clone>(arr: &mut [T], left: &[T], right: &[T]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i].clone();
            i += 1;
        } else {
            arr[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i].clone();
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}

/// Quick Sort - O(n log n) average, O(n²) worst case
pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    quick_sort_helper(arr, 0, len - 1);
}

fn quick_sort_helper<T: Ord>(arr: &mut [T], low: usize, high: usize) {
    if low < high {
        let pivot = partition(arr, low, high);
        if pivot > 0 {
            quick_sort_helper(arr, low, pivot - 1);
        }
        quick_sort_helper(arr, pivot + 1, high);
    }
}

fn partition<T: Ord>(arr: &mut [T], low: usize, high: usize) -> usize {
    let pivot_idx = high;
    let mut i = low;

    for j in low..high {
        if arr[j] <= arr[pivot_idx] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, pivot_idx);
    i
}

/// Heap Sort - O(n log n) time complexity
pub fn heap_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();

    // Build max heap
    for i in (0..len / 2).rev() {
        heapify(arr, len, i);
    }

    // Extract elements from heap one by one
    for i in (1..len).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0);
    }
}

fn heapify<T: Ord>(arr: &mut [T], heap_size: usize, root: usize) {
    let mut largest = root;
    let left = 2 * root + 1;
    let right = 2 * root + 2;

    if left < heap_size && arr[left] > arr[largest] {
        largest = left;
    }

    if right < heap_size && arr[right] > arr[largest] {
        largest = right;
    }

    if largest != root {
        arr.swap(root, largest);
        heapify(arr, heap_size, largest);
    }
}

/// Counting Sort - O(n + k) time complexity, for integers only
pub fn counting_sort(arr: &mut [i32]) {
    if arr.is_empty() {
        return;
    }

    let max = *arr.iter().max().unwrap();
    let min = *arr.iter().min().unwrap();
    let range = (max - min + 1) as usize;

    let mut count = vec![0; range];
    let mut output = vec![0; arr.len()];

    // Store count of each element
    for &num in arr.iter() {
        count[(num - min) as usize] += 1;
    }

    // Change count[i] to contain actual position
    for i in 1..range {
        count[i] += count[i - 1];
    }

    // Build output array
    for &num in arr.iter().rev() {
        let idx = (num - min) as usize;
        output[count[idx] - 1] = num;
        count[idx] -= 1;
    }

    // Copy output to arr
    arr.copy_from_slice(&output);
}

// ============================================================================
// BENCHMARKING
// ============================================================================

#[derive(Debug, Clone)]
struct BenchmarkResult {
    algorithm: String,
    size: usize,
    duration: Duration,
    sorted: bool,
}

impl fmt::Display for BenchmarkResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let micros = self.duration.as_micros();
        let status = if self.sorted { "✓" } else { "✗" };
        write!(
            f,
            "{} {:20} | Size: {:6} | Time: {:8} μs",
            status, self.algorithm, self.size, micros
        )
    }
}

struct Benchmark;

impl Benchmark {
    /// Benchmark a sorting function
    fn benchmark<F>(name: &str, mut sort_fn: F, data: &[i32]) -> BenchmarkResult
    where
        F: FnMut(&mut [i32]),
    {
        let mut arr = data.to_vec();
        
        let start = Instant::now();
        sort_fn(&mut arr);
        let duration = start.elapsed();

        let sorted = Self::is_sorted(&arr);

        BenchmarkResult {
            algorithm: name.to_string(),
            size: data.len(),
            duration,
            sorted,
        }
    }

    /// Check if array is sorted
    fn is_sorted<T: Ord>(arr: &[T]) -> bool {
        arr.windows(2).all(|w| w[0] <= w[1])
    }

    /// Run comprehensive benchmark suite
    fn run_suite(sizes: &[usize]) -> Vec<Vec<BenchmarkResult>> {
        let mut all_results = Vec::new();

        for &size in sizes {
            println!("\n--- Benchmarking with {} elements ---", size);
            let data = Self::generate_random_data(size);
            let mut results = Vec::new();

            // Bubble sort (skip for very large arrays)
            if size <= 10000 {
                results.push(Self::benchmark("Bubble Sort", bubble_sort, &data));
            }

            // Selection sort (skip for very large arrays)
            if size <= 10000 {
                results.push(Self::benchmark("Selection Sort", selection_sort, &data));
            }

            // Insertion sort
            if size <= 10000 {
                results.push(Self::benchmark("Insertion Sort", insertion_sort, &data));
            }

            // Merge sort
            results.push(Self::benchmark("Merge Sort", merge_sort, &data));

            // Quick sort
            results.push(Self::benchmark("Quick Sort", quick_sort, &data));

            // Heap sort
            results.push(Self::benchmark("Heap Sort", heap_sort, &data));

            // Counting sort (convert to i32 for this)
            results.push(Self::benchmark("Counting Sort", counting_sort, &data));

            // Print results for this size
            for result in &results {
                println!("  {}", result);
            }

            all_results.push(results);
        }

        all_results
    }

    /// Generate random test data
    fn generate_random_data(size: usize) -> Vec<i32> {
        // Simple pseudo-random number generation
        let mut data = Vec::with_capacity(size);
        let mut seed = 12345u64;
        
        for _ in 0..size {
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            data.push(((seed / 65536) % 32768) as i32);
        }
        
        data
    }

    /// Generate sorted data
    fn generate_sorted_data(size: usize) -> Vec<i32> {
        (0..size as i32).collect()
    }

    /// Generate reverse sorted data
    fn generate_reverse_sorted_data(size: usize) -> Vec<i32> {
        (0..size as i32).rev().collect()
    }

    /// Generate nearly sorted data
    fn generate_nearly_sorted_data(size: usize) -> Vec<i32> {
        let mut data: Vec<i32> = (0..size as i32).collect();
        // Swap some random elements
        for i in (0..size).step_by(10) {
            if i + 1 < size {
                data.swap(i, i + 1);
            }
        }
        data
    }
}

// ============================================================================
// DEMO AND TESTING
// ============================================================================

fn demo_sorting_algorithms() {
    println!("=== Sorting Algorithm Demonstrations ===\n");

    // Small array for visual demonstration
    let test_data = vec![64, 34, 25, 12, 22, 11, 90, 88, 45, 50];

    println!("Original array: {:?}\n", test_data);

    // Bubble Sort
    let mut arr = test_data.clone();
    bubble_sort(&mut arr);
    println!("Bubble Sort:    {:?}", arr);

    // Selection Sort
    let mut arr = test_data.clone();
    selection_sort(&mut arr);
    println!("Selection Sort: {:?}", arr);

    // Insertion Sort
    let mut arr = test_data.clone();
    insertion_sort(&mut arr);
    println!("Insertion Sort: {:?}", arr);

    // Merge Sort
    let mut arr = test_data.clone();
    merge_sort(&mut arr);
    println!("Merge Sort:     {:?}", arr);

    // Quick Sort
    let mut arr = test_data.clone();
    quick_sort(&mut arr);
    println!("Quick Sort:     {:?}", arr);

    // Heap Sort
    let mut arr = test_data.clone();
    heap_sort(&mut arr);
    println!("Heap Sort:      {:?}", arr);

    // Counting Sort
    let mut arr = test_data.clone();
    counting_sort(&mut arr);
    println!("Counting Sort:  {:?}", arr);
}

fn test_edge_cases() {
    println!("\n=== Testing Edge Cases ===\n");

    // Empty array
    let mut empty: Vec<i32> = vec![];
    quick_sort(&mut empty);
    println!("Empty array: {:?} - {}", empty, if Benchmark::is_sorted(&empty) { "✓" } else { "✗" });

    // Single element
    let mut single = vec![42];
    quick_sort(&mut single);
    println!("Single element: {:?} - {}", single, if Benchmark::is_sorted(&single) { "✓" } else { "✗" });

    // Already sorted
    let mut sorted = vec![1, 2, 3, 4, 5];
    quick_sort(&mut sorted);
    println!("Already sorted: {:?} - {}", sorted, if Benchmark::is_sorted(&sorted) { "✓" } else { "✗" });

    // Reverse sorted
    let mut reverse = vec![5, 4, 3, 2, 1];
    quick_sort(&mut reverse);
    println!("Reverse sorted: {:?} - {}", reverse, if Benchmark::is_sorted(&reverse) { "✓" } else { "✗" });

    // Duplicates
    let mut duplicates = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    quick_sort(&mut duplicates);
    println!("With duplicates: {:?} - {}", duplicates, if Benchmark::is_sorted(&duplicates) { "✓" } else { "✗" });
}

fn benchmark_different_inputs() {
    println!("\n=== Benchmarking Different Input Types ===\n");
    let size = 5000;

    println!("Array size: {}\n", size);

    // Random data
    let random = Benchmark::generate_random_data(size);
    let result = Benchmark::benchmark("Quick Sort (Random)", quick_sort, &random);
    println!("{}", result);

    // Sorted data
    let sorted = Benchmark::generate_sorted_data(size);
    let result = Benchmark::benchmark("Quick Sort (Sorted)", quick_sort, &sorted);
    println!("{}", result);

    // Reverse sorted
    let reverse = Benchmark::generate_reverse_sorted_data(size);
    let result = Benchmark::benchmark("Quick Sort (Reverse)", quick_sort, &reverse);
    println!("{}", result);

    // Nearly sorted
    let nearly = Benchmark::generate_nearly_sorted_data(size);
    let result = Benchmark::benchmark("Quick Sort (Nearly Sorted)", quick_sort, &nearly);
    println!("{}", result);
}

fn main() {
    // Demonstrate sorting algorithms
    demo_sorting_algorithms();

    // Test edge cases
    test_edge_cases();

    // Benchmark different input types
    benchmark_different_inputs();

    // Run comprehensive benchmark suite
    println!("\n=== Comprehensive Benchmark Suite ===");
    let sizes = vec![100, 1000, 5000];
    Benchmark::run_suite(&sizes);

    // Performance comparison summary
    println!("\n=== Performance Summary ===");
    println!("Best for small arrays (< 50 elements): Insertion Sort");
    println!("Best for general use: Quick Sort or Merge Sort");
    println!("Best for stability: Merge Sort");
    println!("Best for memory efficiency: Heap Sort (in-place)");
    println!("Best for integer arrays with limited range: Counting Sort");
    
    println!("\n=== Demo Complete ===");
}
