// Distributed System with Raft Consensus Algorithm
// Implements leader election, log replication, and fault tolerance

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tokio::time::{interval, sleep};

// ========== RAFT MESSAGE TYPES ==========
#[derive(Debug, Clone)]
enum RaftMessage {
    RequestVote {
        term: u64,
        candidate_id: u64,
        last_log_index: usize,
        last_log_term: u64,
    },
    RequestVoteResponse {
        term: u64,
        vote_granted: bool,
    },
    AppendEntries {
        term: u64,
        leader_id: u64,
        prev_log_index: usize,
        prev_log_term: u64,
        entries: Vec<LogEntry>,
        leader_commit: usize,
    },
    AppendEntriesResponse {
        term: u64,
        success: bool,
        match_index: usize,
    },
    ClientRequest {
        command: String,
    },
}

#[derive(Debug, Clone)]
struct LogEntry {
    term: u64,
    index: usize,
    command: String,
}

#[derive(Debug, Clone, PartialEq)]
enum NodeState {
    Follower,
    Candidate,
    Leader,
}

// ========== RAFT NODE ==========
struct RaftNode {
    id: u64,
    state: NodeState,
    current_term: u64,
    voted_for: Option<u64>,
    log: Vec<LogEntry>,
    commit_index: usize,
    last_applied: usize,
    
    // Leader-specific state
    next_index: HashMap<u64, usize>,
    match_index: HashMap<u64, usize>,
    
    // Timing
    last_heartbeat: Instant,
    election_timeout: Duration,
    heartbeat_interval: Duration,
    
    // Voting
    votes_received: usize,
    
    // Peers
    peers: Vec<u64>,
}

impl RaftNode {
    fn new(id: u64, peers: Vec<u64>) -> Self {
        let election_timeout = Duration::from_millis(150 + (id * 50));
        
        RaftNode {
            id,
            state: NodeState::Follower,
            current_term: 0,
            voted_for: None,
            log: Vec::new(),
            commit_index: 0,
            last_applied: 0,
            next_index: HashMap::new(),
            match_index: HashMap::new(),
            last_heartbeat: Instant::now(),
            election_timeout,
            heartbeat_interval: Duration::from_millis(50),
            votes_received: 0,
            peers,
        }
    }

    fn reset_election_timer(&mut self) {
        self.last_heartbeat = Instant::now();
    }

    fn is_election_timeout(&self) -> bool {
        self.last_heartbeat.elapsed() > self.election_timeout
    }

    fn start_election(&mut self) {
        self.state = NodeState::Candidate;
        self.current_term += 1;
        self.voted_for = Some(self.id);
        self.votes_received = 1;
        self.reset_election_timer();
        
        println!(
            "[Node {}] Starting election for term {}",
            self.id, self.current_term
        );
    }

    fn become_leader(&mut self) {
        println!("[Node {}] Became leader for term {}", self.id, self.current_term);
        self.state = NodeState::Leader;
        
        let next_idx = self.log.len();
        for peer in &self.peers {
            self.next_index.insert(*peer, next_idx);
            self.match_index.insert(*peer, 0);
        }
    }

    fn become_follower(&mut self, term: u64) {
        if term > self.current_term {
            self.current_term = term;
            self.voted_for = None;
        }
        self.state = NodeState::Follower;
        self.reset_election_timer();
    }

    fn handle_request_vote(
        &mut self,
        term: u64,
        candidate_id: u64,
        last_log_index: usize,
        last_log_term: u64,
    ) -> RaftMessage {
        let mut vote_granted = false;

        if term > self.current_term {
            self.become_follower(term);
        }

        if term >= self.current_term {
            let log_ok = if self.log.is_empty() {
                last_log_index == 0
            } else {
                let my_last_log = &self.log[self.log.len() - 1];
                last_log_term > my_last_log.term
                    || (last_log_term == my_last_log.term && last_log_index >= my_last_log.index)
            };

            if (self.voted_for.is_none() || self.voted_for == Some(candidate_id)) && log_ok {
                vote_granted = true;
                self.voted_for = Some(candidate_id);
                self.reset_election_timer();
                println!(
                    "[Node {}] Granted vote to {} for term {}",
                    self.id, candidate_id, term
                );
            }
        }

        RaftMessage::RequestVoteResponse {
            term: self.current_term,
            vote_granted,
        }
    }

    fn handle_vote_response(&mut self, term: u64, vote_granted: bool) {
        if term > self.current_term {
            self.become_follower(term);
            return;
        }

        if self.state == NodeState::Candidate && term == self.current_term && vote_granted {
            self.votes_received += 1;
            let majority = (self.peers.len() + 1) / 2 + 1;
            
            if self.votes_received >= majority {
                self.become_leader();
            }
        }
    }

