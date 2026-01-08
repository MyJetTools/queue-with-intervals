# QueueWithIntervals

A Rust library for efficiently managing queues of integer values using interval-based storage. This data structure stores consecutive integer values as ranges, making it memory-efficient for large sequences of sequential numbers.

## Features

- **Efficient Storage**: Stores consecutive integers as intervals `[from, to]` instead of individual values
- **Generic Support**: Works with all standard Rust integer types (i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, isize, usize)
- **Range Operations**: Enqueue and remove entire ranges of values at once
- **Iterator Support**: Iterate over all values in the queue
- **Merge Operations**: Combine queues and automatically merge adjacent intervals

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
queue-with-intervals = "0.1.0"
```

## Quick Start

```rust
use queue_with_intervals::{QueueWithIntervals, QueueIndexRange};

fn main() {
    // Create a new queue (defaults to i64)
    let mut queue = QueueWithIntervals::new();
    
    // Enqueue individual values
    queue.enqueue(5);
    queue.enqueue(6);
    queue.enqueue(7);
    
    // Dequeue values
    assert_eq!(queue.dequeue(), Some(5));
    assert_eq!(queue.dequeue(), Some(6));
    
    // Peek at the next value without removing it
    assert_eq!(queue.peek(), Some(7));
}
```

## Basic Operations

### Creating a Queue

```rust
// Default queue (i64)
let mut queue = QueueWithIntervals::new();

// Specify a different integer type
let mut queue_u32: QueueWithIntervals<u32> = QueueWithIntervals::new();
let mut queue_i16: QueueWithIntervals<i16> = QueueWithIntervals::new();

// Create from a single interval
let queue = QueueWithIntervals::from_single_interval(10, 20);

// Restore from existing intervals
let intervals = vec![
    QueueIndexRange::restore(10, 20),
    QueueIndexRange::restore(30, 40),
];
let queue = QueueWithIntervals::restore(intervals);
```

### Enqueue Operations

```rust
let mut queue = QueueWithIntervals::new();

// Enqueue a single value
queue.enqueue(5);

// Enqueue multiple values (they will be merged into intervals)
queue.enqueue(6);
queue.enqueue(7);
queue.enqueue(10);
queue.enqueue(11);

// Enqueue an entire range at once
queue.enqueue_range(QueueIndexRange::restore(20, 25));
```

### Dequeue Operations

```rust
let mut queue = QueueWithIntervals::new();
queue.enqueue(5);
queue.enqueue(6);
queue.enqueue(7);

// Dequeue values in order
while let Some(value) = queue.dequeue() {
    println!("Dequeued: {}", value);
}

// Peek without removing
if let Some(next) = queue.peek() {
    println!("Next value: {}", next);
}
```

### Remove Operations

```rust
let mut queue = QueueWithIntervals::new();
queue.enqueue_range(QueueIndexRange::restore(10, 20));

// Remove a single value
match queue.remove(15) {
    Ok(()) => println!("Removed successfully"),
    Err(e) => println!("Error: {:?}", e),
}

// Remove an entire range
queue.remove_range(&QueueIndexRange::restore(12, 18));
```

### Query Operations

```rust
let mut queue = QueueWithIntervals::new();
queue.enqueue_range(QueueIndexRange::restore(10, 20));
queue.enqueue_range(QueueIndexRange::restore(30, 40));

// Check if queue is empty
if queue.is_empty() {
    println!("Queue is empty");
}

// Check if a specific value exists
if queue.has_message(15) {
    println!("Value 15 is in the queue");
}

// Get min and max values
if let Some(min) = queue.get_min_id() {
    println!("Min value: {}", min);
}
if let Some(max) = queue.get_max_id() {
    println!("Max value: {}", max);
}

// Get queue size
println!("Queue contains {} values", queue.len());
println!("Queue size (alternative): {}", queue.queue_size());
```

### Iterator Support

```rust
let mut queue = QueueWithIntervals::new();
queue.enqueue_range(QueueIndexRange::restore(10, 12));
queue.enqueue_range(QueueIndexRange::restore(20, 22));

// Iterate over all values
for value in &queue {
    println!("Value: {}", value);
}

