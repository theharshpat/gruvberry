// Example 5: Borrowing Rules
// Run with: cargo run --example 05_borrowing

fn main() {
    println!("=== Borrowing Rules Example ===\n");

    // Rule 1: Multiple immutable borrows OK
    println!("1. Multiple immutable borrows:");
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;
    println!("  r1={}, r2={}, r3={}", r1, r2, r3);
    println!("  ✓ Multiple & borrows allowed!\n");

    // Rule 2: Only ONE mutable borrow at a time
    println!("2. One mutable borrow:");
    let mut s = String::from("world");
    let r1 = &mut s;
    r1.push_str("!");
    println!("  r1={}", r1);
    // let r2 = &mut s;  // ❌ ERROR! Can't have two &mut
    println!("  ✓ Only one &mut borrow at a time!\n");

    // Rule 3: Can't mix & and &mut
    println!("3. Can't mix & and &mut:");
    let mut s = String::from("data");
    let r1 = &s;      // Immutable borrow
    let r2 = &s;      // Another immutable borrow
    println!("  r1={}, r2={}", r1, r2);
    // let r3 = &mut s;  // ❌ ERROR! Can't have &mut while & exists

    // After last use of r1 and r2, we can create &mut
    let r3 = &mut s;  // ✓ OK now!
    r3.push_str("!");
    println!("  r3={}\n", r3);

    // Rule 4: References must be valid
    println!("4. Dangling references prevented:");
    // Uncommenting this would cause a compile error:
    // let reference;
    // {
    //     let value = String::from("temporary");
    //     reference = &value;  // ❌ ERROR! value dies at end of scope
    // }
    // println!("{}", reference);  // Would be dangling!
    println!("  ✓ Compiler prevents dangling references!\n");

    println!("💡 Borrowing Rules:");
    println!("  1. Any number of & (immutable) borrows");
    println!("  2. OR exactly one &mut (mutable) borrow");
    println!("  3. References must be valid (no dangling)");
}
