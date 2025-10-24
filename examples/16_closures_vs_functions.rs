// Example 16: Closures vs Functions - What's the difference?
// Run with: cargo run --example 16_closures_vs_functions

fn main() {
    println!("=== Closures vs Functions ===\n");

    println!("❓ YOUR QUESTIONS:\n");
    println!("1. Are closures just functions without name/args?");
    println!("2. What scope can closures access?");
    println!("3. Why store closures in variables?\n");
    println!("{}\n", "=".repeat(60));

    // ========== QUESTION 1: Closures vs Functions ==========
    println!("Question 1: Are closures just anonymous functions?\n");

    println!("SIMILARITY: Both are callable code\n");

    // Regular function
    fn add_regular(x: i32, y: i32) -> i32 {
        x + y
    }

    // Closure (WITH arguments!)
    let add_closure = |x: i32, y: i32| -> i32 {
        x + y
    };

    println!("Regular function: add_regular(5, 3) = {}", add_regular(5, 3));
    println!("Closure:          add_closure(5, 3) = {}", add_closure(5, 3));
    println!("\n✅ Closures CAN have arguments! Just like functions.\n");

    println!("KEY DIFFERENCE: Closures can CAPTURE their environment!\n");

    let multiplier = 10;

    // Regular function - CANNOT access 'multiplier'
    // fn multiply(x: i32) -> i32 {
    //     x * multiplier  // ❌ ERROR: can't capture from environment
    // }

    // Closure - CAN access 'multiplier'
    let multiply = |x: i32| -> i32 {
        x * multiplier  // ✅ OK: captures 'multiplier' from environment
    };

    println!("multiplier = {}", multiplier);
    println!("multiply(5) = {} (uses captured multiplier)", multiply(5));
    println!("\n✅ Closures capture variables from surrounding scope!\n");

    println!("{}\n", "=".repeat(60));

    // ========== QUESTION 2: Scope Access ==========
    println!("Question 2: What scope can closures access?\n");

    println!("Answer: Closures capture variables from WHERE they are DEFINED\n");

    let outer = 100;

    {
        let inner = 200;

        let closure_here = || {
            println!("  Can access outer: {}", outer);  // ✅ Defined outside
            println!("  Can access inner: {}", inner);  // ✅ In same scope
        };

        closure_here();
    }  // 'inner' and 'closure_here' dropped here

    println!("\n⚠️  Closure captures from WHERE it's DEFINED, not where it's CALLED\n");

    // Example: closure defined in inner scope
    fn create_closure() -> impl Fn() {
        let message = String::from("Hello from closure!");

        move || {
            // Captures 'message' from where it's DEFINED
            println!("{}", message);
        }
    }

    let my_closure = create_closure();
    my_closure();  // Called here, but uses 'message' from create_closure
    println!("\n✅ Closure brought 'message' with it (via 'move')\n");

    println!("{}\n", "=".repeat(60));

    // ========== QUESTION 3: Why store closures? ==========
    println!("Question 3: Why store closures in variables?\n");

    println!("USE CASE 1: Pass to functions (callbacks)\n");

    fn apply_operation(x: i32, operation: impl Fn(i32) -> i32) -> i32 {
        operation(x)
    }

    let double = |x| x * 2;
    let square = |x| x * x;

    println!("  apply_operation(5, double) = {}", apply_operation(5, double));
    println!("  apply_operation(5, square) = {}", apply_operation(5, square));
    println!("\n  ✅ Different behavior without changing apply_operation!\n");

    println!("USE CASE 2: Store in data structures (strategy pattern)\n");

    struct Calculator {
        operation: Box<dyn Fn(i32, i32) -> i32>,
    }

    let add_calc = Calculator {
        operation: Box::new(|a, b| a + b),
    };

    let mul_calc = Calculator {
        operation: Box::new(|a, b| a * b),
    };

    println!("  add_calc.operation(10, 5) = {}", (add_calc.operation)(10, 5));
    println!("  mul_calc.operation(10, 5) = {}", (mul_calc.operation)(10, 5));
    println!("\n  ✅ Change behavior by swapping closures!\n");

    println!("USE CASE 3: Iterator methods (map, filter, etc.)\n");

    let numbers = vec![1, 2, 3, 4, 5];

    // Without storing closure
    let doubled: Vec<_> = numbers.iter().map(|x| x * 2).collect();
    println!("  Inline:  {:?}", doubled);

    // Storing closure first
    let times_three = |x: &i32| x * 3;
    let tripled: Vec<_> = numbers.iter().map(times_three).collect();
    println!("  Stored:  {:?}", tripled);
    println!("\n  ✅ Both work! Storing is clearer when logic is complex\n");

    println!("USE CASE 4: Lazy evaluation / Delay execution\n");

    let expensive_computation = || {
        println!("  Computing...");
        std::thread::sleep(std::time::Duration::from_millis(100));
        42
    };

    println!("  Closure stored, but NOT executed yet!");
    std::thread::sleep(std::time::Duration::from_millis(100));
    println!("  Now calling it...");
    let result = expensive_computation();
    println!("  Result: {}\n", result);
    println!("  ✅ Control WHEN computation happens!\n");

    println!("USE CASE 5: Event handlers / Callbacks\n");

    use std::rc::Rc;
    use std::cell::RefCell;

    struct Button {
        label: String,
        on_click: Box<dyn Fn()>,
    }

    impl Button {
        fn click(&self) {
            println!("  Button '{}' clicked!", self.label);
            (self.on_click)();
        }
    }

    let counter = Rc::new(RefCell::new(0));
    let counter_clone = counter.clone();

    let button = Button {
        label: "Click me".to_string(),
        on_click: Box::new(move || {
            *counter_clone.borrow_mut() += 1;
            println!("  Clicked {} times", counter_clone.borrow());
        }),
    };

    button.click();
    button.click();
    button.click();
    println!("\n  ✅ Attach custom behavior to events!\n");

    println!("USE CASE 6: Configuration / Dependency Injection\n");

    fn process_data(data: Vec<i32>, validator: impl Fn(&i32) -> bool) -> Vec<i32> {
        data.into_iter().filter(|x| validator(x)).collect()
    }

    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let evens = process_data(data.clone(), |x| x % 2 == 0);
    let greater_than_5 = process_data(data.clone(), |x| x > &5);

    println!("  Evens: {:?}", evens);
    println!("  Greater than 5: {:?}", greater_than_5);
    println!("\n  ✅ Same function, different behavior via closures!\n");

    println!("{}\n", "=".repeat(60));

    // ========== REAL WORLD EXAMPLE ==========
    println!("REAL WORLD: How closures are used in gruvberry\n");

    println!("In the audio visualizer:\n");

    println!("1. Iterator methods:");
    println!("   lines.iter().enumerate().map(|(i, line)| {{");
    println!("       // Transform each line with custom logic");
    println!("   }})\n");

    println!("2. Thread spawning:");
    println!("   let buffer = Arc::new(Mutex::new(vec![]));");
    println!("   thread::spawn(move || {{");
    println!("       // Capture 'buffer' and run in background");
    println!("   }})\n");

    println!("3. Event handling:");
    println!("   if let Event::Key(key) = event {{");
    println!("       // Process key events with inline logic");
    println!("   }}\n");

    println!("{}\n", "=".repeat(60));

    // ========== SUMMARY ==========
    println!("💡 SUMMARY\n");

    println!("┌─────────────────────────────────────────────────────────┐");
    println!("│ Question                  │ Answer                      │");
    println!("├─────────────────────────────────────────────────────────┤");
    println!("│ Closures without args?    │ No! Can have args like fns  │");
    println!("│ Anonymous functions?      │ Yes, but MORE - can capture │");
    println!("│ What scope to access?     │ Where DEFINED, not CALLED   │");
    println!("│ Why store in variable?    │ See 6 use cases above       │");
    println!("└─────────────────────────────────────────────────────────┘\n");

    println!("Closures vs Functions:\n");

    println!("  CLOSURES:");
    println!("    - Can capture environment (borrow/move variables)");
    println!("    - Anonymous (no name required)");
    println!("    - Can be stored in variables");
    println!("    - Can have parameters (optional)");
    println!("    - Syntax: |x, y| x + y\n");

    println!("  FUNCTIONS:");
    println!("    - Cannot capture environment");
    println!("    - Must have a name");
    println!("    - Called directly by name");
    println!("    - Must specify parameter types");
    println!("    - Syntax: fn name(x: i32, y: i32) -> i32\n");

    println!("Why store closures in variables?\n");
    println!("  1. Pass as arguments (callbacks, strategies)");
    println!("  2. Store in data structures (event handlers)");
    println!("  3. Reuse complex logic (avoid duplication)");
    println!("  4. Delay execution (lazy evaluation)");
    println!("  5. Capture environment (carry data with code)");
    println!("  6. Polymorphic behavior (different closures, same interface)");

    println!("\nClosure syntax variations:\n");
    println!("  |x| x + 1                    // Inferred types");
    println!("  |x: i32| -> i32 {{ x + 1 }}    // Explicit types");
    println!("  || println!(\"Hi\")            // No parameters");
    println!("  |x, y| {{ x + y }}             // Multiple lines");
}
