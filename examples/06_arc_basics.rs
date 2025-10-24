// Example 6: Arc - Atomic Reference Counting
// Run with: cargo run --example 06_arc_basics

use std::sync::Arc;
use std::thread;

fn main() {
    println!("=== Arc (Atomic Reference Counting) Example ===\n");

    println!("❓ WHY DO WE NEED Arc?\n");
    println!("Problem: Rust's ownership rules say each value has ONE owner.");
    println!("But what if multiple threads need to READ the same data?\n");

    println!("❌ This WON'T work:");
    println!("   let data = vec![1, 2, 3];");
    println!("   thread::spawn(move || {{ /* uses data */ }});");
    println!("   thread::spawn(move || {{ /* ERROR! data already moved */ }});\n");

    println!("✅ Solution: Arc lets MULTIPLE owners share the SAME data");
    println!("   Arc keeps a reference count - data is freed when count hits 0\n");
    println!("{}\n", "=".repeat(60));

    // Create data wrapped in Arc
    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    println!("1️⃣  Created Arc with data: {:?}", data);
    println!("   Reference count: {}", Arc::strong_count(&data));
    println!("   (One owner: 'data' variable)\n");

    // Clone Arc (increments ref count, doesn't copy data!)
    let data2 = data.clone();
    let data3 = data.clone();

    println!("2️⃣  After calling .clone() twice:");
    println!("   Reference count: {}", Arc::strong_count(&data));
    println!("   IMPORTANT: .clone() does NOT copy the vec![1,2,3,4,5]!");
    println!("              It only increments a counter (very cheap!)");
    println!("              All 3 variables point to THE SAME memory\n");

    // Share across threads
    println!("3️⃣  Sharing across threads:\n");
    println!("   🔍 What is 'move ||' (move closure)?");
    println!("      - Closures are anonymous functions: || {{ code }}");
    println!("      - 'move' keyword transfers OWNERSHIP into the closure");
    println!("      - thread::spawn REQUIRES 'move' because the thread");
    println!("        might outlive the current scope\n");

    let data_thread1 = data.clone();  // Ref count: 4
    println!("   Cloned data for thread 1 (ref count now: 4)");

    let handle1 = thread::spawn(move || {
        // 'move' transfers ownership of data_thread1 INTO this closure
        println!("     Thread 1 sees: {:?}", data_thread1);
        println!("     Thread 1 ref count: {}", Arc::strong_count(&data_thread1));

        // When thread 1 ends, data_thread1 is DROPPED here
        // This decrements ref count by 1
    });  // ← Thread 1's closure ends, data_thread1 dropped when thread finishes

    let data_thread2 = data.clone();  // Ref count: 5
    println!("   Cloned data for thread 2 (ref count now: 5)");

    let handle2 = thread::spawn(move || {
        println!("     Thread 2 sees: {:?}", data_thread2);
        println!("     Thread 2 ref count: {}", Arc::strong_count(&data_thread2));

        // When thread 2 ends, data_thread2 is DROPPED here
    });

    println!("\n   🕐 Waiting for threads to finish...\n");

    handle1.join().unwrap();  // Thread 1 ends → data_thread1 dropped → ref count -1
    handle2.join().unwrap();  // Thread 2 ends → data_thread2 dropped → ref count -1

    println!("4️⃣  Back in main:");
    println!("   Reference count: {}", Arc::strong_count(&data));
    println!("   (Threads finished, their Arcs were auto-dropped)\n");

    // Drop clones
    drop(data2);
    println!("5️⃣  Manually dropped data2");
    println!("   Reference count: {}", Arc::strong_count(&data));

    drop(data3);
    println!("   Manually dropped data3");
    println!("   Reference count: {}", Arc::strong_count(&data));
    println!("   (Only 'data' remains)\n");

    println!("{}\n", "=".repeat(60));
    println!("💡 Key concepts:\n");
    println!("  Arc = Atomic Reference Counted smart pointer");
    println!("    - Solves: How to share data across multiple threads?");
    println!("    - .clone() increments counter (cheap! Just +1 to a number)");
    println!("    - Does NOT copy the actual data");
    println!("    - Data freed when LAST Arc is dropped (count → 0)\n");

    println!("  'move' closure:");
    println!("    - Transfers ownership of captured variables INTO closure");
    println!("    - Required by thread::spawn (thread might outlive scope)");
    println!("    - Syntax: move || {{ code }}\n");

    println!("  When is Arc dropped (count decremented)?");
    println!("    - Automatically when variable goes out of scope");
    println!("    - Manually with drop(variable)");
    println!("    - For threads: when the thread function ends\n");

    println!("  Arc is IMMUTABLE by default!");
    println!("    - For mutation: use Arc<Mutex<T>> (see example 07)");
    println!("    - Thread-safe (unlike Rc which is single-threaded)");
}
