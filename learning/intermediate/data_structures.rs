// Data Structures: Linked List, Binary Search Tree, and HashMap implementations
//
// COMPILE & RUN:
//   rustc data_structures.rs && ./data_structures
//
// This program implements common data structures from scratch in Rust

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::fmt::Debug;

// ============================================================================
// SINGLY LINKED LIST
// ============================================================================

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Link<T>,
}

/// Singly Linked List implementation
#[derive(Debug)]
pub struct LinkedList<T> {
    head: Link<T>,
    size: usize,
}

impl<T> LinkedList<T> {
    /// Create a new empty linked list
    pub fn new() -> Self {
        LinkedList {
            head: None,
            size: 0,
        }
    }

    /// Push element to the front
    pub fn push_front(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.size += 1;
    }

    /// Pop element from the front
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.data
        })
    }

    /// Peek at the front element
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    /// Get the size of the list
    pub fn len(&self) -> usize {
        self.size
    }

    /// Check if the list is empty
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Iterate over the list
    pub fn iter(&self) -> LinkedListIter<T> {
        LinkedListIter {
            current: self.head.as_deref(),
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut node) = current {
            current = node.next.take();
        }
    }
}

pub struct LinkedListIter<'a, T> {
    current: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| {
            self.current = node.next.as_deref();
            &node.data
        })
    }
}

// ============================================================================
// BINARY SEARCH TREE
// ============================================================================

type TreeLink<T> = Option<Box<TreeNode<T>>>;

#[derive(Debug)]
struct TreeNode<T: Ord> {
    data: T,
    left: TreeLink<T>,
    right: TreeLink<T>,
}

/// Binary Search Tree implementation
#[derive(Debug)]
pub struct BinarySearchTree<T: Ord> {
    root: TreeLink<T>,
    size: usize,
}

impl<T: Ord + Debug> BinarySearchTree<T> {
    /// Create a new empty BST
    pub fn new() -> Self {
        BinarySearchTree {
            root: None,
            size: 0,
        }
    }

    /// Insert a value into the BST
    pub fn insert(&mut self, data: T) {
        if Self::insert_recursive(&mut self.root, data) {
            self.size += 1;
        }
    }

    fn insert_recursive(node: &mut TreeLink<T>, data: T) -> bool {
        match node {
            None => {
                *node = Some(Box::new(TreeNode {
                    data,
                    left: None,
                    right: None,
                }));
                true
            }
            Some(n) => {
                if data < n.data {
                    Self::insert_recursive(&mut n.left, data)
                } else if data > n.data {
                    Self::insert_recursive(&mut n.right, data)
                } else {
                    false // Duplicate, not inserted
                }
            }
        }
    }

    /// Search for a value in the BST
    pub fn contains(&self, data: &T) -> bool {
        Self::contains_recursive(&self.root, data)
    }

    fn contains_recursive(node: &TreeLink<T>, data: &T) -> bool {
        match node {
            None => false,
            Some(n) => {
                if data < &n.data {
                    Self::contains_recursive(&n.left, data)
                } else if data > &n.data {
                    Self::contains_recursive(&n.right, data)
                } else {
                    true
                }
            }
        }
    }

    /// Get the size of the tree
    pub fn len(&self) -> usize {
        self.size
    }

    /// Check if the tree is empty
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// In-order traversal (sorted order)
    pub fn inorder(&self) -> Vec<&T> {
        let mut result = Vec::new();
        Self::inorder_recursive(&self.root, &mut result);
        result
    }

    fn inorder_recursive<'a>(node: &'a TreeLink<T>, result: &mut Vec<&'a T>) {
        if let Some(n) = node {
            Self::inorder_recursive(&n.left, result);
            result.push(&n.data);
            Self::inorder_recursive(&n.right, result);
        }
    }

    /// Find minimum value
    pub fn min(&self) -> Option<&T> {
        Self::min_recursive(&self.root)
    }

    fn min_recursive(node: &TreeLink<T>) -> Option<&T> {
        node.as_ref().map(|n| {
            if n.left.is_none() {
                &n.data
            } else {
                Self::min_recursive(&n.left).unwrap()
            }
        })
    }

    /// Find maximum value
    pub fn max(&self) -> Option<&T> {
        Self::max_recursive(&self.root)
    }

    fn max_recursive(node: &TreeLink<T>) -> Option<&T> {
        node.as_ref().map(|n| {
            if n.right.is_none() {
                &n.data
            } else {
                Self::max_recursive(&n.right).unwrap()
            }
        })
    }

    /// Get height of the tree
    pub fn height(&self) -> usize {
        Self::height_recursive(&self.root)
    }

    fn height_recursive(node: &TreeLink<T>) -> usize {
        match node {
            None => 0,
            Some(n) => {
                let left_height = Self::height_recursive(&n.left);
                let right_height = Self::height_recursive(&n.right);
                1 + left_height.max(right_height)
            }
        }
    }
}

// ============================================================================
// HASH MAP
// ============================================================================

const INITIAL_CAPACITY: usize = 16;
const LOAD_FACTOR: f64 = 0.75;

#[derive(Clone, Debug)]
struct Bucket<K, V> {
    key: K,
    value: V,
}

/// HashMap implementation using separate chaining
#[derive(Debug)]
pub struct HashMap<K, V> {
    buckets: Vec<Vec<Bucket<K, V>>>,
    size: usize,
    capacity: usize,
}

impl<K: Hash + Eq + Clone, V: Clone> HashMap<K, V> {
    /// Create a new empty HashMap
    pub fn new() -> Self {
        HashMap {
            buckets: vec![Vec::new(); INITIAL_CAPACITY],
            size: 0,
            capacity: INITIAL_CAPACITY,
        }
    }

