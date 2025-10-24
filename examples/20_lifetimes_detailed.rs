// Example 20: Lifetimes (Detailed Explanation)
// Run with: cargo run --example 20_lifetimes_detailed

fn main() {
    println!("=== Lifetimes - The Complete Guide ===\n");

    println!("Lifetimes are Rust's way of ensuring references are valid.\n");
    println!("{}\n", "=".repeat(60));

    // ========== PART 1: The Problem ==========
    println!("Part 1: THE PROBLEM - Why do we need lifetimes?\n");

    println!("❌ This is what lifetimes PREVENT:\n");
    println!("  {{");
    println!("      let r;                    // Declare reference");
    println!("      {{");
    println!("          let x = 5;            // x created");
    println!("          r = &x;               // r points to x");
    println!("      }}                        // x destroyed here!");
    println!("      println!(\"{{}}\", r);        // ❌ DANGLING REFERENCE!");
    println!("  }}\n");

    println!("The problem: r points to memory that was freed");
    println!("This would cause: use-after-free bug (crashes, security issues)\n");

    println!("✅ Rust's solution: LIFETIMES");
    println!("The compiler tracks how long each reference lives\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 2: What are lifetimes? ==========
    println!("Part 2: What ARE lifetimes?\n");

    println!("A lifetime is the SCOPE during which a reference is valid.\n");

    println!("Example:");
    println!("  {{                      // ← Scope starts");
    println!("      let x = 5;          // x's lifetime begins");
    println!("      let r = &x;         // r borrows x");
    println!("      println!(\"{{}}\", r);  // r is valid here ✅");
    println!("  }}                      // ← x's lifetime ends, r becomes invalid\n");

    println!("💡 Lifetime = how long a value is valid in memory\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 3: Lifetime annotations ==========
    println!("Part 3: Lifetime annotations (the 'a syntax)\n");

    println!("When do you need to write lifetimes?\n");
    println!("When the compiler can't figure out how long references should live.\n");

    println!("Example: Which reference should we return?\n");
    println!("  fn longest(x: &str, y: &str) -> &str {{  // ❌ ERROR!");
    println!("      if x.len() > y.len() {{ x }} else {{ y }}");
    println!("  }}\n");

    println!("ERROR: compiler doesn't know if return value refers to x or y\n");

    println!("✅ Solution: Add lifetime annotations:\n");
    println!("  fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {{");
    println!("      if x.len() > y.len() {{ x }} else {{ y }}");
    println!("  }}\n");

    println!("Reading the signature:");
    println!("  <'a>           = declares a lifetime parameter named 'a");
    println!("  x: &'a str     = x is a reference valid for lifetime 'a");
    println!("  y: &'a str     = y is a reference valid for lifetime 'a");
    println!("  -> &'a str     = return value valid for lifetime 'a\n");

    println!("💡 'a means: \"all these references live at least as long as 'a\"\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 4: Concrete example ==========
    println!("Part 4: Concrete example with lifetime annotations\n");

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("  The longest string is: {}", result);
    }  // string2 dropped here

    println!("\n  This works because:");
    println!("  - Both string1 and string2 are valid when longest() is called");
    println!("  - result is used before string2 is dropped");
    println!("  - Lifetime 'a is the SMALLER of string1 and string2's lifetimes\n");

    println!("❌ This would NOT compile:\n");
    println!("  let string1 = String::from(\"long\");");
    println!("  let result;");
    println!("  {{");
    println!("      let string2 = String::from(\"xyz\");");
    println!("      result = longest(&string1, &string2);");
    println!("  }}  // string2 dropped");
    println!("  println!(\"{{}}\", result);  // ERROR! string2 might be returned\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 5: Lifetime elision ==========
    println!("Part 5: Lifetime elision (when you can skip 'a)\n");

    println!("Rust has RULES that let you skip lifetime annotations in common cases:\n");

    println!("Rule 1: Each input reference gets its own lifetime\n");
    println!("  fn foo(x: &str) -> &str          // Compiler infers");
    println!("  fn foo<'a>(x: &'a str) -> &'a str  // What it means\n");

    println!("Rule 2: If one input, output gets same lifetime\n");
    println!("  fn first_word(s: &str) -> &str   // No annotation needed!");
    println!("  // Compiler knows output lifetime = s's lifetime\n");

    println!("Rule 3: If &self, output gets self's lifetime\n");
    println!("  impl Thing {{");
    println!("      fn get_name(&self) -> &str   // No annotation needed!");
    println!("      // Compiler knows return value tied to self");
    println!("  }}\n");

    println!("💡 Most of the time, you DON'T need to write lifetimes!\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 6: Lifetimes in structs ==========
    println!("Part 6: Lifetimes in structs\n");

    println!("When a struct holds references, you MUST annotate lifetimes:\n");

    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();

    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };

    println!("  struct ImportantExcerpt<'a> {{");
    println!("      part: &'a str,");
    println!("  }}\n");

    println!("  excerpt.part = \"{}\"\n", excerpt.part);

    println!("This tells Rust:");
    println!("  - ImportantExcerpt can't outlive the reference it holds");
    println!("  - part is valid for lifetime 'a");
    println!("  - ImportantExcerpt instance is also valid for 'a\n");

    println!("❌ This would NOT compile:");
    println!("  let excerpt;");
    println!("  {{");
    println!("      let novel = String::from(\"...\");");
    println!("      excerpt = ImportantExcerpt {{ part: &novel }};");
    println!("  }}  // novel dropped, but excerpt.part still references it!\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 7: Multiple lifetimes ==========
    println!("Part 7: Multiple lifetime parameters\n");

    println!("Sometimes you need DIFFERENT lifetimes:\n");

    fn announce_and_return<'a, 'b>(announcement: &'a str, _value: &'b str) -> &'a str {
        println!("  Announcement: {}", announcement);
        announcement  // Only returns first parameter
    }

    let ann = String::from("Important!");
    let val = String::from("xyz");
    let result = announce_and_return(&ann, &val);
    println!("  Returned: {}\n", result);

    println!("  fn announce_and_return<'a, 'b>(");
    println!("      announcement: &'a str,");
    println!("      value: &'b str,");
    println!("  ) -> &'a str\n");

    println!("Why two lifetimes?");
    println!("  - Return value is tied to 'a (announcement)");
    println!("  - value has independent lifetime 'b");
    println!("  - Allows value to be dropped earlier than announcement\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 8: Static lifetime ==========
    println!("Part 8: The 'static lifetime\n");

    println!("'static = lives for the ENTIRE program duration\n");

    let _s: &'static str = "I live forever";
    println!("  let s: &'static str = \"I live forever\"\n");

    println!("Where 'static is used:");
    println!("  1. String literals (\"hello\") - stored in program binary");
    println!("  2. Global constants");
    println!("  3. Leaked memory (Box::leak)\n");

    println!("⚠️  Don't use 'static unless you really need it!\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 9: Lifetime bounds on generics ==========
    println!("Part 9: Lifetime bounds on generics\n");

    println!("Generic types can have lifetime requirements:\n");

    println!("  fn print_ref<'a, T>(t: &'a T)");
    println!("  where");
    println!("      T: std::fmt::Display + 'a  // T must live at least 'a");
    println!("  {{");
    println!("      println!(\"{{}}\", t);");
    println!("  }}\n");

    println!("The 'a bound means:");
    println!("  - If T contains references, they must live at least 'a");
    println!("  - Ensures T is valid for the entire lifetime 'a\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 10: Common lifetime patterns ==========
    println!("Part 10: Common lifetime patterns\n");

    println!("Pattern 1: Returning a reference to input\n");
    fn first_word<'a>(s: &'a str) -> &'a str {
        s.split_whitespace().next().unwrap_or("")
    }
    let sentence = "hello world";
    let word = first_word(sentence);
    println!("  first_word(\"{}\") = \"{}\"\n", sentence, word);

    println!("Pattern 2: Struct with reference\n");
    struct Parser<'a> {
        data: &'a str,
    }
    impl<'a> Parser<'a> {
        fn new(data: &'a str) -> Self {
            Parser { data }
        }
    }
    let data = "parse this";
    let parser = Parser::new(data);
    println!("  Parser {{ data: \"{}\" }}\n", parser.data);

    println!("Pattern 3: Iterator that borrows\n");
    println!("  struct Lines<'a> {{");
    println!("      data: &'a str,");
    println!("  }}\n");
    println!("  impl<'a> Iterator for Lines<'a> {{");
    println!("      type Item = &'a str;");
    println!("      // ...");
    println!("  }}\n");

    println!("{}\n", "=".repeat(60));

    // ========== PART 11: Real-world from gruvberry ==========
    println!("Part 11: Real-world example from gruvberry\n");

    println!("SampleCapture uses generic I, which might have lifetimes:\n");
    println!("  impl<I> Iterator for SampleCapture<I>");
    println!("  where");
    println!("      I: Source<Item = f32>,  // I might contain references");
    println!("  {{");
    println!("      type Item = f32;");
    println!("      fn next(&mut self) -> Option<Self::Item> {{");
    println!("          self.source.next()");
    println!("      }}");
    println!("  }}\n");

    println!("Why no explicit lifetime?");
    println!("  - Lifetime elision rules apply");
    println!("  - &mut self has implicit lifetime");
    println!("  - Option<f32> doesn't contain references\n");

    println!("{}\n", "=".repeat(60));

    // ========== SUMMARY ==========
    println!("💡 SUMMARY\n");

    println!("What are lifetimes?");
    println!("  - Annotations that describe how long references are valid");
    println!("  - Prevent dangling references (use-after-free)");
    println!("  - Checked at COMPILE TIME (zero runtime cost)\n");

    println!("Lifetime syntax:");
    println!("  'a, 'b, 'static     // Lifetime names");
    println!("  &'a str            // Reference valid for 'a");
    println!("  <'a>               // Declare lifetime parameter\n");

    println!("When to write lifetimes:");
    println!("  ✅ Functions with multiple references");
    println!("  ✅ Structs that hold references");
    println!("  ✅ When compiler can't infer");
    println!("  ❌ Most other times (elision rules handle it)\n");

    println!("Elision rules (when you can skip):");
    println!("  1. Each input reference gets own lifetime");
    println!("  2. If one input, output gets same lifetime");
    println!("  3. If &self, output gets self's lifetime\n");

    println!("Common patterns:");
    println!("  fn foo<'a>(x: &'a str) -> &'a str    // Return input ref");
    println!("  struct S<'a> {{ field: &'a str }}     // Struct with ref");
    println!("  impl<'a> S<'a> {{ }}                 // Impl for struct with lifetime\n");

    println!("Special lifetimes:");
    println!("  'static    = lives entire program");
    println!("  '_         = anonymous lifetime (compiler infers)\n");

    println!("Mental model:");
    println!("  Think of 'a as: \"this reference must be valid for AT LEAST this long\"");
    println!("  Rust picks the SHORTEST lifetime that satisfies all constraints\n");

    println!("Key insight:");
    println!("  Lifetimes DON'T change how long values live");
    println!("  They DESCRIBE relationships between references");
    println!("  They're for the COMPILER, not runtime");
}
