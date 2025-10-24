// Example 9: Generics
// Run with: cargo run --example 09_generics

use std::fmt::Display;

// Generic function
fn print_twice<T: Display>(item: T) {
    println!("  {}", item);
    println!("  {}", item);
}

// Generic struct
struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T, U> Pair<T, U> {
    fn new(first: T, second: U) -> Self {
        Pair { first, second }
    }
}

// Generic with trait bounds
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    println!("=== Generics Example ===\n");

    println!("1. Generic function:");
    print_twice(42);
    print_twice("hello");
    print_twice(3.14);

    println!("\n2. Generic struct:");
    let pair1 = Pair::new(1, 2);
    println!("  Pair<i32, i32>: {}, {}", pair1.first, pair1.second);

    let pair2 = Pair::new("hello", 42);
    println!("  Pair<&str, i32>: {}, {}", pair2.first, pair2.second);

    println!("\n3. Generic with constraints:");
    let numbers = vec![34, 50, 25, 100, 65];
    println!("  Largest number: {}", largest(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("  Largest char: {}", largest(&chars));

    println!("\n💡 Key concepts:");
    println!("  - <T> = generic type parameter");
    println!("  - Works with any type");
    println!("  - T: Display = trait bound (T must implement Display)");
    println!("  - Zero runtime cost (monomorphization)");
    println!("  - Compiler generates specific code for each type used");
}
