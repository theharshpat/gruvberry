// Example 21: Smart Pointers (Box, Rc, Arc)
// Run with: cargo run --example 21_smart_pointers

use std::rc::Rc;
use std::sync::Arc;

fn main() {
    println!("=== Smart Pointers: Box, Rc, Arc ===\n");

    println!("Smart pointers are data structures that act like pointers");
    println!("but have additional metadata and capabilities.\n");
    println!("{}\n", "=".repeat(60));

    // ========== PART 1: Box<T> - Heap allocation ==========
    println!("Part 1: Box<T> - Allocate on heap\n");

    println!("❓ Why Box?\n");
    println!("1. Store data on HEAP instead of stack");
    println!("2. Transfer ownership of large data without copying");
    println!("3. Trait objects (dyn Trait)");
    println!("4. Recursive types\n");

    // Simple box
    let boxed_int = Box::new(5);
    println!("  let boxed_int = Box::new(5);");
    println!("  Value: {}", boxed_int);
    println!("  Type: Box<i32>\n");

    println!("Stack vs Heap:");
    println!("  let x = 5;           // Stack: 4 bytes");
    println!("  let x = Box::new(5); // Heap: pointer on stack (8 bytes)");
    println!("                       //       value on heap (4 bytes)\n");

    // Large data
    let large_array = Box::new([0; 1000000]);
    println!("  let large_array = Box::new([0; 1000000]);");
    println!("  Allocated {} bytes on heap", std::mem::size_of_val(&*large_array));
    println!("  Pointer on stack: {} bytes\n", std::mem::size_of_val(&large_array));

    println!("💡 Box automatically deallocates when it goes out of scope (RAII)\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 2: Recursive types with Box ==========
    println!("Part 2: Recursive types (MUST use Box)\n");

    #[derive(Debug)]
    #[allow(dead_code)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    use List::{Cons, Nil};

    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil)
            ))
        ))
    );

    println!("  enum List {{");
    println!("      Cons(i32, Box<List>),  // Box breaks infinite size!");
    println!("      Nil,");
    println!("  }}\n");

    println!("  Created list: {:?}\n", list);

    println!("Why Box is needed:");
    println!("  ❌ Without Box: Compiler can't calculate size");
    println!("     (List contains List, which contains List, ...)");
    println!("  ✅ With Box: Size is known (pointer = 8 bytes)\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 3: Box vs regular references ==========
    println!("Part 3: Box<T> vs &T\n");

    println!("┌─────────────────────────────────────────────────────────┐");
    println!("│ Feature        │ Box<T>           │ &T                 │");
    println!("├─────────────────────────────────────────────────────────┤");
    println!("│ Ownership      │ Owns the data    │ Borrows            │");
    println!("│ Heap/Stack     │ Heap             │ Either             │");
    println!("│ Lifetime       │ Till Box drops   │ Must track         │");
    println!("│ Mutable        │ Can own mut data │ Needs &mut         │");
    println!("└─────────────────────────────────────────────────────────┘\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 4: Rc<T> - Reference counting (single-thread) ==========
    println!("Part 4: Rc<T> - Multiple owners (single-threaded)\n");

    println!("❓ Why Rc?\n");
    println!("When you want MULTIPLE parts of code to OWN the same data\n");

    let data = Rc::new(String::from("shared data"));
    println!("  let data = Rc::new(String::from(\"shared data\"));");
    println!("  Reference count: {}\n", Rc::strong_count(&data));

    let data2 = Rc::clone(&data);  // Increment ref count
    let data3 = Rc::clone(&data);

    println!("  let data2 = Rc::clone(&data);");
    println!("  let data3 = Rc::clone(&data);");
    println!("  Reference count: {}\n", Rc::strong_count(&data));

    println!("  data:  {:?}", data);
    println!("  data2: {:?}", data2);
    println!("  data3: {:?}\n", data3);

    drop(data2);
    println!("  After drop(data2):");
    println!("  Reference count: {}\n", Rc::strong_count(&data));

    println!("💡 Rc::clone() is CHEAP - just increments counter");
    println!("💡 Data freed when count reaches 0");
    println!("⚠️  Rc is NOT thread-safe (single-threaded only)\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 5: Arc<T> - Atomic reference counting (multi-thread) ==========
    println!("Part 5: Arc<T> - Multiple owners (multi-threaded)\n");

    println!("Arc = Atomic Rc (thread-safe version)\n");

    use std::thread;

    let shared = Arc::new(vec![1, 2, 3, 4, 5]);
    println!("  let shared = Arc::new(vec![1, 2, 3, 4, 5]);");
    println!("  Reference count: {}\n", Arc::strong_count(&shared));

    let shared1 = Arc::clone(&shared);
    let shared2 = Arc::clone(&shared);

    let handle1 = thread::spawn(move || {
        println!("    Thread 1: {:?}", shared1);
    });

    let handle2 = thread::spawn(move || {
        println!("    Thread 2: {:?}", shared2);
    });

    println!("  Main thread: {:?}", shared);
    println!("  Reference count: {}\n", Arc::strong_count(&shared));

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("  After threads finish:");
    println!("  Reference count: {}\n", Arc::strong_count(&shared));

    println!("💡 Arc has atomic operations (slightly slower than Rc)");
    println!("💡 Use Arc for multi-threaded, Rc for single-threaded\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 6: Comparison table ==========
    println!("Part 6: Box vs Rc vs Arc comparison\n");

    println!("┌─────────────────────────────────────────────────────────────┐");
    println!("│ Type   │ Ownership   │ Thread-safe │ Overhead   │ Use case │");
    println!("├─────────────────────────────────────────────────────────────┤");
    println!("│ Box<T> │ Single      │ Yes         │ None       │ Heap     │");
    println!("│        │             │             │            │ alloc    │");
    println!("├─────────────────────────────────────────────────────────────┤");
    println!("│ Rc<T>  │ Multiple    │ NO          │ Low        │ Shared   │");
    println!("│        │             │             │ (counter)  │ single   │");
    println!("│        │             │             │            │ thread   │");
    println!("├─────────────────────────────────────────────────────────────┤");
    println!("│ Arc<T> │ Multiple    │ YES         │ Medium     │ Shared   │");
    println!("│        │             │             │ (atomic)   │ multi    │");
    println!("│        │             │             │            │ thread   │");
    println!("└─────────────────────────────────────────────────────────────┘\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 7: Important notes ==========
    println!("Part 7: Important notes about Rc/Arc\n");

    println!("⚠️  Rc and Arc provide IMMUTABLE access by default!\n");

    println!("  let data = Rc::new(vec![1, 2, 3]);");
    println!("  data.push(4);  // ❌ ERROR! Can't mutate through Rc\n");

    println!("To mutate, combine with RefCell (Rc) or Mutex (Arc):\n");

    println!("  use std::cell::RefCell;");
    println!("  let data = Rc::new(RefCell::new(vec![1, 2, 3]));");
    println!("  data.borrow_mut().push(4);  // ✅ OK!\n");

    println!("  use std::sync::Mutex;");
    println!("  let data = Arc::new(Mutex::new(vec![1, 2, 3]));");
    println!("  data.lock().unwrap().push(4);  // ✅ OK!\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 8: When to use which ==========
    println!("Part 8: Decision guide\n");

    println!("┌──────────────────────────────────────────────────────┐");
    println!("│ Question                  │ Answer                   │");
    println!("├──────────────────────────────────────────────────────┤");
    println!("│ Need heap allocation?     │ Box<T>                   │");
    println!("│ Recursive type?           │ Box<T>                   │");
    println!("│ Trait object?             │ Box<dyn Trait>           │");
    println!("├──────────────────────────────────────────────────────┤");
    println!("│ Multiple owners?          │ Rc<T> or Arc<T>          │");
    println!("│ Single-threaded?          │ Rc<T>                    │");
    println!("│ Multi-threaded?           │ Arc<T>                   │");
    println!("├──────────────────────────────────────────────────────┤");
    println!("│ Need to mutate?           │ + RefCell or Mutex       │");
    println!("│ Single-threaded mutate?   │ Rc<RefCell<T>>           │");
    println!("│ Multi-threaded mutate?    │ Arc<Mutex<T>>            │");
    println!("└──────────────────────────────────────────────────────┘\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 9: Real-world from gruvberry ==========
    println!("Part 9: Real-world usage in gruvberry\n");

    println!("Example 1: Arc<Mutex<Vec<f32>>>\n");
    println!("  let buffer = Arc::new(Mutex::new(Vec::new()));");
    println!("  let buffer_clone = buffer.clone();\n");
    println!("  thread::spawn(move || {{");
    println!("      buffer_clone.lock().unwrap().push(sample);");
    println!("  }});\n");

    println!("Why this combination?");
    println!("  Arc        - Multiple owners (main + thread)");
    println!("  Mutex      - Thread-safe mutation");
    println!("  Vec<f32>   - The actual data (audio samples)\n");

    println!("Example 2: Box<dyn Trait>\n");
    println!("  let animals: Vec<Box<dyn Animal>> = vec![");
    println!("      Box::new(Dog {{ name: \"Buddy\" }}),");
    println!("      Box::new(Cat {{ name: \"Whiskers\" }}),");
    println!("  ];\n");

    println!("Why Box?");
    println!("  - Trait objects (dyn Trait) have unknown size");
    println!("  - Box puts them on heap with known pointer size");
    println!("  - Allows storing different types in same Vec\n");

    println!("{}\n", "=".repeat(60));

    // ========== SUMMARY ==========
    println!("💡 SUMMARY\n");

    println!("Box<T>:");
    println!("  - Heap allocation");
    println!("  - Single ownership");
    println!("  - Zero runtime overhead");
    println!("  - Use for: large data, recursive types, trait objects\n");

    println!("Rc<T>:");
    println!("  - Reference counting (single-threaded)");
    println!("  - Multiple ownership");
    println!("  - Cheap clone (increments counter)");
    println!("  - Use for: shared data in single-threaded code\n");

    println!("Arc<T>:");
    println!("  - Atomic reference counting (multi-threaded)");
    println!("  - Multiple ownership (thread-safe)");
    println!("  - Slightly more expensive than Rc");
    println!("  - Use for: shared data across threads\n");

    println!("Key differences:");
    println!("  Box  = Single owner, heap, no overhead");
    println!("  Rc   = Multiple owners, single-thread, low overhead");
    println!("  Arc  = Multiple owners, multi-thread, atomic overhead\n");

    println!("Common combinations:");
    println!("  Box<dyn Trait>     - Trait objects");
    println!("  Rc<RefCell<T>>     - Shared mutable (single-thread)");
    println!("  Arc<Mutex<T>>      - Shared mutable (multi-thread)");
}