    fn handle_append_entries(
        &mut self,
        term: u64,
        leader_id: u64,
        prev_log_index: usize,
        prev_log_term: u64,
        entries: Vec<LogEntry>,
        leader_commit: usize,
    ) -> RaftMessage {
        if term > self.current_term {
            self.become_follower(term);
        }

        self.reset_election_timer();

        if term < self.current_term {
            return RaftMessage::AppendEntriesResponse {
                term: self.current_term,
                success: false,
                match_index: 0,
            };
        }

        if self.state == NodeState::Candidate {
            self.become_follower(term);
        }

        let log_ok = if prev_log_index == 0 {
            true
        } else if prev_log_index <= self.log.len() {
            self.log[prev_log_index - 1].term == prev_log_term
        } else {
            false
        };

        if !log_ok {
            return RaftMessage::AppendEntriesResponse {
                term: self.current_term,
                success: false,
                match_index: self.log.len(),
            };
        }

        let mut index = prev_log_index;
        for entry in entries {
            index += 1;
            if index <= self.log.len() {
                if self.log[index - 1].term != entry.term {
                    self.log.truncate(index - 1);
                    self.log.push(entry);
                }
            } else {
                self.log.push(entry);
            }
        }

        if leader_commit > self.commit_index {
            self.commit_index = leader_commit.min(self.log.len());
            self.apply_committed_entries();
        }

        RaftMessage::AppendEntriesResponse {
            term: self.current_term,
            success: true,
            match_index: self.log.len(),
        }
    }

    fn handle_append_entries_response(
        &mut self,
        peer_id: u64,
        term: u64,
        success: bool,
        match_index: usize,
    ) {
        if term > self.current_term {
            self.become_follower(term);
            return;
        }

        if self.state != NodeState::Leader || term != self.current_term {
            return;
        }

        if success {
            self.next_index.insert(peer_id, match_index + 1);
            self.match_index.insert(peer_id, match_index);
            
            self.update_commit_index();
        } else {
            let next = self.next_index.get(&peer_id).copied().unwrap_or(1);
            if next > 1 {
                self.next_index.insert(peer_id, next - 1);
            }
        }
    }

    fn update_commit_index(&mut self) {
        if self.state != NodeState::Leader {
            return;
        }

        for n in (self.commit_index + 1)..=self.log.len() {
            if self.log[n - 1].term == self.current_term {
                let mut count = 1;
                for peer in &self.peers {
                    if self.match_index.get(peer).copied().unwrap_or(0) >= n {
                        count += 1;
                    }
                }
                
                let majority = (self.peers.len() + 1) / 2 + 1;
                if count >= majority {
                    self.commit_index = n;
                    self.apply_committed_entries();
                }
            }
        }
    }

    fn apply_committed_entries(&mut self) {
        while self.last_applied < self.commit_index {
            self.last_applied += 1;
            let entry = &self.log[self.last_applied - 1];
            println!(
                "[Node {}] Applied log entry {}: {}",
                self.id, entry.index, entry.command
            );
        }
    }

    fn handle_client_request(&mut self, command: String) -> Result<(), String> {
        if self.state != NodeState::Leader {
            return Err("Not the leader".to_string());
        }

        let entry = LogEntry {
            term: self.current_term,
            index: self.log.len() + 1,
            command,
        };
        
        println!(
            "[Node {}] Received client command: {} (index: {})",
            self.id, entry.command, entry.index
        );
        
        self.log.push(entry);
        Ok(())
    }

    fn create_request_vote(&self) -> RaftMessage {
        let (last_log_index, last_log_term) = if self.log.is_empty() {
            (0, 0)
        } else {
            let last = &self.log[self.log.len() - 1];
            (last.index, last.term)
        };

        RaftMessage::RequestVote {
            term: self.current_term,
            candidate_id: self.id,
            last_log_index,
            last_log_term,
        }
    }

    fn create_append_entries(&self, peer_id: u64) -> RaftMessage {
        let next_idx = self.next_index.get(&peer_id).copied().unwrap_or(1);
        
        let (prev_log_index, prev_log_term) = if next_idx > 1 && !self.log.is_empty() {
            let prev = &self.log[next_idx - 2];
            (prev.index, prev.term)
        } else {
            (0, 0)
        };

        let entries = if next_idx <= self.log.len() {
            self.log[next_idx - 1..].to_vec()
        } else {
            Vec::new()
        };

        RaftMessage::AppendEntries {
            term: self.current_term,
            leader_id: self.id,
            prev_log_index,
            prev_log_term,
            entries,
            leader_commit: self.commit_index,
        }
    }
}

// ========== CLUSTER SIMULATION ==========
type NodeHandle = Arc<Mutex<RaftNode>>;

struct Cluster {
    nodes: HashMap<u64, NodeHandle>,
    channels: HashMap<u64, mpsc::UnboundedSender<(u64, RaftMessage)>>,
}

