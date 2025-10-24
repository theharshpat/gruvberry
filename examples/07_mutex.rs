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

    println!("\n   📝 Important: When do threads actually start?\n");
    println!("   ✅ thread::spawn() IMMEDIATELY starts the thread!");
    println!("      - It does NOT wait for join()");
    println!("      - The thread runs in the background right away");
    println!("      - 'handle' is just a reference to the running thread\n");

    for i in 0..5 {
        let counter_clone = counter.clone();
        println!("   🚀 Spawning thread {}...", i);
        let handle = thread::spawn(move || {
            // This code runs IMMEDIATELY in a new OS thread!
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
            println!("     Thread {} incremented counter to {}", i, *num);
            // Lock released here when 'num' (MutexGuard) goes out of scope
        });
        handles.push(handle);
    }

    println!("\n   All 5 threads are now running in parallel!\n");
    println!("   🕐 Now calling join() on each handle...\n");

    for (i, handle) in handles.into_iter().enumerate() {
        println!("   Waiting for thread {} to finish...", i);
        handle.join().unwrap();
        // join() blocks until THIS SPECIFIC thread completes
        // It does NOT wait for all threads - just the one it's called on
    }

    println!("\n   ✅ All threads finished!\n");
    println!("Final counter value: {}", *counter.lock().unwrap());

    // What happens without Mutex?
    println!("\n3. Why we need Mutex:");
    println!("  Without Mutex: Multiple threads could modify at same time");
    println!("  Result: Data race! Undefined behavior!");
    println!("  With Mutex: Only one thread can access at a time");
    println!("  Result: Safe! ✓");

    println!("\n{}\n", "=".repeat(60));
    println!("💡 Key concepts:\n");

    println!("  Mutex<T> = Mutual exclusion lock");
    println!("    - .lock() acquires lock (blocks if already locked)");
    println!("    - Returns MutexGuard<T> (smart pointer)");
    println!("    - Lock auto-released when guard is dropped (RAII)");
    println!("    - Arc<Mutex<T>> = share mutable data across threads\n");

    println!("  Thread spawning behavior:");
    println!("    - thread::spawn() = IMMEDIATELY creates new OS thread");
    println!("    - Threads start running RIGHT AWAY (parallel execution)");
    println!("    - Returns JoinHandle<T> (reference to running thread)");
    println!("    - JoinHandle is NOT a definition - thread is ALREADY running!\n");

    println!("  handle.join() behavior:");
    println!("    - Blocks until THAT SPECIFIC thread finishes");
    println!("    - Does NOT wait for all threads - only the one it's called on");
    println!("    - If thread already finished, join() returns immediately");
    println!("    - Calling join() on all handles = wait for ALL to complete\n");

    println!("  Execution timeline:");
    println!("    1. Loop iteration 1: spawn thread 0 → starts immediately");
    println!("    2. Loop iteration 2: spawn thread 1 → starts immediately");
    println!("    3. ... (threads 0, 1, 2, 3, 4 all running in parallel)");
    println!("    4. First join() call: wait for thread 0 to finish");
    println!("    5. Second join() call: wait for thread 1 to finish");
    println!("    6. ... (if thread already done, join returns right away)");
    println!("    7. All joins complete → all threads finished → continue");
}
