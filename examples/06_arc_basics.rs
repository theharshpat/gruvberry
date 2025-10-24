// Example 6: Arc - Atomic Reference Counting
// Run with: cargo run --example 06_arc_basics

use std::sync::Arc;
use std::thread;

fn main() {
    println!("=== Arc (Atomic Reference Counting) Example ===\n");

    // Create data wrapped in Arc
    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    println!("Created Arc with data: {:?}", data);
    println!("Reference count: {}\n", Arc::strong_count(&data));

    // Clone Arc (increments ref count, doesn't copy data!)
    let data2 = data.clone();
    let data3 = data.clone();

    println!("After 2 clones:");
    println!("Reference count: {}\n", Arc::strong_count(&data));

    // Share across threads
    println!("Sharing across threads:");
    let data_thread1 = data.clone();
    let handle1 = thread::spawn(move || {
        println!("  Thread 1 sees: {:?}", data_thread1);
        println!("  Thread 1 ref count: {}", Arc::strong_count(&data_thread1));
    });

    let data_thread2 = data.clone();
    let handle2 = thread::spawn(move || {
        println!("  Thread 2 sees: {:?}", data_thread2);
        println!("  Thread 2 ref count: {}", Arc::strong_count(&data_thread2));
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("\nBack in main:");
    println!("Reference count: {}", Arc::strong_count(&data));

    // Drop clones
    drop(data2);
    drop(data3);
    println!("After dropping 2 clones: {}", Arc::strong_count(&data));

    println!("\n💡 Key concepts:");
    println!("  - Arc = Atomic Reference Counted smart pointer");
    println!("  - .clone() increments counter (cheap!)");
    println!("  - Data freed when last Arc is dropped");
    println!("  - Thread-safe (unlike Rc)");
    println!("  - Immutable by default (use Arc<Mutex<T>> for mutation)");
}
