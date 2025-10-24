// Example 22: Trait Objects & dyn
// Run with: cargo run --example 22_trait_objects_dyn

fn main() {
    println!("=== Trait Objects & dyn ===\n");

    println!("Trait objects allow polymorphism - storing different types together.\n");
    println!("{}\n", "=".repeat(60));

    // ========== PART 1: The problem ==========
    println!("Part 1: THE PROBLEM - Different types in same collection\n");

    println!("❌ This doesn't work:\n");
    println!("  struct Dog {{}}");
    println!("  struct Cat {{}}");
    println!("  let animals = vec![Dog {{}}, Cat {{}}];");
    println!("  ERROR: Vec can only hold ONE type!\n");

    println!("✅ Solution: Trait objects\n");
    println!("  trait Animal {{ fn speak(&self); }}");
    println!("  impl Animal for Dog {{ ... }}");
    println!("  impl Animal for Cat {{ ... }}");
    println!("  let animals: Vec<Box<dyn Animal>> = vec![");
    println!("      Box::new(Dog {{}}),");
    println!("      Box::new(Cat {{}}),");
    println!("  ];\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 2: Concrete example ==========
    println!("Part 2: Concrete example with trait objects\n");

    trait Animal {
        fn speak(&self) -> &str;
        fn name(&self) -> &str;
    }

    struct Dog {
        name: String,
    }

    struct Cat {
        name: String,
    }

    struct Bird {
        name: String,
    }

    impl Animal for Dog {
        fn speak(&self) -> &str { "Woof!" }
        fn name(&self) -> &str { &self.name }
    }

    impl Animal for Cat {
        fn speak(&self) -> &str { "Meow!" }
        fn name(&self) -> &str { &self.name }
    }

    impl Animal for Bird {
        fn speak(&self) -> &str { "Tweet!" }
        fn name(&self) -> &str { &self.name }
    }

    // Trait object: Box<dyn Animal>
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog { name: "Buddy".to_string() }),
        Box::new(Cat { name: "Whiskers".to_string() }),
        Box::new(Bird { name: "Tweety".to_string() }),
    ];

    println!("  Animals in our collection:");
    for animal in &animals {
        println!("    {} says {}", animal.name(), animal.speak());
    }

    println!("\n💡 All different types, but treated uniformly!\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 3: impl Trait vs dyn Trait ==========
    println!("Part 3: impl Trait vs dyn Trait\n");

    println!("Two ways to use traits as types:\n");

    println!("1. impl Trait (static dispatch)\n");
    println!("  fn foo(x: impl Animal) {{  }}");
    println!("  fn bar() -> impl Animal {{  }}\n");

    println!("  - Compiler knows exact type at compile time");
    println!("  - Generates specific code for each type (monomorphization)");
    println!("  - Faster (no runtime overhead)");
    println!("  - Can only return ONE concrete type\n");

    println!("2. dyn Trait (dynamic dispatch)\n");
    println!("  fn foo(x: &dyn Animal) {{  }}");
    println!("  fn bar() -> Box<dyn Animal> {{  }}\n");

    println!("  - Type determined at runtime");
    println!("  - Uses vtable (virtual table) for method lookups");
    println!("  - Slightly slower (runtime indirection)");
    println!("  - Can return DIFFERENT types\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 4: Static vs Dynamic dispatch ==========
    println!("Part 4: Static vs Dynamic dispatch (detailed)\n");

    // Static dispatch example
    fn print_animal_static(animal: &impl Animal) {
        println!("    {} says {}", animal.name(), animal.speak());
    }

    println!("Static dispatch:");
    let dog = Dog { name: "Max".to_string() };
    let cat = Cat { name: "Luna".to_string() };

    print_animal_static(&dog);
    print_animal_static(&cat);

    println!("\n  Compiler generates TWO functions:");
    println!("    print_animal_static::<Dog>(&dog)");
    println!("    print_animal_static::<Cat>(&cat)");
    println!("  ✅ Fast! Direct function call");
    println!("  ❌ Binary size increases (code for each type)\n");

    // Dynamic dispatch example
    fn print_animal_dynamic(animal: &dyn Animal) {
        println!("    {} says {}", animal.name(), animal.speak());
    }

    println!("Dynamic dispatch:");
    print_animal_dynamic(&dog);
    print_animal_dynamic(&cat);

    println!("\n  Compiler generates ONE function:");
    println!("    print_animal_dynamic(animal)");
    println!("  - Looks up method in vtable at runtime");
    println!("  ✅ Smaller binary");
    println!("  ❌ Slight runtime cost (vtable lookup)\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 5: Size and pointers ==========
    println!("Part 5: Why Box<dyn Trait>?\n");

    println!("❓ Why not just dyn Trait?\n");

    println!("  let animal: dyn Animal = Dog {{ ... }};");
    println!("  ❌ ERROR: dyn Animal doesn't have a size known at compile-time\n");

    println!("The problem:");
    println!("  - Dog, Cat, Bird have DIFFERENT sizes");
    println!("  - dyn Animal could be ANY of them");
    println!("  - Compiler can't allocate stack space (size unknown!)\n");

    println!("Solutions (all involve indirection):\n");
    println!("  1. &dyn Animal        - Reference (borrowed)");
    println!("  2. Box<dyn Animal>    - Box (owned, heap)");
    println!("  3. Rc<dyn Animal>     - Rc (shared, single-thread)");
    println!("  4. Arc<dyn Animal>    - Arc (shared, multi-thread)\n");

    println!("These all have KNOWN size (pointer = 16 bytes)\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 6: Object safety ==========
    println!("Part 6: Object safety (not all traits can be trait objects)\n");

    println!("A trait is 'object safe' if:\n");
    println!("  ✅ No generic methods");
    println!("  ✅ No Self type in return (except &self, &mut self, Box<Self>)");
    println!("  ✅ No associated functions (no self parameter)\n");

    println!("Example of NON-object-safe trait:\n");
    println!("  trait Cloneable {{");
    println!("      fn clone(&self) -> Self;  // ❌ Returns Self");
    println!("  }}\n");
    println!("  let x: Box<dyn Cloneable> = ...;  // ERROR!\n");

    println!("Why?");
    println!("  - Self has unknown size at runtime");
    println!("  - Can't return unknown-sized value\n");

    println!("Object-safe version:\n");
    println!("  trait CloneableObject {{");
    println!("      fn clone_box(&self) -> Box<dyn CloneableObject>;");
    println!("  }}\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 7: Returning different types ==========
    println!("Part 7: Returning different types with trait objects\n");

    fn get_animal(animal_type: &str) -> Box<dyn Animal> {
        match animal_type {
            "dog" => Box::new(Dog { name: "Rex".to_string() }),
            "cat" => Box::new(Cat { name: "Felix".to_string() }),
            _ => Box::new(Bird { name: "Polly".to_string() }),
        }
    }

    println!("  fn get_animal(animal_type: &str) -> Box<dyn Animal> {{");
    println!("      match animal_type {{");
    println!("          \"dog\" => Box::new(Dog {{ ... }}),");
    println!("          \"cat\" => Box::new(Cat {{ ... }}),");
    println!("          _ => Box::new(Bird {{ ... }}),");
    println!("      }}");
    println!("  }}\n");

    let animal1 = get_animal("dog");
    let animal2 = get_animal("cat");
    let animal3 = get_animal("bird");

    println!("  get_animal(\"dog\"):  {} says {}", animal1.name(), animal1.speak());
    println!("  get_animal(\"cat\"):  {} says {}", animal2.name(), animal2.speak());
    println!("  get_animal(\"bird\"): {} says {}", animal3.name(), animal3.speak());

    println!("\n💡 Same function returns DIFFERENT types!\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 8: Real-world usage ==========
    println!("Part 8: Real-world examples\n");

    println!("Example 1: GUI framework\n");
    println!("  trait Widget {{");
    println!("      fn draw(&self);");
    println!("  }}");
    println!("  struct Button {{ ... }}");
    println!("  struct TextBox {{ ... }}");
    println!("  struct Checkbox {{ ... }}\n");
    println!("  let widgets: Vec<Box<dyn Widget>> = vec![");
    println!("      Box::new(Button {{ ... }}),");
    println!("      Box::new(TextBox {{ ... }}),");
    println!("      Box::new(Checkbox {{ ... }}),");
    println!("  ];\n");
    println!("  for widget in widgets {{");
    println!("      widget.draw();");
    println!("  }}\n");

    println!("Example 2: Plugin system\n");
    println!("  trait Plugin {{");
    println!("      fn execute(&self);");
    println!("  }}");
    println!("  let plugins: Vec<Box<dyn Plugin>> = load_plugins();\n");

    println!("Example 3: Error handling\n");
    println!("  Box<dyn Error>  // Any error type!");
    println!("  fn run() -> Result<(), Box<dyn Error>> {{");
    println!("      // Can return different error types");
    println!("  }}\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 9: Performance considerations ==========
    println!("Part 9: Performance considerations\n");

    println!("┌─────────────────────────────────────────────────────────┐");
    println!("│ Aspect         │ impl Trait    │ dyn Trait            │");
    println!("├─────────────────────────────────────────────────────────┤");
    println!("│ Dispatch       │ Static        │ Dynamic (vtable)     │");
    println!("│ Speed          │ Faster        │ Slightly slower      │");
    println!("│ Binary size    │ Larger        │ Smaller              │");
    println!("│ Flexibility    │ Less          │ More                 │");
    println!("│ Return types   │ Single type   │ Different types      │");
    println!("│ Collections    │ Can't mix     │ Can mix types        │");
    println!("└─────────────────────────────────────────────────────────┘\n");

    println!("{}\n", "=".repeat(60));

    // ========== SUMMARY ==========
    println!("💡 SUMMARY\n");

    println!("Trait objects (dyn Trait):");
    println!("  - Allow storing different types implementing same trait");
    println!("  - Must use with pointer: &dyn, Box<dyn>, Rc<dyn>, Arc<dyn>");
    println!("  - Dynamic dispatch (runtime vtable lookup)");
    println!("  - Slightly slower but more flexible\n");

    println!("impl Trait:");
    println!("  - Static dispatch (compile-time)");
    println!("  - Faster (no vtable)");
    println!("  - Can only return one concrete type");
    println!("  - Use in function parameters and return types\n");

    println!("When to use trait objects:");
    println!("  ✅ Need to store different types in collection");
    println!("  ✅ Need to return different types from function");
    println!("  ✅ Plugin systems, GUI frameworks");
    println!("  ✅ Error handling (Box<dyn Error>)\n");

    println!("When to use impl Trait:");
    println!("  ✅ Performance critical code");
    println!("  ✅ Return type is always same concrete type");
    println!("  ✅ Function parameters (cleaner than generics)\n");

    println!("Common patterns:");
    println!("  &dyn Trait           - Borrow trait object");
    println!("  Box<dyn Trait>       - Own trait object");
    println!("  Vec<Box<dyn Trait>>  - Collection of different types");
    println!("  Box<dyn Error>       - Any error type\n");

    println!("Key insight:");
    println!("  Trait objects trade compile-time knowledge for runtime flexibility");
}
