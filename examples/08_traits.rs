// Example 8: Traits
// Run with: cargo run --example 08_traits

// Define a trait (like an interface)
trait Animal {
    fn make_sound(&self) -> String;
    fn name(&self) -> String;
}

struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

// Implement trait for Dog
impl Animal for Dog {
    fn make_sound(&self) -> String {
        "Woof!".to_string()
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

// Implement trait for Cat
impl Animal for Cat {
    fn make_sound(&self) -> String {
        "Meow!".to_string()
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

// Function that works with ANY type implementing Animal
fn introduce_animal(animal: &impl Animal) {
    println!("  {} says: {}", animal.name(), animal.make_sound());
}

// Alternative syntax (trait bound)
fn introduce_animal_v2<T: Animal>(animal: &T) {
    println!("  {} says: {}", animal.name(), animal.make_sound());
}

fn main() {
    println!("=== Traits Example ===\n");

    let dog = Dog { name: "Buddy".to_string() };
    let cat = Cat { name: "Whiskers".to_string() };

    println!("1. Calling trait methods:");
    println!("  {}: {}", dog.name(), dog.make_sound());
    println!("  {}: {}\n", cat.name(), cat.make_sound());

    println!("2. Generic function with trait:");
    introduce_animal(&dog);
    introduce_animal(&cat);

    println!("\n3. Trait objects (dynamic dispatch):");
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog { name: "Max".to_string() }),
        Box::new(Cat { name: "Luna".to_string() }),
    ];

    for animal in &animals {
        println!("  {} says: {}", animal.name(), animal.make_sound());
    }

    println!("\n💡 Key concepts:");
    println!("  - Trait = shared behavior (like interface)");
    println!("  - impl Trait for Type = implement trait");
    println!("  - &impl Trait = static dispatch (compile-time)");
    println!("  - &dyn Trait = dynamic dispatch (runtime)");
    println!("  - Traits enable polymorphism in Rust");
}