// Collect into a vector
let values: Vec<i64> = queue.iter().collect();
assert_eq!(values, vec![10, 11, 12, 20, 21, 22]);

// Use with iterator methods
let sum: i64 = queue.iter().sum();
let filtered: Vec<i64> = queue.iter().filter(|&x| x > 15).collect();
```

### Merge Operations

```rust
let mut queue1 = QueueWithIntervals::new();
queue1.enqueue_range(QueueIndexRange::restore(10, 20));

let mut queue2 = QueueWithIntervals::new();
queue2.enqueue_range(QueueIndexRange::restore(15, 25));

// Merge queue2 into queue1 (adjacent intervals will be merged)
queue1.merge(queue2);
// queue1 now contains [10, 25]
```

### Interval Management

```rust
let mut queue = QueueWithIntervals::new();
queue.enqueue_range(QueueIndexRange::restore(10, 20));
queue.enqueue_range(QueueIndexRange::restore(30, 40));

// Get all intervals
let intervals = queue.get_intervals();
for interval in intervals {
    println!("Interval: [{}, {}]", interval.from_id, interval.to_id);
}

// Get a specific interval
if let Some(interval) = queue.get_interval(0) {
    println!("First interval: [{}, {}]", interval.from_id, interval.to_id);
}

// Get a snapshot (clone of intervals)
let snapshot = queue.get_snapshot();

// Reset queue with new intervals
queue.reset(vec![
    QueueIndexRange::restore(50, 60),
    QueueIndexRange::restore(70, 80),
]);

// Clean the queue (removes all values but preserves structure)
queue.clean();
```

## Supported Types

The `QueueValue` trait is implemented for all standard Rust integer types:

- **Signed integers**: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
- **Unsigned integers**: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`

**Note**: Float types (`f32`, `f64`) are **not** supported.

## Error Handling

```rust
use queue_with_intervals::{QueueWithIntervals, QueueWithIntervalsError};

let mut queue = QueueWithIntervals::new();

// Remove from empty queue
match queue.remove(5) {
    Err(QueueWithIntervalsError::QueueIsEmpty) => {
        println!("Cannot remove from empty queue");
    }
    Err(QueueWithIntervalsError::MessagesNotFound) => {
        println!("Value not found in queue");
    }
    Ok(()) => {
        println!("Removed successfully");
    }
    _ => {}
}
```

## Use Cases

- **Message Queue Systems**: Track processed message IDs efficiently
- **Gap Detection**: Identify missing values in sequences
- **Range Queries**: Quickly check if values exist in ranges
- **Memory-Efficient Storage**: Store large sequences of consecutive integers
- **Event Processing**: Track processed event IDs or timestamps

## Example: Message Queue Tracking

```rust
use queue_with_intervals::{QueueWithIntervals, QueueIndexRange};

fn main() {
    let mut processed_messages = QueueWithIntervals::<u64>::new();
    
    // Mark messages 100-200 as processed
    processed_messages.enqueue_range(QueueIndexRange::restore(100, 200));
    
    // Mark individual messages
    processed_messages.enqueue(250);
    processed_messages.enqueue(251);
    
    // Check if a message was processed
    if processed_messages.has_message(150) {
        println!("Message 150 was already processed");
    }
    
    // Find gaps (unprocessed messages)
    let all_messages = QueueIndexRange::restore(100, 300);
    // ... logic to find gaps ...
    
    // Get statistics
    println!("Processed {} messages", processed_messages.len());
    if let Some(min) = processed_messages.get_min_id() {
        println!("First processed: {}", min);
    }
}
```

## Performance Considerations

- **Memory Efficiency**: Consecutive values are stored as single intervals, significantly reducing memory usage for sequential data
- **Time Complexity**:
  - `enqueue()` / `dequeue()`: O(log n) where n is the number of intervals
  - `has_message()`: O(log n)
  - `remove()`: O(log n)
  - `enqueue_range()` / `remove_range()`: O(n) where n is the number of intervals
- **Best Performance**: When values are mostly consecutive or in ranges
- **Worst Performance**: When values are completely random with no consecutive sequences

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

Copyright (c) 2026 My Jet Tools
