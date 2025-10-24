// Example 1: File I/O Basics
// Run with: cargo run --example 01_file_io

use std::fs::File;
use std::io::Write;

fn main() -> Result<(), std::io::Error> {
    println!("=== File I/O Example ===\n");

    // Create and write to a file
    let mut file = File::create("test_output.txt")?;
    file.write_all(b"Hello from Rust!\n")?;
    println!("✓ Created and wrote to 'test_output.txt'");

    // Open an existing file
    let file = File::open("test_output.txt")?;
    println!("✓ Opened 'test_output.txt' successfully");
    println!("  File size: {} bytes", file.metadata()?.len());

    // What happens if file doesn't exist?
    match File::open("nonexistent.txt") {
        Ok(_) => println!("File found (unexpected!)"),
        Err(e) => println!("✗ Expected error: {}", e),
    }

    println!("\n💡 Key concepts:");
    println!("  - File::open() returns Result<File, Error>");
    println!("  - ? operator: unwraps Ok or returns Err early");
    println!("  - RAII: File auto-closes when out of scope");

    Ok(())
}
