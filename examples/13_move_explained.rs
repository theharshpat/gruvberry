// Example 13: Why 'move' is needed in thread::spawn
// Run with: cargo run --example 13_move_explained

use std::thread;

fn main() {
    println!("=== Why 'move' is Required in thread::spawn ===\n");

    println!("❓ THE PROBLEM\n");
    println!("When you spawn a thread, it might run for a long time.");
    println!("The main thread could finish and clean up its variables.");
    println!("What if the spawned thread tries to use those variables?");
    println!("→ DANGLING REFERENCE! (use-after-free bug)\n");
    println!("{}\n", "=".repeat(60));

    // Example 1: Why we can't just borrow
    println!("1️⃣  What happens WITHOUT 'move'?\n");
    println!("Let's say we try this (won't compile):\n");
    println!("   fn broken_example() {{");
    println!("       let data = vec![1, 2, 3];");
    println!("       thread::spawn(|| {{");
    println!("           println!(\"{{:?}}\", data);  // ❌ Borrowing 'data'");
    println!("       }});");
    println!("   }}  // ← 'data' is destroyed here!\n");

    println!("   ❌ COMPILER ERROR:");
    println!("      'data' does not live long enough");
    println!("      closure may outlive the current function\n");

    println!("   Why the error?");
    println!("   - Thread might still be running after function ends");
    println!("   - 'data' would be destroyed (dropped)");
    println!("   - Thread would access freed memory → CRASH!\n");

    println!("{}\n", "=".repeat(60));

    // Example 2: Simple case with move
    println!("2️⃣  Solution: Use 'move' to transfer ownership\n");

    let data = vec![1, 2, 3];
    println!("   Before spawn: data = {:?}", data);
    println!("   Main thread OWNS data (stored on stack)\n");

    let handle = thread::spawn(move || {
        // 'move' transfers ownership of 'data' INTO this closure
        println!("   Inside thread: data = {:?}", data);
        println!("   Thread now OWNS data (moved to thread's stack)");
    });

    // println!("{:?}", data);  // ❌ ERROR! 'data' was moved
    println!("   Main thread can NO LONGER use 'data'");
    println!("   (Ownership transferred to thread)\n");

    handle.join().unwrap();
    println!("   ✅ Thread finished and 'data' was dropped safely\n");

    println!("{}\n", "=".repeat(60));

    // Example 3: What 'move' actually does
    println!("3️⃣  What does 'move' actually do?\n");

    let x = 5;
    let s = String::from("hello");

    println!("   Before closure:");
    println!("   x = {} (Copy type - stored on stack)", x);
    println!("   s = '{}' (Move type - pointer to heap)\n", s);

    let closure = move || {
        println!("   Inside closure:");
        println!("   x = {} (COPIED into closure)", x);
        println!("   s = '{}' (MOVED into closure)", s);
    };

    println!("   After creating closure:");
    println!("   x = {} (still accessible - was copied)", x);
    // println!("{}", s);  // ❌ ERROR! s was moved

    closure();
    println!("\n   ✅ Closure owns 's' now\n");

    println!("{}\n", "=".repeat(60));

    // Example 4: Multiple threads with shared data (Arc)
    println!("4️⃣  Sharing data across threads (Arc + move)\n");

    use std::sync::Arc;

    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    println!("   Created Arc<Vec> with ref count: {}", Arc::strong_count(&data));

    let data1 = data.clone();  // Clone Arc (cheap! just +1 ref count)
    let handle1 = thread::spawn(move || {
        // 'move' transfers ownership of data1 (the Arc clone)
        println!("   Thread 1: {:?}", data1);
    });

    let data2 = data.clone();
    let handle2 = thread::spawn(move || {
        println!("   Thread 2: {:?}", data2);
    });

    println!("   Main thread still has: {:?}", data);
    println!("   Ref count: {}\n", Arc::strong_count(&data));

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("   ✅ All threads finished\n");

    println!("{}\n", "=".repeat(60));

    // Example 5: Lifetime demonstration
    println!("5️⃣  Lifetime problem (why 'move' is mandatory)\n");

    println!("   Scenario: What if thread outlives the variable?\n");
    println!("   {{");
    println!("       let msg = String::from(\"Hello\");");
    println!("       thread::spawn(|| {{");
    println!("           // Without 'move': borrows 'msg'");
    println!("           println!(\"{{}} from thread\", msg);");
    println!("       }});");
    println!("   }}  // ← 'msg' dropped here!");
    println!("   // Thread might still be running → accessing freed memory!\n");

    println!("   With 'move':");
    {
        let msg = String::from("Hello");
        let handle = thread::spawn(move || {
            // 'move' transfers ownership
            std::thread::sleep(std::time::Duration::from_millis(50));
            println!("   '{}' from thread (safely owned)", msg);
        });
        // 'msg' no longer accessible here
        handle.join().unwrap();
    }  // Thread guaranteed finished, msg safely dropped

    println!("   ✅ Safe! Thread owns 'msg', no dangling reference\n");

    println!("{}\n", "=".repeat(60));

    // Summary
    println!("💡 SUMMARY\n");

    println!("  Why 'move' is needed:");
    println!("    - Threads can outlive the scope that created them");
    println!("    - Borrowing would create dangling references");
    println!("    - 'move' transfers OWNERSHIP to the thread");
    println!("    - Thread becomes the new owner → safe!\n");

    println!("  What 'move' does:");
    println!("    - For Copy types (i32, f64, bool): copies the value");
    println!("    - For Move types (String, Vec): transfers ownership");
    println!("    - Captures ALL used variables from outer scope\n");

    println!("  Without 'move' (borrowing):");
    println!("    ❌ let x = 5;");
    println!("    ❌ thread::spawn(|| println!(\"{{}}\" x));");
    println!("    ERROR: closure may outlive the current function\n");

    println!("  With 'move' (ownership transfer):");
    println!("    ✅ let x = 5;");
    println!("    ✅ thread::spawn(move || println!(\"{{}}\" x));");
    println!("    OK: thread owns x, guaranteed safe\n");

    println!("  How to share data:");
    println!("    - Use Arc::clone() to create multiple owners");
    println!("    - Each thread gets its own Arc (moved into thread)");
    println!("    - Arc tracks ref count, drops when count = 0");
    println!("    - For mutation: Arc<Mutex<T>>");
}
