// Example 11: Threading
// Run with: cargo run --example 11_threading

use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Threading Example ===\n");

    println!("1. Basic thread:");
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("  Thread: count {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    for i in 1..3 {
        println!("Main: count {}", i);
        thread::sleep(Duration::from_millis(150));
    }

    handle.join().unwrap();  // Wait for thread to finish
    println!();

    println!("2. Thread with move:");
    let data = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("  Thread owns: {:?}", data);
        // data is moved into this closure
    });

    handle.join().unwrap();
    // println!("{:?}", data);  // ❌ ERROR! data was moved
    println!();

    println!("3. Multiple threads:");
    let mut handles = vec![];

    for i in 0..5 {
        let handle = thread::spawn(move || {
            println!("  Thread {} started", i);
            thread::sleep(Duration::from_millis(100));
            println!("  Thread {} finished", i);
            i * 2  // Return value
        });
        handles.push(handle);
    }

    for (i, handle) in handles.into_iter().enumerate() {
        let result = handle.join().unwrap();
        println!("Thread {} returned: {}", i, result);
    }

    println!("\n💡 Key concepts:");
    println!("  - thread::spawn(closure) creates thread");
    println!("  - .join() waits for thread to finish");
    println!("  - move closure takes ownership of variables");
    println!("  - JoinHandle<T> can return value from thread");
}
