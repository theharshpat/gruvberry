// Example 24: Rust Basics - Type Conversions, Vec Methods, Modules
// Run with: cargo run --example 24_rust_basics_misc

// Module example
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }

    // Private function (not accessible outside module)
    #[allow(dead_code)]
    fn private_helper() -> i32 {
        42
    }
}

fn main() {
    println!("=== Rust Basics: Miscellaneous Concepts ===\n");
    println!("{}\n", "=".repeat(60));

    // ========== PART 1: Type Conversions ==========
    println!("Part 1: Type conversions (as, From, Into)\n");

    println!("1. 'as' keyword (casting)\n");
    let x: i32 = 5;
    let y: f64 = x as f64;
    let z: u8 = 200;
    let w: i32 = z as i32;

    println!("  let x: i32 = 5;");
    println!("  let y: f64 = x as f64;     // y = {}", y);
    println!("  let z: u8 = 200;");
    println!("  let w: i32 = z as i32;     // w = {}\n", w);

    println!("Common 'as' conversions:");
    println!("  Numeric: i32 as f64, f32 as i32, usize as u64");
    println!("  Truncation: f64 as u8 (truncates decimal)");
    println!("  Pointer: *const T as *mut T (unsafe)\n");

    println!("⚠️  'as' can lose precision or overflow:\n");
    let big: i32 = 300;
    let small: i8 = big as i8;  // Overflow!
    println!("  let big: i32 = 300;");
    println!("  let small: i8 = big as i8;  // small = {} (overflow!)\n", small);

    println!("2. From/Into traits (safe conversions)\n");

    let s = String::from("hello");
    println!("  String::from(\"hello\") = {:?}", s);

    let num: u32 = 5;
    let num_u64: u64 = num.into();
    println!("  let num_u64: u64 = num.into();  // num_u64 = {}\n", num_u64);

    println!("3. to_string() and parse()\n");

    let num = 42;
    let s = num.to_string();
    println!("  42.to_string() = {:?}", s);

    let parsed: i32 = "42".parse().unwrap();
    println!("  \"42\".parse::<i32>() = {}\n", parsed);

    println!("{}\n", "=".repeat(60));

    // ========== PART 2: Vec Methods ==========
    println!("Part 2: Vec (Vector) methods\n");

    println!("Creating Vecs:\n");
    let mut v1 = Vec::new();
    v1.push(1);
    v1.push(2);

    let v2 = vec![1, 2, 3, 4, 5];
    let v3 = vec![0; 5];  // [0, 0, 0, 0, 0]

    println!("  Vec::new()           = {:?}", v1);
    println!("  vec![1, 2, 3, 4, 5]  = {:?}", v2);
    println!("  vec![0; 5]           = {:?} (repeat 0, 5 times)\n", v3);

    println!("Common Vec methods:\n");

    let mut v = vec![1, 2, 3];

    // push/pop
    v.push(4);
    println!("  v.push(4)       → {:?}", v);

    let last = v.pop();
    println!("  v.pop()         → {:?}, popped {:?}", v, last);

    // len/is_empty
    println!("  v.len()         → {}", v.len());
    println!("  v.is_empty()    → {}", v.is_empty());

    // get (returns Option)
    println!("  v.get(0)        → {:?}", v.get(0));
    println!("  v.get(10)       → {:?}", v.get(10));

    // Index access
    println!("  v[0]            → {}", v[0]);
    // println!("{}", v[10]);  // Would panic!

    // first/last
    println!("  v.first()       → {:?}", v.first());
    println!("  v.last()        → {:?}", v.last());

    // contains
    println!("  v.contains(&2)  → {}", v.contains(&2));

    // drain
    let mut v = vec![1, 2, 3, 4, 5];
    v.drain(1..3);  // Removes indices 1, 2
    println!("  v.drain(1..3)   → {:?} (removed indices 1-2)\n", v);

    // clear
    let mut v = vec![1, 2, 3];
    v.clear();
    println!("  v.clear()       → {:?} (all elements removed)", v);

    println!("\nCapacity vs Length:\n");
    let mut v = Vec::with_capacity(10);
    println!("  Vec::with_capacity(10)");
    println!("  len: {}, capacity: {}", v.len(), v.capacity());

    v.push(1);
    v.push(2);
    println!("  After pushing 2 items:");
    println!("  len: {}, capacity: {}\n", v.len(), v.capacity());

    println!("💡 Capacity = allocated space, Length = actual items\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 3: Iteration over Vec ==========
    println!("Part 3: Iterating over Vec\n");

    let v = vec![1, 2, 3, 4, 5];

    println!("Method 1: for loop (borrows)\n");
    for item in &v {
        print!("  {} ", item);
    }
    println!("\n  Vec still accessible: {:?}\n", v);

    println!("Method 2: for loop (mutable)\n");
    let mut v = vec![1, 2, 3];
    for item in &mut v {
        *item *= 2;
    }
    println!("  After doubling: {:?}\n", v);

    println!("Method 3: for loop (consume)\n");
    let v = vec![1, 2, 3];
    for item in v {
        print!("  {} ", item);
    }
    println!("\n  // Vec consumed, can't use it anymore\n");

    println!("Method 4: iter() with methods\n");
    let v = vec![1, 2, 3, 4, 5];
    let sum: i32 = v.iter().sum();
    println!("  v.iter().sum() = {}", sum);

    let doubled: Vec<_> = v.iter().map(|x| x * 2).collect();
    println!("  v.iter().map(|x| x * 2) = {:?}", doubled);

    let evens: Vec<_> = v.iter().filter(|x| *x % 2 == 0).collect();
    println!("  v.iter().filter(evens) = {:?}\n", evens);

    println!("{}\n", "=".repeat(60));

    // ========== PART 4: Modules ==========
    println!("Part 4: Modules and 'use' statements\n");

    println!("Defining a module:\n");
    println!("  mod math {{");
    println!("      pub fn add(a: i32, b: i32) -> i32 {{ a + b }}");
    println!("      pub fn multiply(a: i32, b: i32) -> i32 {{ a * b }}");
    println!("      fn private_helper() -> i32 {{ 42 }}");
    println!("  }}\n");

    println!("Using module functions:\n");
    let sum = math::add(5, 3);
    let product = math::multiply(5, 3);
    println!("  math::add(5, 3)      = {}", sum);
    println!("  math::multiply(5, 3) = {}\n", product);

    println!("Importing with 'use':\n");
    use math::add;
    println!("  use math::add;");
    println!("  add(10, 20)          = {}\n", add(10, 20));

    println!("Visibility:\n");
    println!("  pub fn  - Public (accessible outside module)");
    println!("  fn      - Private (only inside module)\n");

    println!("Real-world module organization:\n");
    println!("  src/");
    println!("    main.rs");
    println!("    lib.rs");
    println!("    math/");
    println!("      mod.rs");
    println!("      operations.rs\n");

    println!("Common 'use' patterns:\n");
    println!("  use std::collections::HashMap;");
    println!("  use std::io::{{Read, Write}};     // Import multiple");
    println!("  use std::sync::*;                // Import all");
    println!("  use std::fmt::Result as FmtResult; // Rename\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 5: String vs &str ==========
    println!("Part 5: String vs &str\n");

    println!("┌─────────────────────────────────────────────────────────┐");
    println!("│ Type     │ Owned/Borrowed │ Mutable │ Size             │");
    println!("├─────────────────────────────────────────────────────────┤");
    println!("│ String   │ Owned          │ Yes     │ Heap, growable   │");
    println!("│ &str     │ Borrowed       │ No      │ View into String │");
    println!("└─────────────────────────────────────────────────────────┘\n");

    let s = String::from("hello");
    let slice: &str = &s[0..3];

    println!("  let s = String::from(\"hello\");  // Owned");
    println!("  let slice: &str = &s[0..3];     // Borrowed");
    println!("  s     = {:?}", s);
    println!("  slice = {:?}\n", slice);

    println!("String literals are &str:\n");
    println!("  let s: &str = \"hello\";  // Type is &'static str\n");

    println!("Converting between String and &str:\n");
    println!("  String → &str:   s.as_str() or &s");
    println!("  &str → String:   s.to_string() or String::from(s)\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 6: Ranges ==========
    println!("Part 6: Ranges\n");

    println!("Inclusive range (includes end):\n");
    for i in 1..=5 {
        print!("  {} ", i);
    }
    println!("\n  1..=5 includes 5\n");

    println!("Exclusive range (excludes end):\n");
    for i in 1..5 {
        print!("  {} ", i);
    }
    println!("\n  1..5 excludes 5\n");

    println!("Range with step:\n");
    for i in (0..10).step_by(2) {
        print!("  {} ", i);
    }
    println!("\n  (0..10).step_by(2)\n");

    println!("Range for indexing:\n");
    let v = vec![10, 20, 30, 40, 50];
    println!("  v = {:?}", v);
    println!("  v[1..4] = {:?}", &v[1..4]);
    println!("  v[..3]  = {:?}", &v[..3]);
    println!("  v[2..]  = {:?}\n", &v[2..]);

    println!("{}\n", "=".repeat(60));

    // ========== PART 7: Common traits ==========
    println!("Part 7: Common derivable traits\n");

    #[derive(Debug, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1, y: 2 };
    let p3 = p1.clone();

    println!("  #[derive(Debug, Clone, PartialEq)]");
    println!("  struct Point {{ x: i32, y: i32 }}\n");

    println!("  Debug:     {:?}", p1);
    println!("  Clone:     {:?} (cloned from p1)", p3);
    println!("  PartialEq: p1 == p2 → {}\n", p1 == p2);

    println!("Common derivable traits:");
    println!("  Debug       - println!(\"{{:?}}\", value)");
    println!("  Clone       - value.clone()");
    println!("  Copy        - Implicit copy on assignment");
    println!("  PartialEq   - == and !=");
    println!("  Eq          - Full equality");
    println!("  PartialOrd  - <, >, <=, >=");
    println!("  Ord         - Full ordering");
    println!("  Default     - Type::default()\n");

    println!("{}\n", "=".repeat(60));

    // ========== SUMMARY ==========
    println!("💡 SUMMARY\n");

    println!("Type conversions:");
    println!("  as           - Casting (can lose precision)");
    println!("  From/Into    - Safe trait-based conversion");
    println!("  to_string()  - Any type → String");
    println!("  parse()      - String → any type\n");

    println!("Vec methods:");
    println!("  push/pop     - Add/remove from end");
    println!("  get()        - Safe indexing (returns Option)");
    println!("  len()        - Number of elements");
    println!("  drain()      - Remove range");
    println!("  iter()       - Create iterator\n");

    println!("Modules:");
    println!("  mod name {{ }}  - Define module");
    println!("  pub          - Make public");
    println!("  use          - Import items\n");

    println!("Strings:");
    println!("  String       - Owned, mutable, heap");
    println!("  &str         - Borrowed, immutable, view\n");

    println!("Ranges:");
    println!("  1..5         - Exclusive (1,2,3,4)");
    println!("  1..=5        - Inclusive (1,2,3,4,5)");
    println!("  v[1..4]      - Slice indexing\n");

    println!("Common traits:");
    println!("  #[derive(Debug, Clone, PartialEq)]");
}
