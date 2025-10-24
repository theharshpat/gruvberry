// Example 2: Buffered I/O
// Run with: cargo run --example 02_buffered_io

use std::fs::File;
use std::io::{BufReader, BufRead, Write};
use std::time::Instant;

fn main() -> Result<(), std::io::Error> {
    println!("=== Buffered I/O Example ===\n");

    // Create a test file with 10,000 lines
    let mut file = File::create("large_file.txt")?;
    for i in 0..10000 {
        writeln!(file, "Line number {}", i)?;
    }
    println!("✓ Created test file with 10,000 lines");

    // Method 1: Without buffering
    let file = File::open("large_file.txt")?;
    // Reading byte-by-byte would be VERY slow
    drop(file);
    println!("\n📖 Without BufReader: Each read = system call (slow!)");

    // Method 2: With buffering (faster)
    let start = Instant::now();
    let file = File::open("large_file.txt")?;
    let reader = BufReader::new(file);
    let line_count = reader.lines().count();
    let elapsed = start.elapsed();

    println!("📚 With BufReader: Reads in 8KB chunks (fast!)");
    println!("   Lines read: {}", line_count);
    println!("   Time: {:?}", elapsed);

    println!("\n💡 Key concept:");
    println!("  BufReader wraps a File and adds an 8KB RAM buffer");
    println!("  Instead of: read 1 byte → OS call → disk");
    println!("  You get:    read 8192 bytes → OS call → disk (once)");
    println!("              then read from RAM buffer (very fast!)");

    Ok(())
}
