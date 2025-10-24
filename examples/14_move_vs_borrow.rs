// Example 14: Move vs Borrow - Side by Side Comparison
// Run with: cargo run --example 14_move_vs_borrow

use std::thread;
use std::sync::Arc;

fn main() {
    println!("=== Move vs Borrow in Closures ===\n");

    println!("{}\n", "=".repeat(60));

    // ========== REGULAR CLOSURES (NOT THREADS) ==========
    println!("Part 1: Regular closures (not threads)\n");

    // Example 1: Closure WITHOUT move (borrowing)
    println!("1️⃣  Closure WITHOUT 'move' (borrows):\n");
    {
        let x = 5;
        let y = String::from("hello");

        let borrow_closure = || {
            // Borrows x and y from outer scope
            println!("   Borrowed x: {}", x);
            println!("   Borrowed y: {}", y);
        };

        borrow_closure();

        // ✅ Still accessible because closure only borrowed
        println!("   After closure: x={}, y={}\n", x, y);
    }

    // Example 2: Closure WITH move (takes ownership)
    println!("2️⃣  Closure WITH 'move' (owns):\n");
    {
        let x = 5;
        let y = String::from("world");

        let move_closure = move || {
            // Takes ownership of x and y
            println!("   Owned x: {}", x);
            println!("   Owned y: {}", y);
        };

        move_closure();

        // ✅ x still accessible (Copy type - was copied)
        println!("   After closure: x={}", x);

        // ❌ y NOT accessible (Move type - was moved)
        // println!("{}", y);  // ERROR: value borrowed after move
        println!("   y is no longer accessible (moved into closure)\n");
    }

    println!("{}\n", "=".repeat(60));

    // ========== THREADS (MUST USE MOVE) ==========
    println!("Part 2: Threads (MUST use 'move')\n");

    // Example 3: Thread WITHOUT move (DOESN'T COMPILE)
    println!("3️⃣  Thread WITHOUT 'move':\n");
    println!("   let data = vec![1, 2, 3];");
    println!("   thread::spawn(|| {{");
    println!("       println!(\"{{:?}}\", data);");
    println!("   }});\n");
    println!("   ❌ COMPILER ERROR:");
    println!("   closure may outlive the current function, but it borrows `data`");
    println!("   which is owned by the current function\n");

    // Example 4: Thread WITH move (WORKS!)
    println!("4️⃣  Thread WITH 'move' (correct):\n");
    {
        let data = vec![1, 2, 3];
        println!("   Before thread: data={:?}", data);

        let handle = thread::spawn(move || {
            println!("   Inside thread: data={:?}", data);
            // Thread owns data now
        });

        // println!("{:?}", data);  // ❌ ERROR: moved
        println!("   Main thread: data is no longer accessible");

        handle.join().unwrap();
        println!("   ✅ Thread finished, data was safely dropped\n");
    }

    println!("{}\n", "=".repeat(60));

    // ========== THE LIFETIME PROBLEM ==========
    println!("Part 3: Why thread::spawn REQUIRES 'move'\n");

    println!("5️⃣  The lifetime problem:\n");
    println!("   Imagine this scenario:\n");
    println!("   fn spawn_thread() {{");
    println!("       let msg = String::from(\"Hello\");");
    println!("       thread::spawn(|| {{");
    println!("           sleep(2 seconds);");
    println!("           println!(\"{{}} from thread\", msg);  // Borrow msg");
    println!("       }});");
    println!("   }}  // ← Function ends, 'msg' is DESTROYED\n");
    println!("   Problem:");
    println!("   - Function returns immediately");
    println!("   - 'msg' is dropped (freed from memory)");
    println!("   - Thread wakes up after 2 seconds");
    println!("   - Tries to use 'msg' → DANGLING POINTER! 💥\n");

    println!("   With 'move':");
    println!("   fn spawn_thread() {{");
    println!("       let msg = String::from(\"Hello\");");
    println!("       thread::spawn(move || {{");
    println!("           sleep(2 seconds);");
    println!("           println!(\"{{}} from thread\", msg);  // OWNS msg");
    println!("       }});");
    println!("   }}  // ← 'msg' moved to thread, NOT destroyed!\n");
    println!("   Solution:");
    println!("   - 'msg' is MOVED into the thread");
    println!("   - Thread owns it now");
    println!("   - Will be dropped when THREAD finishes");
    println!("   - No dangling reference! ✅\n");

    println!("{}\n", "=".repeat(60));

    // ========== SHARING WITH Arc ==========
    println!("Part 4: Sharing data with Arc + move\n");

    println!("6️⃣  Multiple threads, same data (Arc):\n");

    let shared_data = Arc::new(vec![10, 20, 30]);
    println!("   Created Arc: {:?}", shared_data);
    println!("   Ref count: {}\n", Arc::strong_count(&shared_data));

    // Clone the Arc (cheap! just increments counter)
    let data_for_thread1 = shared_data.clone();
    let data_for_thread2 = shared_data.clone();
    let data_for_thread3 = shared_data.clone();

    println!("   After 3 clones, ref count: {}\n", Arc::strong_count(&shared_data));

    let h1 = thread::spawn(move || {
        // 'move' transfers ownership of data_for_thread1 Arc
        println!("   Thread 1: {:?}", data_for_thread1);
    });

    let h2 = thread::spawn(move || {
        println!("   Thread 2: {:?}", data_for_thread2);
    });

    let h3 = thread::spawn(move || {
        println!("   Thread 3: {:?}", data_for_thread3);
    });

    println!("   Main thread: {:?}", shared_data);
    println!("   Main thread ref count: {}\n", Arc::strong_count(&shared_data));

    h1.join().unwrap();
    h2.join().unwrap();
    h3.join().unwrap();

    println!("   After threads finish, ref count: {}", Arc::strong_count(&shared_data));
    println!("   ✅ Each thread had its own Arc, data shared safely\n");

    println!("{}\n", "=".repeat(60));

    // ========== SUMMARY ==========
    println!("💡 VISUAL SUMMARY\n");

    println!("┌─────────────────────────────────────────────────────────┐");
    println!("│ Regular Closure (optional 'move')                      │");
    println!("├─────────────────────────────────────────────────────────┤");
    println!("│ WITHOUT move:                                           │");
    println!("│   let x = 5;                                            │");
    println!("│   let f = || x + 1;    // Borrows x                     │");
    println!("│   println!(\"{{}}\", x);    // ✅ OK: x still accessible    │");
    println!("├─────────────────────────────────────────────────────────┤");
    println!("│ WITH move:                                              │");
    println!("│   let x = 5;                                            │");
    println!("│   let f = move || x + 1;  // Copies/moves x             │");
    println!("│   println!(\"{{}}\", x);    // ✅ OK: x is Copy type       │");
    println!("└─────────────────────────────────────────────────────────┘\n");

    println!("┌─────────────────────────────────────────────────────────┐");
    println!("│ Thread Closure (MUST use 'move')                       │");
    println!("├─────────────────────────────────────────────────────────┤");
    println!("│ WITHOUT move:                                           │");
    println!("│   let x = 5;                                            │");
    println!("│   thread::spawn(|| x + 1);  // ❌ COMPILER ERROR        │");
    println!("│   ERROR: closure may outlive current function           │");
    println!("├─────────────────────────────────────────────────────────┤");
    println!("│ WITH move:                                              │");
    println!("│   let x = 5;                                            │");
    println!("│   thread::spawn(move || x + 1);  // ✅ OK               │");
    println!("│   // x moved/copied to thread, thread owns it           │");
    println!("└─────────────────────────────────────────────────────────┘\n");

    println!("KEY TAKEAWAY:");
    println!("  'move' = \"Give this closure OWNERSHIP of captured variables\"");
    println!("  Without 'move' = \"Let this closure BORROW captured variables\"");
    println!("  thread::spawn = MUST use 'move' (thread might outlive scope)");
    println!("  Regular closures = 'move' is optional (closure in same scope)");
}