    /// Insert a key-value pair
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.load_factor() > LOAD_FACTOR {
            self.resize();
        }

        let index = self.get_index(&key);
        let bucket = &mut self.buckets[index];

        // Check if key exists
        for item in bucket.iter_mut() {
            if item.key == key {
                let old_value = item.value.clone();
                item.value = value;
                return Some(old_value);
            }
        }

        // Key doesn't exist, insert new
        bucket.push(Bucket { key, value });
        self.size += 1;
        None
    }

    /// Get a value by key
    pub fn get(&self, key: &K) -> Option<&V> {
        let index = self.get_index(key);
        let bucket = &self.buckets[index];

        for item in bucket {
            if &item.key == key {
                return Some(&item.value);
            }
        }

        None
    }

    /// Remove a key-value pair
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let index = self.get_index(key);
        let bucket = &mut self.buckets[index];

        for (i, item) in bucket.iter().enumerate() {
            if &item.key == key {
                let removed = bucket.remove(i);
                self.size -= 1;
                return Some(removed.value);
            }
        }

        None
    }

    /// Check if key exists
    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    /// Get the number of key-value pairs
    pub fn len(&self) -> usize {
        self.size
    }

    /// Check if the map is empty
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Get all keys
    pub fn keys(&self) -> Vec<&K> {
        let mut keys = Vec::new();
        for bucket in &self.buckets {
            for item in bucket {
                keys.push(&item.key);
            }
        }
        keys
    }

    /// Get all values
    pub fn values(&self) -> Vec<&V> {
        let mut values = Vec::new();
        for bucket in &self.buckets {
            for item in bucket {
                values.push(&item.value);
            }
        }
        values
    }

    fn get_index(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.capacity
    }

    fn load_factor(&self) -> f64 {
        self.size as f64 / self.capacity as f64
    }

    fn resize(&mut self) {
        let new_capacity = self.capacity * 2;
        let mut new_buckets = vec![Vec::new(); new_capacity];

        // Rehash all items
        for bucket in &self.buckets {
            for item in bucket {
                let mut hasher = DefaultHasher::new();
                item.key.hash(&mut hasher);
                let new_index = (hasher.finish() as usize) % new_capacity;
                new_buckets[new_index].push(item.clone());
            }
        }

        self.buckets = new_buckets;
        self.capacity = new_capacity;
    }
}

// ============================================================================
// DEMO FUNCTIONS
// ============================================================================

fn demo_linked_list() {
    println!("=== Linked List Demo ===\n");

    let mut list = LinkedList::new();

    // Insert elements
    println!("Pushing: 1, 2, 3, 4, 5");
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    list.push_front(4);
    list.push_front(5);

    println!("List size: {}", list.len());
    println!("Front element: {:?}", list.peek());

    // Iterate
    print!("List contents: ");
    for item in list.iter() {
        print!("{} ", item);
    }
    println!("\n");

    // Pop elements
    println!("Popping elements:");
    while let Some(data) = list.pop_front() {
        println!("  Popped: {}", data);
    }
    println!("List is empty: {}", list.is_empty());
}

fn demo_bst() {
    println!("\n=== Binary Search Tree Demo ===\n");

    let mut bst = BinarySearchTree::new();

    // Insert elements
    println!("Inserting: 50, 30, 70, 20, 40, 60, 80");
    bst.insert(50);
    bst.insert(30);
    bst.insert(70);
    bst.insert(20);
    bst.insert(40);
    bst.insert(60);
    bst.insert(80);

    println!("Tree size: {}", bst.len());
    println!("Tree height: {}", bst.height());

    // Search
    println!("\nSearching:");
    println!("  Contains 40: {}", bst.contains(&40));
    println!("  Contains 100: {}", bst.contains(&100));

    // Min/Max
    println!("\nMin: {:?}", bst.min());
    println!("Max: {:?}", bst.max());

    // In-order traversal (sorted)
    print!("\nIn-order traversal (sorted): ");
    for val in bst.inorder() {
        print!("{} ", val);
    }
    println!();
}

fn demo_hashmap() {
    println!("\n=== HashMap Demo ===\n");

    let mut map = HashMap::new();

    // Insert key-value pairs
    println!("Inserting key-value pairs:");
    map.insert("name", "Alice");
    map.insert("age", "30");
    map.insert("city", "Seattle");
    map.insert("country", "USA");

    println!("Map size: {}", map.len());

    // Get values
    println!("\nGetting values:");
    println!("  name: {:?}", map.get(&"name"));
    println!("  age: {:?}", map.get(&"age"));
    println!("  email: {:?}", map.get(&"email"));

    // Contains key
    println!("\nChecking keys:");
    println!("  Contains 'city': {}", map.contains_key(&"city"));
    println!("  Contains 'email': {}", map.contains_key(&"email"));

    // All keys and values
    println!("\nAll keys: {:?}", map.keys());
    println!("All values: {:?}", map.values());

    // Update value
    println!("\nUpdating 'age' to 31:");
    map.insert("age", "31");
    println!("  age: {:?}", map.get(&"age"));

    // Remove
    println!("\nRemoving 'city':");
    let removed = map.remove(&"city");
    println!("  Removed value: {:?}", removed);
    println!("  Map size: {}", map.len());

    // Test with numeric keys
    println!("\n--- Numeric HashMap ---");
    let mut num_map = HashMap::new();
    for i in 0..10 {
        num_map.insert(i, i * i);
    }
    
    println!("Map with numbers 0-9 and their squares:");
    for i in 0..10 {
        println!("  {}: {:?}", i, num_map.get(&i));
    }
}

fn main() {
    demo_linked_list();
    demo_bst();
    demo_hashmap();
    
    println!("\n=== All Demos Complete ===");
}
