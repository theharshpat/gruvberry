// Example 19: Struct Methods & impl Blocks
// Run with: cargo run --example 19_struct_methods_impl

fn main() {
    println!("=== Struct Methods & impl Blocks ===\n");

    println!("impl blocks let you attach methods to structs.\n");
    println!("{}\n", "=".repeat(60));

    // ========== PART 1: Basic struct ==========
    println!("Part 1: Defining structs\n");

    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Rectangle {{ width: {}, height: {} }}\n", rect.width, rect.height);

    println!("{}\n", "=".repeat(60));

    // ========== PART 2: Methods with impl ==========
    println!("Part 2: Adding methods with impl\n");

    struct Circle {
        radius: f64,
    }

    impl Circle {
        // Method - takes &self
        fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }

        // Another method
        fn circumference(&self) -> f64 {
            2.0 * std::f64::consts::PI * self.radius
        }
    }

    let circle = Circle { radius: 5.0 };
    println!("Circle with radius {}", circle.radius);
    println!("  Area: {:.2}", circle.area());
    println!("  Circumference: {:.2}\n", circle.circumference());

    println!("💡 Methods are called with dot notation: circle.area()\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 3: self, &self, &mut self ==========
    println!("Part 3: Understanding self, &self, &mut self\n");

    struct Counter {
        count: i32,
    }

    impl Counter {
        // &self - borrows immutably (read-only)
        fn get_count(&self) -> i32 {
            self.count
        }

        // &mut self - borrows mutably (can modify)
        fn increment(&mut self) {
            self.count += 1;
        }

        // self - takes ownership (consumes the struct)
        fn consume(self) -> i32 {
            println!("  Consuming counter...");
            self.count
        }
    }

    let mut counter = Counter { count: 0 };

    println!("Initial count: {}", counter.get_count());
    counter.increment();
    counter.increment();
    println!("After 2 increments: {}", counter.get_count());

    let final_count = counter.consume();
    println!("Final count: {}", final_count);
    // println!("{}", counter.count);  // ❌ ERROR: counter was consumed!

    println!("\n┌─────────────────────────────────────────────────────┐");
    println!("│ Method type   │ Usage                               │");
    println!("├─────────────────────────────────────────────────────┤");
    println!("│ &self         │ Read data (no modification)         │");
    println!("│ &mut self     │ Modify data (most common)           │");
    println!("│ self          │ Consume/transform (rare)            │");
    println!("└─────────────────────────────────────────────────────┘\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 4: Associated Functions (no self) ==========
    println!("Part 4: Associated functions (constructors)\n");

    struct Point {
        x: f64,
        y: f64,
    }

    impl Point {
        // Associated function - no self parameter
        // Called with Point::new(), not point.new()
        fn new(x: f64, y: f64) -> Point {
            Point { x, y }
        }

        // Another constructor
        fn origin() -> Point {
            Point { x: 0.0, y: 0.0 }
        }

        // Regular method
        fn distance_from_origin(&self) -> f64 {
            (self.x * self.x + self.y * self.y).sqrt()
        }
    }

    let p1 = Point::new(3.0, 4.0);
    let p2 = Point::origin();

    println!("p1 = Point::new(3.0, 4.0)");
    println!("  Distance from origin: {:.2}", p1.distance_from_origin());

    println!("\np2 = Point::origin()");
    println!("  Distance from origin: {:.2}\n", p2.distance_from_origin());

    println!("💡 Associated functions are called with :: (like Point::new)");
    println!("💡 Methods are called with . (like point.distance())\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 5: Multiple impl blocks ==========
    println!("Part 5: Multiple impl blocks (for organization)\n");

    struct Person {
        name: String,
        age: u32,
    }

    // First impl block - constructors
    impl Person {
        fn new(name: String, age: u32) -> Person {
            Person { name, age }
        }
    }

    // Second impl block - getters
    impl Person {
        fn name(&self) -> &str {
            &self.name
        }

        fn age(&self) -> u32 {
            self.age
        }
    }

    // Third impl block - actions
    impl Person {
        fn have_birthday(&mut self) {
            self.age += 1;
            println!("  🎂 Happy birthday! Now {} years old", self.age);
        }
    }

    let mut person = Person::new("Alice".to_string(), 30);
    println!("{} is {} years old", person.name(), person.age());
    person.have_birthday();

    println!("\n💡 You can split methods across multiple impl blocks\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 6: Real-world example from gruvberry ==========
    println!("Part 6: Real-world example - SampleCapture from gruvberry\n");

    use std::sync::{Arc, Mutex};

    #[allow(dead_code)]
    struct SampleCapture<I> {
        source: I,
        buffer: Arc<Mutex<Vec<f32>>>,
        sample_rate: u32,
    }

    #[allow(dead_code)]
    impl<I> SampleCapture<I> {
        // Associated function (constructor)
        fn new(source: I, sample_rate: u32) -> (Self, Arc<Mutex<Vec<f32>>>) {
            let buffer = Arc::new(Mutex::new(Vec::new()));
            let capture = SampleCapture {
                source,
                buffer: buffer.clone(),
                sample_rate,
            };
            (capture, buffer)
        }

        // Method to get sample rate
        fn get_sample_rate(&self) -> u32 {
            self.sample_rate
        }
    }

    println!("  struct SampleCapture<I> {{");
    println!("      source: I,");
    println!("      buffer: Arc<Mutex<Vec<f32>>>,");
    println!("      sample_rate: u32,");
    println!("  }}\n");

    println!("  impl<I> SampleCapture<I> {{");
    println!("      fn new(source: I, sample_rate: u32) -> (Self, Arc<Mutex<Vec<f32>>>) {{");
    println!("          // Constructor logic");
    println!("      }}");
    println!("  }}\n");

    println!("Usage:");
    println!("  let (capture, buffer) = SampleCapture::new(source, 44100);\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 7: Builder pattern ==========
    println!("Part 7: Builder pattern (method chaining)\n");

    struct Car {
        brand: String,
        color: String,
        doors: u8,
    }

    impl Car {
        fn new() -> Car {
            Car {
                brand: String::new(),
                color: String::from("white"),
                doors: 4,
            }
        }

        // Methods that return self for chaining
        fn brand(mut self, brand: &str) -> Self {
            self.brand = brand.to_string();
            self
        }

        fn color(mut self, color: &str) -> Self {
            self.color = color.to_string();
            self
        }

        fn doors(mut self, doors: u8) -> Self {
            self.doors = doors;
            self
        }

        fn build(&self) {
            println!("  Built: {} {} with {} doors",
                     self.color, self.brand, self.doors);
        }
    }

    let car = Car::new()
        .brand("Toyota")
        .color("red")
        .doors(2);

    car.build();

    println!("\n💡 Builder pattern: methods return self for chaining\n");

    println!("{}\n", "=".repeat(60));

    // ========== SUMMARY ==========
    println!("💡 SUMMARY\n");

    println!("Defining structs:\n");
    println!("  struct Name {{");
    println!("      field1: Type1,");
    println!("      field2: Type2,");
    println!("  }}\n");

    println!("Adding methods:\n");
    println!("  impl Name {{");
    println!("      fn method(&self) {{  }}          // Read-only");
    println!("      fn method_mut(&mut self) {{  }}  // Can modify");
    println!("      fn consume(self) {{  }}          // Takes ownership");
    println!("      fn new() -> Self {{  }}          // Associated function");
    println!("  }}\n");

    println!("Calling methods:\n");
    println!("  let obj = Name::new();        // Associated function (::)");
    println!("  obj.method();                 // Method (.)");
    println!("  obj.method_mut();             // Mutable method\n");

    println!("Method receiver types:\n");
    println!("  ┌──────────┬─────────────────────────────────────┐");
    println!("  │ Type     │ When to use                         │");
    println!("  ├──────────┼─────────────────────────────────────┤");
    println!("  │ &self    │ Just reading data (most common)     │");
    println!("  │ &mut self│ Modifying data (second most common) │");
    println!("  │ self     │ Consuming/transforming (rare)       │");
    println!("  └──────────┴─────────────────────────────────────┘\n");

    println!("Common patterns:\n");
    println!("  1. new() constructor - associated function");
    println!("  2. Getters - methods with &self");
    println!("  3. Setters - methods with &mut self");
    println!("  4. Builders - methods returning self for chaining");
    println!("  5. Multiple impl blocks - organize related methods\n");

    println!("Why use impl blocks?");
    println!("  ✅ Organize related functionality");
    println!("  ✅ Method syntax (obj.method() vs function(obj))");
    println!("  ✅ Encapsulation (can add private methods)");
    println!("  ✅ Trait implementations (next example!)");
}
