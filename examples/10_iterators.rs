// Example 10: Iterators
// Run with: cargo run --example 10_iterators

struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Counter {
        Counter { count: 0, max }
    }
}

// Implement Iterator trait
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    println!("=== Iterators Example ===\n");

    println!("1. Basic iteration:");
    let v = vec![1, 2, 3, 4, 5];
    for item in v.iter() {
        print!("{} ", item);
    }
    println!("\n");

    println!("2. Iterator methods (lazy!):");
    let v = vec![1, 2, 3, 4, 5];

    // Nothing happens yet - iterators are lazy!
    let doubled = v.iter().map(|x| x * 2);

    // Collect forces evaluation
    let result: Vec<i32> = doubled.collect();
    println!("  Original: {:?}", v);
    println!("  Doubled: {:?}\n", result);

    println!("3. Chaining iterator methods:");
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result: Vec<i32> = v.iter()
        .filter(|x| *x % 2 == 0)  // Keep evens
        .map(|x| x * x)            // Square them
        .collect();
    println!("  Even numbers squared: {:?}\n", result);

    println!("4. Custom iterator:");
    let counter = Counter::new(5);
    for num in counter {
        print!("{} ", num);
    }
    println!("\n");

    println!("5. Iterator adapters:");
    let v = vec![1, 2, 3];

    // take(n) - first n items
    let first_two: Vec<_> = v.iter().take(2).collect();
    println!("  First 2: {:?}", first_two);

    // skip(n) - skip first n items
    let skip_one: Vec<_> = v.iter().skip(1).collect();
    println!("  Skip 1: {:?}", skip_one);

    // enumerate - add index
    for (i, val) in v.iter().enumerate() {
        println!("  Index {}: {}", i, val);
    }

    println!("\n💡 Key concepts:");
    println!("  - Iterator trait: type Item + fn next()");
    println!("  - Lazy evaluation: nothing happens until consumed");
    println!("  - map, filter, collect, etc. are adapters");
    println!("  - Zero-cost abstractions (as fast as loops!)");
}
