// Example 4: Ownership & Move Semantics
// Run with: cargo run --example 04_ownership

fn take_ownership(s: String) {
    println!("  I now own: {}", s);
    // s is dropped here when function ends
}

fn borrow_value(s: &String) {
    println!("  I'm borrowing: {}", s);
    // s is NOT dropped - we don't own it!
}

fn main() {
    println!("=== Ownership Example ===\n");

    // Example 1: Move
    println!("1. Move semantics:");
    let s1 = String::from("hello");
    let s2 = s1;  // s1 is MOVED to s2

    // println!("{}", s1);  // ❌ ERROR! s1 is no longer valid
    println!("  s2 = {}", s2);  // ✓ OK
    println!("  (s1 is no longer accessible)\n");

    // Example 2: Clone (deep copy)
    println!("2. Clone:");
    let s3 = String::from("world");
    let s4 = s3.clone();  // Create a copy
    println!("  s3 = {}", s3);  // ✓ Both valid
    println!("  s4 = {}\n", s4);

    // Example 3: Function takes ownership
    println!("3. Function taking ownership:");
    let s5 = String::from("data");
    take_ownership(s5);  // s5 is moved into function
    // println!("{}", s5);  // ❌ ERROR! s5 was moved
    println!("  (s5 is no longer accessible)\n");

    // Example 4: Borrowing (reference)
    println!("4. Borrowing with &:");
    let s6 = String::from("borrowed");
    borrow_value(&s6);  // Borrow, don't move
    println!("  Still accessible: {}\n", s6);  // ✓ OK

    // Example 5: Mutable reference
    println!("5. Mutable borrowing:");
    let mut s7 = String::from("mutable");
    println!("  Before: {}", s7);
    modify_value(&mut s7);
    println!("  After: {}\n", s7);

    println!("💡 Key concepts:");
    println!("  - Each value has ONE owner");
    println!("  - When owner goes out of scope, value is dropped");
    println!("  - Move = transfer ownership");
    println!("  - Clone = deep copy (new owner)");
    println!("  - &T = immutable borrow (read-only)");
    println!("  - &mut T = mutable borrow (read-write)");
}

fn modify_value(s: &mut String) {
    s.push_str(" changed!");
}
