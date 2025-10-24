// Example 3: Result Type & Error Handling
// Run with: cargo run --example 03_result_type

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Cannot divide by zero!".to_string())
    } else {
        Ok(a / b)
    }
}

fn parse_number(s: &str) -> Result<i32, String> {
    match s.parse::<i32>() {
        Ok(num) => Ok(num),
        Err(_) => Err(format!("'{}' is not a valid number", s)),
    }
}

fn calculate(a_str: &str, b_str: &str) -> Result<i32, String> {
    // Using ? operator - returns early if error
    let a = parse_number(a_str)?;  // If Err, return immediately
    let b = parse_number(b_str)?;  // If Err, return immediately
    let result = divide(a, b)?;    // If Err, return immediately
    Ok(result)
}

fn main() {
    println!("=== Result Type Example ===\n");

    // Method 1: Using match
    match divide(10, 2) {
        Ok(result) => println!("✓ 10 / 2 = {}", result),
        Err(e) => println!("✗ Error: {}", e),
    }

    match divide(10, 0) {
        Ok(result) => println!("✓ Result: {}", result),
        Err(e) => println!("✗ Error: {}", e),
    }

    println!();

    // Method 2: Using ? operator (cleaner!)
    println!("Using ? operator:");
    match calculate("20", "4") {
        Ok(r) => println!("✓ 20 / 4 = {}", r),
        Err(e) => println!("✗ Error: {}", e),
    }

    match calculate("10", "abc") {
        Ok(r) => println!("✓ Result: {}", r),
        Err(e) => println!("✗ Error: {}", e),
    }

    match calculate("10", "0") {
        Ok(r) => println!("✓ Result: {}", r),
        Err(e) => println!("✗ Error: {}", e),
    }

    println!("\n💡 Key concepts:");
    println!("  Result<T, E> = Ok(value) or Err(error)");
    println!("  ? operator = unwrap Ok or return Err");
    println!("  No exceptions in Rust - errors are values!");
}
