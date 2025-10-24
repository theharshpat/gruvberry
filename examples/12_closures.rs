// Example 12: Closures
// Run with: cargo run --example 12_closures

fn main() {
    println!("=== Closures Example ===\n");

    println!("1. Basic closure:");
    let add_one = |x| x + 1;
    println!("  5 + 1 = {}\n", add_one(5));

    println!("2. Closure with type annotations:");
    let multiply = |x: i32, y: i32| -> i32 { x * y };
    println!("  3 * 4 = {}\n", multiply(3, 4));

    println!("3. Capturing environment:");
    let multiplier = 10;
    let scale = |x| x * multiplier;  // Captures 'multiplier'
    println!("  5 * {} = {}\n", multiplier, scale(5));

    println!("4. Three types of closures:\n");

    // FnOnce - consumes captured variables
    let data = vec![1, 2, 3];
    let consume = || {
        println!("  FnOnce: Consuming {:?}", data);
        drop(data);  // Takes ownership and drops
    };
    consume();
    // consume();  // ❌ ERROR! Can only call once
    // println!("{:?}", data);  // ❌ ERROR! data was moved

    // FnMut - can mutate captured variables
    let mut count = 0;
    let mut increment = || {
        count += 1;
        println!("  FnMut: Count is now {}", count);
    };
    increment();
    increment();
    println!();

    // Fn - immutably borrows captured variables
    let text = String::from("hello");
    let print = || {
        println!("  Fn: Text is '{}'", text);
    };
    print();
    print();  // Can call multiple times
    println!("  Text still accessible: {}\n", text);

    println!("5. Closures with iterators:");
    let numbers = vec![1, 2, 3, 4, 5];

    let doubled: Vec<_> = numbers.iter().map(|x| x * 2).collect();
    println!("  Doubled: {:?}", doubled);

    let evens: Vec<_> = numbers.iter().filter(|x| *x % 2 == 0).collect();
    println!("  Evens: {:?}", evens);

    println!("\n💡 Key concepts:");
    println!("  - Closure = anonymous function");
    println!("  - |params| expr syntax");
    println!("  - Captures environment variables");
    println!("  - FnOnce, FnMut, Fn traits");
    println!("  - Common with iterators: map, filter, etc.");
}