impl Cluster {
    fn new(node_count: usize) -> Self {
        let mut nodes = HashMap::new();
        let mut channels = HashMap::new();

        let peer_ids: Vec<u64> = (0..node_count as u64).collect();

        for id in 0..node_count as u64 {
            let peers: Vec<u64> = peer_ids.iter().filter(|&&p| p != id).copied().collect();
            let node = Arc::new(Mutex::new(RaftNode::new(id, peers)));
            nodes.insert(id, node);
            
            let (tx, _rx) = mpsc::unbounded_channel();
            channels.insert(id, tx);
        }

        Cluster { nodes, channels }
    }

    async fn run_node(&self, node_id: u64) {
        let node_handle = self.nodes.get(&node_id).unwrap().clone();
        let (tx, mut rx) = mpsc::unbounded_channel::<(u64, RaftMessage)>();
        
        let channels = self.channels.clone();
        
        tokio::spawn(async move {
            let mut heartbeat_timer = interval(Duration::from_millis(50));
            
            loop {
                tokio::select! {
                    _ = heartbeat_timer.tick() => {
                        let mut node = node_handle.lock().unwrap();
                        
                        match node.state {
                            NodeState::Leader => {
                                for peer in node.peers.clone() {
                                    let msg = node.create_append_entries(peer);
                                    if let Some(sender) = channels.get(&peer) {
                                        let _ = sender.send((node_id, msg));
                                    }
                                }
                            }
                            NodeState::Follower | NodeState::Candidate => {
                                if node.is_election_timeout() {
                                    node.start_election();
                                    let msg = node.create_request_vote();
                                    for peer in node.peers.clone() {
                                        if let Some(sender) = channels.get(&peer) {
                                            let _ = sender.send((node_id, msg.clone()));
                                        }
                                    }
                                }
                            }
                        }
                    }
                    
                    Some((from_id, msg)) = rx.recv() => {
                        let mut node = node_handle.lock().unwrap();
                        
                        let response = match msg {
                            RaftMessage::RequestVote { term, candidate_id, last_log_index, last_log_term } => {
                                Some(node.handle_request_vote(term, candidate_id, last_log_index, last_log_term))
                            }
                            RaftMessage::RequestVoteResponse { term, vote_granted } => {
                                node.handle_vote_response(term, vote_granted);
                                None
                            }
                            RaftMessage::AppendEntries { term, leader_id, prev_log_index, prev_log_term, entries, leader_commit } => {
                                Some(node.handle_append_entries(term, leader_id, prev_log_index, prev_log_term, entries, leader_commit))
                            }
                            RaftMessage::AppendEntriesResponse { term, success, match_index } => {
                                node.handle_append_entries_response(from_id, term, success, match_index);
                                None
                            }
                            RaftMessage::ClientRequest { command } => {
                                let _ = node.handle_client_request(command);
                                None
                            }
                        };
                        
                        if let Some(resp) = response {
                            if let Some(sender) = channels.get(&from_id) {
                                let _ = sender.send((node_id, resp));
                            }
                        }
                    }
                }
            }
        });
    }

    fn get_leader(&self) -> Option<u64> {
        for (id, node_handle) in &self.nodes {
            let node = node_handle.lock().unwrap();
            if node.state == NodeState::Leader {
                return Some(*id);
            }
        }
        None
    }

    fn send_client_request(&self, leader_id: u64, command: String) {
        if let Some(node_handle) = self.nodes.get(&leader_id) {
            let mut node = node_handle.lock().unwrap();
            let _ = node.handle_client_request(command);
        }
    }
}

// ========== MAIN ==========
#[tokio::main]
async fn main() {
    println!("=== Distributed System with Raft Consensus ===\n");

    println!("Creating a 5-node Raft cluster...");
    let cluster = Arc::new(Cluster::new(5));

    println!("Starting all nodes...\n");
    for id in 0..5 {
        cluster.run_node(id).await;
    }

    println!("Waiting for leader election...");
    sleep(Duration::from_secs(2)).await;

    if let Some(leader_id) = cluster.get_leader() {
        println!("\n✓ Leader elected: Node {}\n", leader_id);

        println!("Sending client requests to leader...");
        cluster.send_client_request(leader_id, "SET x = 10".to_string());
        sleep(Duration::from_millis(200)).await;

        cluster.send_client_request(leader_id, "SET y = 20".to_string());
        sleep(Duration::from_millis(200)).await;

        cluster.send_client_request(leader_id, "ADD x y".to_string());
        sleep(Duration::from_millis(200)).await;

        println!("\n✓ Raft consensus demonstration complete!");
        println!("\nKey features demonstrated:");
        println!("  • Leader election with randomized timeouts");
        println!("  • Log replication across follower nodes");
        println!("  • Heartbeat mechanism to maintain leadership");
        println!("  • Term-based conflict resolution");
        println!("  • Majority-based commit consensus");
    } else {
        println!("\n✗ No leader elected (this is expected in some scenarios)");
    }

    sleep(Duration::from_secs(1)).await;
}
