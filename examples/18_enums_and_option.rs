// Example 18: Enums & Option<T>
// Run with: cargo run --example 18_enums_and_option

fn main() {
    println!("=== Enums & Option<T> ===\n");

    println!("Enums let you define a type with multiple possible variants.\n");
    println!("{}\n", "=".repeat(60));

    // ========== PART 1: Basic Enums ==========
    println!("Part 1: Basic enums\n");

    #[allow(dead_code)]
    enum TrafficLight {
        Red,
        Yellow,
        Green,
    }

    let light = TrafficLight::Green;

    match light {
        TrafficLight::Red => println!("Stop!"),
        TrafficLight::Yellow => println!("Slow down!"),
        TrafficLight::Green => println!("Go!"),
    }

    println!("\n💡 Enum variants are namespaced: TrafficLight::Red\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 2: Enums with Data ==========
    println!("Part 2: Enums with data (most powerful feature!)\n");

    #[allow(dead_code)]
    enum IpAddress {
        V4(u8, u8, u8, u8),      // Tuple variant
        V6(String),               // Single value variant
    }

    let home = IpAddress::V4(127, 0, 0, 1);
    let _loopback = IpAddress::V6(String::from("::1"));

    match home {
        IpAddress::V4(a, b, c, d) => {
            println!("IPv4: {}.{}.{}.{}", a, b, c, d);
        }
        IpAddress::V6(addr) => {
            println!("IPv6: {}", addr);
        }
    }

    println!("\n💡 Each variant can hold DIFFERENT types of data!\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 3: Struct-like variants ==========
    println!("Part 3: Struct-like enum variants\n");

    #[allow(dead_code)]
    enum Message {
        Quit,                           // No data
        Move { x: i32, y: i32 },       // Named fields
        Write(String),                  // Tuple variant
        ChangeColor(u8, u8, u8),       // Multiple values
    }

    let msg = Message::Move { x: 10, y: 20 };

    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Text: {}", text),
        Message::ChangeColor(r, g, b) => println!("Color: ({}, {}, {})", r, g, b),
    }

    println!("\n💡 Enums can mix different variant styles!\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 4: Option<T> (THE MOST IMPORTANT ENUM) ==========
    println!("Part 4: Option<T> - Rust's way of handling 'null'\n");

    println!("❓ THE PROBLEM:\n");
    println!("In many languages, variables can be 'null':");
    println!("  String name = null;  // Allowed in Java/C#");
    println!("  name.length();       // CRASH! NullPointerException\n");

    println!("✅ RUST'S SOLUTION: Option<T>\n");
    println!("Rust has NO null! Instead, use Option:\n");

    println!("enum Option<T> {{");
    println!("    Some(T),   // Has a value");
    println!("    None,      // No value (like null, but safe!)");
    println!("}}\n");

    let some_number: Option<i32> = Some(5);
    let no_number: Option<i32> = None;

    println!("some_number = {:?}", some_number);
    println!("no_number = {:?}\n", no_number);

    println!("💡 Option<T> forces you to handle the 'no value' case!\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 5: Working with Option ==========
    println!("Part 5: Working with Option<T>\n");

    println!("Method 1: match\n");
    let x = Some(5);

    match x {
        Some(value) => println!("  Got value: {}", value),
        None => println!("  No value!"),
    }

    println!("\nMethod 2: if let\n");
    if let Some(value) = x {
        println!("  Got value: {}", value);
    }

    println!("\nMethod 3: unwrap() - use carefully!\n");
    println!("  x.unwrap() = {}", x.unwrap());
    println!("  ⚠️  WARNING: unwrap() panics if None!");
    // no_number.unwrap();  // Would crash!

    println!("\nMethod 4: unwrap_or() - safe default\n");
    println!("  x.unwrap_or(0) = {}", x.unwrap_or(0));
    println!("  no_number.unwrap_or(0) = {}", no_number.unwrap_or(0));

    println!("\nMethod 5: ? operator (in functions returning Option)\n");
    println!("  fn get_value() -> Option<i32> {{");
    println!("      let x = some_option?;  // Returns early if None");
    println!("      Some(x + 1)");
    println!("  }}\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 6: Option methods ==========
    println!("Part 6: Useful Option methods\n");

    let maybe_value = Some(10);

    // is_some() / is_none()
    println!("is_some(): {}", maybe_value.is_some());
    println!("is_none(): {}", maybe_value.is_none());

    // map() - transform the value inside
    let doubled = maybe_value.map(|x| x * 2);
    println!("map(|x| x * 2): {:?}", doubled);

    // and_then() - chain operations
    let result = maybe_value.and_then(|x| {
        if x > 5 {
            Some(x * 2)
        } else {
            None
        }
    });
    println!("and_then: {:?}", result);

    // filter() - keep only if condition
    let filtered = maybe_value.filter(|x| *x > 5);
    println!("filter(|x| *x > 5): {:?}", filtered);

    // unwrap_or_else() - compute default
    let value = None.unwrap_or_else(|| {
        println!("  Computing default...");
        42
    });
    println!("unwrap_or_else: {}\n", value);

    println!("{}\n", "=".repeat(60));

    // ========== PART 7: Real-world examples ==========
    println!("Part 7: Real-world examples from gruvberry\n");

    println!("Example 1: Iterator next() returns Option\n");
    println!("  fn next(&mut self) -> Option<Self::Item> {{");
    println!("      if let Some(sample) = self.source.next() {{");
    println!("          Some(sample)  // Has value");
    println!("      }} else {{");
    println!("          None          // No more items");
    println!("      }}");
    println!("  }}\n");

    println!("Example 2: Optional duration\n");
    println!("  fn total_duration(&self) -> Option<Duration> {{");
    println!("      self.source.total_duration()");
    println!("  }}");
    println!("  // Returns None if duration unknown\n");

    println!("Example 3: Finding in a vector\n");
    let numbers = vec![1, 2, 3, 4, 5];
    let found = numbers.iter().find(|&&x| x == 3);
    println!("  numbers.iter().find(|&&x| x == 3) = {:?}", found);
    let not_found = numbers.iter().find(|&&x| x == 10);
    println!("  numbers.iter().find(|&&x| x == 10) = {:?}\n", not_found);

    println!("{}\n", "=".repeat(60));

    // ========== PART 8: Comparing with other languages ==========
    println!("Part 8: Option vs null in other languages\n");

    println!("┌────────────────────────────────────────────────────────┐");
    println!("│ Language      │ Nullable handling                      │");
    println!("├────────────────────────────────────────────────────────┤");
    println!("│ Java/C#       │ String s = null; (runtime crashes)    │");
    println!("│ JavaScript    │ let x = undefined/null; (bugs!)        │");
    println!("│ Python        │ x = None (runtime errors)              │");
    println!("│ Rust          │ Option<T> (compile-time safety!) ✅    │");
    println!("└────────────────────────────────────────────────────────┘\n");

    println!("Why Option is better:\n");
    println!("  ❌ With null: Compiler can't help you");
    println!("     String name = null;");
    println!("     name.length();  // CRASH at runtime!\n");

    println!("  ✅ With Option: Compiler FORCES you to check");
    println!("     let name: Option<String> = None;");
    println!("     name.length();  // ERROR: can't call on Option!");
    println!("     // Must unwrap or match first\n");

    println!("{}\n", "=".repeat(60));

    // ========== SUMMARY ==========
    println!("💡 SUMMARY\n");

    println!("Enums:\n");
    println!("  - Define types with multiple variants");
    println!("  - Each variant can hold different data");
    println!("  - Exhaustive matching (compiler checks all cases)");
    println!("  - Syntax:");
    println!("      enum Name {{");
    println!("          Variant1,                    // No data");
    println!("          Variant2(Type),              // Tuple");
    println!("          Variant3 {{ field: Type }},    // Struct");
    println!("      }}\n");

    println!("Option<T>:\n");
    println!("  - Rust's way of representing 'optional' values");
    println!("  - Replaces 'null' from other languages");
    println!("  - Two variants: Some(T) or None");
    println!("  - Forces explicit handling (no null crashes!)\n");

    println!("Common patterns:\n");
    println!("  match option {{              // Full control");
    println!("      Some(x) => use(x),");
    println!("      None => default,");
    println!("  }}\n");

    println!("  if let Some(x) = option {{   // Only care about Some");
    println!("      use(x);");
    println!("  }}\n");

    println!("  option.unwrap_or(default)  // Safe default value\n");

    println!("  option?                    // Propagate None (in functions)\n");

    println!("Key methods:\n");
    println!("  is_some(), is_none()       // Check variant");
    println!("  unwrap()                   // Get value (panics on None!)");
    println!("  unwrap_or(default)         // Get value or default");
    println!("  map(f)                     // Transform Some value");
    println!("  and_then(f)                // Chain operations");
    println!("  filter(predicate)          // Keep if condition");
}
