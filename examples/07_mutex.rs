// Example 7: Mutex - Mutual Exclusion
// Run with: cargo run --example 07_mutex

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("=== Mutex Example ===\n");

    // Simple mutex
    println!("1. Basic Mutex:");
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
        println!("  Changed value to: {}", *num);
        // Lock automatically released here when 'num' goes out of scope
    }

    println!("  Value is now: {:?}\n", m);

    // Mutex with Arc for thread sharing
    println!("2. Mutex + Arc across threads:");
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..5 {
        let counter_clone = counter.clone();
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
            println!("  Thread {} incremented counter to {}", i, *num);
            // Lock released here
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("\nFinal counter value: {}", *counter.lock().unwrap());

    // What happens without Mutex?
    println!("\n3. Why we need Mutex:");
    println!("  Without Mutex: Multiple threads could modify at same time");
    println!("  Result: Data race! Undefined behavior!");
    println!("  With Mutex: Only one thread can access at a time");
    println!("  Result: Safe! ✓");

    println!("\n💡 Key concepts:");
    println!("  - Mutex<T> = Mutual exclusion lock");
    println!("  - .lock() acquires lock (blocks if locked)");
    println!("  - Returns MutexGuard (smart pointer)");
    println!("  - Lock auto-released when guard is dropped");
    println!("  - Arc<Mutex<T>> = share mutable data across threads");
}
