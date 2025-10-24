// Example 23: Macros
// Run with: cargo run --example 23_macros

fn main() {
    println!("=== Macros in Rust ===\n");

    println!("Macros are code that writes code (metaprogramming).\n");
    println!("{}\n", "=".repeat(60));

    // ========== PART 1: What are macros? ==========
    println!("Part 1: What are macros?\n");

    println!("Macros look like functions but end with ! \n");
    println!("  println!(\"Hello\")   // Macro");
    println!("  print(\"Hello\")      // Function\n");

    println!("Key difference:");
    println!("  Functions:  Run at runtime, work with values");
    println!("  Macros:     Expand at compile time, work with code\n");

    println!("Why macros?");
    println!("  1. Variable number of arguments");
    println!("  2. Generate repetitive code");
    println!("  3. Domain-specific languages (DSLs)");
    println!("  4. Compile-time code generation\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 2: Common built-in macros ==========
    println!("Part 2: Common built-in macros\n");

    println!("1. println! and print!\n");
    println!("  println!(\"Value: {{}}\", 42);");
    println!("  print!(\"No newline\");\n");

    println!("2. format!\n");
    let s = format!("The answer is {}", 42);
    println!("  let s = format!(\"The answer is {{}}\", 42);");
    println!("  s = \"{}\"\n", s);

    println!("3. vec!\n");
    let v = vec![1, 2, 3, 4, 5];
    println!("  let v = vec![1, 2, 3, 4, 5];");
    println!("  v = {:?}\n", v);

    println!("  Without vec! macro:");
    println!("  let mut v = Vec::new();");
    println!("  v.push(1);");
    println!("  v.push(2);");
    println!("  v.push(3);\n");

    println!("4. panic!\n");
    println!("  panic!(\"Something went wrong!\");");
    println!("  // Crashes the program with error message\n");

    println!("5. assert! and assert_eq!\n");
    println!("  assert!(true);                // OK");
    println!("  assert_eq!(2 + 2, 4);         // OK");
    println!("  // assert!(false);            // Would panic!\n");

    assert!(1 + 1 == 2);
    assert_eq!(3 * 3, 9);
    println!("  ✅ Assertions passed!\n");

    println!("6. dbg!\n");
    let x = 5;
    let y = dbg!(x * 2);  // Prints debug info
    println!("  let y = dbg!(x * 2);");
    println!("  y = {}\n", y);

    println!("{}\n", "=".repeat(60));

    // ========== PART 3: Variadic arguments ==========
    println!("Part 3: Why macros? (variable arguments)\n");

    println!("println! can take ANY number of arguments:\n");
    println!("  println!(\"No args\");");
    println!("  println!(\"One: {{}}\", 1);");
    println!("  println!(\"Two: {{}} {{}}\", 1, 2);");
    println!("  println!(\"Three: {{}} {{}} {{}}\", 1, 2, 3);\n");

    println!("Functions CAN'T do this in Rust!");
    println!("Each function must have fixed number of parameters.\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 4: Simple macro example ==========
    println!("Part 4: Defining a simple macro\n");

    macro_rules! say_hello {
        () => {
            println!("Hello from macro!")
        };
    }

    println!("  macro_rules! say_hello {{");
    println!("      () => {{");
    println!("          println!(\"Hello from macro!\")");
    println!("      }};");
    println!("  }}\n");

    println!("  Calling: say_hello!()");
    say_hello!();

    println!("\n💡 macro_rules! defines a macro\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 5: Macro with arguments ==========
    println!("Part 5: Macro with arguments\n");

    macro_rules! create_function {
        ($func_name:ident) => {
            fn $func_name() {
                println!("Function {:?} was called", stringify!($func_name));
            }
        };
    }

    create_function!(foo);
    create_function!(bar);

    println!("  macro_rules! create_function {{");
    println!("      ($func_name:ident) => {{");
    println!("          fn $func_name() {{");
    println!("              println!(\"Function was called\");");
    println!("          }}");
    println!("      }};");
    println!("  }}\n");

    println!("  create_function!(foo);  // Creates fn foo()");
    println!("  create_function!(bar);  // Creates fn bar()\n");

    foo();
    bar();

    println!("\n💡 $func_name:ident captures an identifier\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 6: Pattern matching in macros ==========
    println!("Part 6: Multiple patterns in macros\n");

    macro_rules! calculate {
        (add $a:expr, $b:expr) => {
            $a + $b
        };
        (mul $a:expr, $b:expr) => {
            $a * $b
        };
    }

    println!("  macro_rules! calculate {{");
    println!("      (add $a:expr, $b:expr) => {{ $a + $b }};");
    println!("      (mul $a:expr, $b:expr) => {{ $a * $b }};");
    println!("  }}\n");

    let sum = calculate!(add 5, 3);
    let product = calculate!(mul 5, 3);

    println!("  calculate!(add 5, 3) = {}", sum);
    println!("  calculate!(mul 5, 3) = {}\n", product);

    println!("💡 Macros can match different patterns\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 7: Repetition ==========
    println!("Part 7: Repetition with $(...)*\n");

    macro_rules! vec_strs {
        ($($x:expr),*) => {{
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x.to_string());
            )*
            temp_vec
        }};
    }

    println!("  macro_rules! vec_strs {{");
    println!("      ($($x:expr),*) => {{{{");
    println!("          let mut temp_vec = Vec::new();");
    println!("          $(temp_vec.push($x.to_string());)*");
    println!("          temp_vec");
    println!("      }}}};");
    println!("  }}\n");

    let strings = vec_strs!["hello", "world", "rust"];
    println!("  vec_strs![\"hello\", \"world\", \"rust\"] = {:?}\n", strings);

    println!("Repetition syntax:");
    println!("  $(...)*   - Zero or more");
    println!("  $(...)+   - One or more");
    println!("  $(...)?   - Zero or one\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 8: Designators ==========
    println!("Part 8: Fragment specifiers (designators)\n");

    println!("┌────────────────────────────────────────────────────────┐");
    println!("│ Specifier │ Matches                                    │");
    println!("├────────────────────────────────────────────────────────┤");
    println!("│ ident     │ Identifier (variable/function name)        │");
    println!("│ expr      │ Expression (5 + 3, \"hello\", x * 2)         │");
    println!("│ ty        │ Type (i32, String, Vec<T>)                 │");
    println!("│ pat       │ Pattern (Some(x), _ )                      │");
    println!("│ stmt      │ Statement (let x = 5;)                     │");
    println!("│ block     │ Block ({{ ... }})                            │");
    println!("│ item      │ Item (fn, struct, impl)                    │");
    println!("│ tt        │ Token tree (any single token)              │");
    println!("└────────────────────────────────────────────────────────┘\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 9: Real-world macro examples ==========
    println!("Part 9: Real-world macro examples\n");

    println!("Example 1: HashMap initialization\n");
    println!("  macro_rules! hash_map {{");
    println!("      ($($key:expr => $val:expr),*) => {{{{");
    println!("          let mut map = HashMap::new();");
    println!("          $(map.insert($key, $val);)*");
    println!("          map");
    println!("      }}}};");
    println!("  }}\n");

    println!("  let map = hash_map![");
    println!("      \"one\" => 1,");
    println!("      \"two\" => 2,");
    println!("      \"three\" => 3");
    println!("  ];\n");

    println!("Example 2: Derive macros (used everywhere!)\n");
    println!("  #[derive(Debug, Clone, PartialEq)]");
    println!("  struct Point {{ x: i32, y: i32 }}\n");

    println!("  Automatically implements:");
    println!("  - Debug:      println!(\"{{:?}}\", point)");
    println!("  - Clone:      let p2 = p1.clone()");
    println!("  - PartialEq:  if p1 == p2 {{ ... }}\n");

    println!("Example 3: Test macros\n");
    println!("  #[test]");
    println!("  fn test_addition() {{");
    println!("      assert_eq!(2 + 2, 4);");
    println!("  }}\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 10: When to use macros ==========
    println!("Part 10: When to use macros vs functions\n");

    println!("Use MACROS when:");
    println!("  ✅ Need variable number of arguments");
    println!("  ✅ Need to operate on syntax (code as data)");
    println!("  ✅ Need compile-time code generation");
    println!("  ✅ DRY (Don't Repeat Yourself) for patterns\n");

    println!("Use FUNCTIONS when:");
    println!("  ✅ Normal runtime behavior");
    println!("  ✅ Fixed number of arguments");
    println!("  ✅ Type checking and error messages matter");
    println!("  ✅ Most cases! (prefer functions by default)\n");

    println!("Downsides of macros:");
    println!("  ❌ Harder to understand");
    println!("  ❌ Worse error messages");
    println!("  ❌ Harder to debug");
    println!("  ❌ Hygiene issues (variable capture)\n");

    println!("{}\n", "=".repeat(60));

    // ========== SUMMARY ==========
    println!("💡 SUMMARY\n");

    println!("What are macros?");
    println!("  - Code that generates code");
    println!("  - Identified by ! (println!, vec!, macro_name!)");
    println!("  - Expanded at compile time\n");

    println!("Common built-in macros:");
    println!("  println!(...)     - Print with newline");
    println!("  format!(...)      - Create String");
    println!("  vec![...]         - Create Vec");
    println!("  panic!(...)       - Crash with message");
    println!("  assert!(...)      - Runtime check");
    println!("  dbg!(...)         - Debug print\n");

    println!("Defining macros:");
    println!("  macro_rules! name {{");
    println!("      (pattern) => {{ expansion }};");
    println!("  }}\n");

    println!("Fragment specifiers:");
    println!("  $name:ident   - Identifier");
    println!("  $name:expr    - Expression");
    println!("  $name:ty      - Type");
    println!("  $(...)*       - Repetition\n");

    println!("Real-world usage:");
    println!("  - Variadic arguments (println!)");
    println!("  - Code generation (vec!, hash_map!)");
    println!("  - Derive traits (#[derive(...)])");
    println!("  - Testing (#[test])");
    println!("  - DSLs (domain-specific languages)\n");

    println!("Remember:");
    println!("  Prefer functions over macros when possible!");
    println!("  Macros are powerful but harder to maintain.");
}
