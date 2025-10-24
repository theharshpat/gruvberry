// Example 17: Pattern Matching
// Run with: cargo run --example 17_pattern_matching

fn main() {
    println!("=== Pattern Matching ===\n");

    println!("Pattern matching is Rust's way of destructuring and matching data.\n");
    println!("{}\n", "=".repeat(60));

    // ========== PART 1: Basic match ==========
    println!("Part 1: Basic match expressions\n");

    let number = 3;

    match number {
        1 => println!("One!"),
        2 => println!("Two!"),
        3 => println!("Three!"),
        4 | 5 => println!("Four or Five!"),  // Multiple patterns
        _ => println!("Something else"),     // Catch-all (like default)
    }

    println!("\n💡 match is an EXPRESSION (returns a value)\n");

    let message = match number {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "other",
    };
    println!("Message: {}\n", message);

    println!("{}\n", "=".repeat(60));

    // ========== PART 2: Matching enums ==========
    println!("Part 2: Matching enums\n");

    #[allow(dead_code)]
    enum Direction {
        North,
        South,
        East,
        West,
    }

    let direction = Direction::North;

    match direction {
        Direction::North => println!("Going north!"),
        Direction::South => println!("Going south!"),
        Direction::East => println!("Going east!"),
        Direction::West => println!("Going west!"),
    }

    println!("\n✅ Match is EXHAUSTIVE - must cover all cases!\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 3: Destructuring ==========
    println!("Part 3: Destructuring with match\n");

    #[allow(dead_code)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(u8, u8, u8),
    }

    let msg = Message::Move { x: 10, y: 20 };

    match msg {
        Message::Quit => {
            println!("Quit message");
        }
        Message::Move { x, y } => {
            println!("Move to x={}, y={}", x, y);  // Destructure struct
        }
        Message::Write(text) => {
            println!("Write: {}", text);  // Destructure tuple variant
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change color to RGB({}, {}, {})", r, g, b);
        }
    }

    println!("{}\n", "=".repeat(60));

    // ========== PART 4: if let ==========
    println!("Part 4: if let (shorthand for single pattern)\n");

    let some_value = Some(7);

    // With match (verbose)
    match some_value {
        Some(x) => println!("Got value: {}", x),
        None => {}  // Do nothing
    }

    // With if let (concise!)
    if let Some(x) = some_value {
        println!("Got value with if let: {}", x);
    }

    println!("\n💡 Use if let when you only care about ONE pattern\n");

    // Real-world example from gruvberry
    println!("Real-world example:");
    println!("  if let Event::Key(key) = event {{");
    println!("      // Only care about key events, ignore others");
    println!("  }}\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 5: while let ==========
    println!("Part 5: while let (loop while pattern matches)\n");

    let mut stack = vec![1, 2, 3, 4, 5];

    println!("Popping from stack:");
    while let Some(top) = stack.pop() {
        println!("  Popped: {}", top);
    }
    println!("Stack is now empty: {:?}\n", stack);

    println!("{}\n", "=".repeat(60));

    // ========== PART 6: Destructuring tuples ==========
    println!("Part 6: Destructuring tuples\n");

    let point = (3, 5);

    match point {
        (0, 0) => println!("Origin"),
        (0, y) => println!("On Y axis at {}", y),
        (x, 0) => println!("On X axis at {}", x),
        (x, y) => println!("Point at ({}, {})", x, y),
    }

    // Direct destructuring
    let (x, y) = point;
    println!("x={}, y={}\n", x, y);

    println!("{}\n", "=".repeat(60));

    // ========== PART 7: Pattern guards ==========
    println!("Part 7: Pattern guards (extra conditions)\n");

    let num = 4;

    match num {
        x if x < 0 => println!("Negative: {}", x),
        x if x % 2 == 0 => println!("Even: {}", x),
        x => println!("Odd: {}", x),
    }

    println!("\n💡 'if' in match = pattern guard (extra condition)\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 8: Ignoring values ==========
    println!("Part 8: Ignoring values with _ and ..\n");

    let tuple = (1, 2, 3, 4, 5);

    // Ignore specific values
    let (first, _, third, _, fifth) = tuple;
    println!("first={}, third={}, fifth={}", first, third, fifth);

    // Ignore remaining values
    let (a, b, ..) = tuple;
    println!("a={}, b={}, rest ignored\n", a, b);

    println!("{}\n", "=".repeat(60));

    // ========== PART 9: @ bindings ==========
    println!("Part 9: @ bindings (bind and match at same time)\n");

    enum MessageType {
        Hello { id: i32 },
    }

    let msg = MessageType::Hello { id: 5 };

    match msg {
        MessageType::Hello { id: id_var @ 3..=7 } => {
            println!("Found ID in range 3-7: {}", id_var);
        }
        MessageType::Hello { id: 10..=12 } => {
            println!("Found ID in range 10-12");
        }
        MessageType::Hello { id } => {
            println!("Found other ID: {}", id);
        }
    }

    println!("\n💡 @ lets you bind value AND test it\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 10: Real-world examples ==========
    println!("Part 10: Real-world examples from gruvberry\n");

    println!("Example 1: Event handling");
    println!("  match event {{");
    println!("      Event::Key(key) => {{");
    println!("          if key.code == KeyCode::Char('q') {{");
    println!("              break;");
    println!("          }}");
    println!("      }}");
    println!("      Event::Resize(_, _) => {{");
    println!("          // Handle resize");
    println!("      }}");
    println!("      _ => {{}}");
    println!("  }}\n");

    println!("Example 2: Option handling");
    println!("  if let Some(sample) = self.source.next() {{");
    println!("      // Use sample");
    println!("  }}\n");

    println!("Example 3: Result handling");
    println!("  if let Ok(mut buf) = self.buffer.lock() {{");
    println!("      buf.push(sample);");
    println!("  }}\n");

    println!("{}\n", "=".repeat(60));

    // ========== SUMMARY ==========
    println!("💡 SUMMARY\n");

    println!("Pattern Matching Forms:\n");
    println!("  match value {{           // Full pattern matching");
    println!("      pattern => expr,");
    println!("      _ => default,");
    println!("  }}\n");

    println!("  if let pattern = value {{ // Single pattern");
    println!("      // ...");
    println!("  }}\n");

    println!("  while let pattern = expr {{ // Loop while matches");
    println!("      // ...");
    println!("  }}\n");

    println!("Patterns:\n");
    println!("  Literals:        1, 'a', \"hello\"");
    println!("  Variables:       x, y, name");
    println!("  Wildcards:       _, ..");
    println!("  Ranges:          1..=5");
    println!("  Multiple:        1 | 2 | 3");
    println!("  Tuples:          (x, y)");
    println!("  Structs:         Point {{ x, y }}");
    println!("  Enums:           Some(x), None");
    println!("  Guards:          x if x > 5");
    println!("  Bindings:        x @ 1..=5\n");

    println!("Why use pattern matching?");
    println!("  ✅ Exhaustive checking (compiler ensures all cases covered)");
    println!("  ✅ Destructure complex data");
    println!("  ✅ More readable than if/else chains");
    println!("  ✅ Type-safe");
}
