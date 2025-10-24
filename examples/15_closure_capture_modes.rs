// Example 15: Closure Capture Modes - Borrow vs Move
// Run with: cargo run --example 15_closure_capture_modes

fn main() {
    println!("=== Closure Capture Modes ===\n");

    println!("❓ THE QUESTION:\n");
    println!("In Rust, assignment MOVES by default:");
    println!("   let s1 = String::from(\"hello\");");
    println!("   let s2 = s1;  // MOVE! s1 no longer valid\n");
    println!("But closures BORROW by default:");
    println!("   let s = String::from(\"hello\");");
    println!("   let f = || println!(\"{{}}\", s);  // BORROW! s still valid\n");
    println!("Why the difference?\n");
    println!("{}\n", "=".repeat(60));

    // ========== PART 1: Regular Assignment (MOVE) ==========
    println!("Part 1: Regular assignment (MOVE by default)\n");

    println!("1️⃣  Move semantics in regular code:\n");
    {
        let s1 = String::from("hello");
        println!("   s1 = '{}'", s1);

        let s2 = s1;  // MOVE! Ownership transferred
        println!("   s2 = '{}' (moved from s1)", s2);

        // println!("{}", s1);  // ❌ ERROR: value borrowed after move
        println!("   s1 is no longer accessible\n");
    }

    println!("{}\n", "=".repeat(60));

    // ========== PART 2: Closures (BORROW by default) ==========
    println!("Part 2: Closures (BORROW by default)\n");

    println!("2️⃣  Closures borrow by default:\n");
    {
        let s = String::from("world");
        let x = 42;

        println!("   Before closure: s='{}', x={}", s, x);

        let print_values = || {
            // Closure automatically borrows s and x
            println!("   Inside closure: s='{}', x={}", s, x);
        };

        print_values();
        print_values();  // Can call multiple times!

        // ✅ Still accessible!
        println!("   After closure: s='{}', x={}\n", s, x);
    }

    println!("   Why borrow by default?");
    println!("   - Closures are meant to be called multiple times");
    println!("   - You often want to use variables after closure runs");
    println!("   - More convenient and flexible\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 3: Three Capture Modes ==========
    println!("Part 3: Rust compiler chooses the LEAST restrictive mode\n");

    println!("Rust closures use THREE capture modes (in order of preference):\n");
    println!("   1. Fn      - Immutable borrow (&T)");
    println!("   2. FnMut   - Mutable borrow (&mut T)");
    println!("   3. FnOnce  - Takes ownership (T)\n");

    println!("Compiler picks the LEAST restrictive mode that works:\n");

    // Example: Fn (immutable borrow)
    println!("3️⃣  Fn - Immutable borrow (default):\n");
    {
        let text = String::from("hello");

        let read_only = || {
            println!("   Reading: {}", text);  // Just reading → Fn
        };

        read_only();
        read_only();

        println!("   text still accessible: {}", text);
        println!("   Trait: Fn (can call multiple times)\n");
    }

    // Example: FnMut (mutable borrow)
    println!("4️⃣  FnMut - Mutable borrow (when modifying):\n");
    {
        let mut count = 0;

        let mut increment = || {
            count += 1;  // Modifying → needs FnMut
            println!("   Count: {}", count);
        };

        increment();
        increment();

        println!("   count still accessible: {}", count);
        println!("   Trait: FnMut (can call multiple times, mutates)\n");
    }

    // Example: FnOnce (takes ownership)
    println!("5️⃣  FnOnce - Takes ownership (when consuming):\n");
    {
        let data = vec![1, 2, 3];

        let consume = || {
            drop(data);  // Consumes data → needs FnOnce
            println!("   Data consumed!");
        };

        consume();
        // consume();  // ❌ ERROR: can only call once

        // println!("{:?}", data);  // ❌ ERROR: data was moved
        println!("   Trait: FnOnce (can only call once, takes ownership)\n");
    }

    println!("{}\n", "=".repeat(60));

    // ========== PART 4: Using 'move' keyword ==========
    println!("Part 4: Forcing ownership with 'move'\n");

    println!("6️⃣  Without 'move' (borrows):\n");
    {
        let s = String::from("hello");
        let x = 5;

        let borrow_closure = || {
            println!("   s={}, x={}", s, x);
        };

        borrow_closure();
        println!("   After: s={}, x={} (still accessible)\n", s, x);
    }

    println!("7️⃣  With 'move' (takes ownership):\n");
    {
        let s = String::from("world");
        let x = 10;

        let move_closure = move || {
            println!("   s={}, x={}", s, x);
        };

        move_closure();

        // ✅ x accessible (Copy type - was copied)
        println!("   After: x={} (still accessible)", x);

        // ❌ s NOT accessible (Move type - was moved)
        // println!("{}", s);  // ERROR: value borrowed after move
        println!("   s is no longer accessible (moved)\n");
    }

    println!("{}\n", "=".repeat(60));

    // ========== PART 5: Why thread::spawn needs 'move' ==========
    println!("Part 5: Why thread::spawn REQUIRES 'move'\n");

    println!("8️⃣  The lifetime problem:\n");
    println!("   Regular closure (same scope):");
    println!("   {{");
    println!("       let x = 5;");
    println!("       let f = || x + 1;  // Borrows x");
    println!("       f();               // Called in same scope");
    println!("       println!(\"{{}}\", x);  // x still valid ✅");
    println!("   }}\n");

    println!("   Thread (different scope):");
    println!("   {{");
    println!("       let x = 5;");
    println!("       thread::spawn(|| x + 1);  // ❌ ERROR!");
    println!("   }}  // Scope ends, x destroyed");
    println!("   // Thread might still be running → dangling reference!\n");

    println!("   Solution - use 'move':");
    println!("   {{");
    println!("       let x = 5;");
    println!("       thread::spawn(move || x + 1);  // ✅ OK!");
    println!("   }}  // x moved to thread, not destroyed\n");

    use std::thread;

    let x = 100;
    let handle = thread::spawn(move || {
        println!("   Thread: x = {}", x);
        x + 1
    });

    let result = handle.join().unwrap();
    println!("   Result: {}\n", result);

    println!("{}\n", "=".repeat(60));

    // ========== SUMMARY ==========
    println!("💡 SUMMARY\n");

    println!("┌────────────────────────────────────────────────────────┐");
    println!("│ Context              │ Default Behavior                │");
    println!("├────────────────────────────────────────────────────────┤");
    println!("│ Variable assignment  │ MOVE (ownership transfer)       │");
    println!("│   let s2 = s1;       │ s1 becomes invalid              │");
    println!("├────────────────────────────────────────────────────────┤");
    println!("│ Regular closure      │ BORROW (keeps accessible)       │");
    println!("│   let f = || s;      │ s still valid after f()         │");
    println!("├────────────────────────────────────────────────────────┤");
    println!("│ Thread closure       │ MUST use 'move'                 │");
    println!("│   spawn(move || s)   │ s moved to thread               │");
    println!("└────────────────────────────────────────────────────────┘\n");

    println!("WHY do closures borrow by default?");
    println!("  1. Closures are often called multiple times");
    println!("  2. You usually want to keep using variables after closure");
    println!("  3. Compiler can't know how many times you'll call it");
    println!("  4. Borrowing is more flexible (you can always add 'move' later)\n");

    println!("WHEN to use 'move'?");
    println!("  1. thread::spawn - REQUIRED (thread might outlive scope)");
    println!("  2. Returning closures from functions");
    println!("  3. When you want closure to own captured variables");
    println!("  4. When captured variables need to live beyond current scope\n");

    println!("HOW does Rust pick capture mode?");
    println!("  1. Without 'move': Borrows by default (Fn or FnMut)");
    println!("  2. With 'move': Takes ownership (moves or copies)");
    println!("  3. Compiler picks least restrictive mode that satisfies usage");
    println!("  4. Copy types: always copied (cheap)");
    println!("  5. Move types: borrowed unless 'move' used");
}
