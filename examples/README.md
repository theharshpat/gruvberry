# Rust Concepts Examples

Learn Rust concepts step-by-step with runnable examples!

## How to Run

Each example is a standalone file. Run them with:

```bash
cargo run --example 01_file_io
cargo run --example 02_buffered_io
# ... etc
```

## Examples (in learning order)

### Basics
1. **01_file_io** - File I/O, Result type, ? operator, RAII
2. **02_buffered_io** - BufReader, why buffering matters
3. **03_result_type** - Error handling, Result<T,E>, match vs ?

### Ownership System
4. **04_ownership** - Move semantics, ownership rules
5. **05_borrowing** - References, borrowing rules, & vs &mut

### Concurrency Primitives
6. **06_arc_basics** - Arc (Atomic Reference Counting)
7. **07_mutex** - Mutex (mutual exclusion locks)
8. **11_threading** - Creating and joining threads

### Advanced Types
8. **08_traits** - Trait definition, implementation, polymorphism
9. **09_generics** - Generic functions, structs, trait bounds
10. **10_iterators** - Iterator trait, lazy evaluation, adapters
12. **12_closures** - Anonymous functions, capturing environment
13. **13_move_explained** - Why 'move' is needed in thread::spawn
14. **14_move_vs_borrow** - Move vs borrow in closures, visual comparison
15. **15_closure_capture_modes** - Why closures borrow by default, Fn/FnMut/FnOnce
16. **16_closures_vs_functions** - Closures vs functions, scope, why store them

## Concepts Used in Gruvberry

All these concepts are used in the main `gruvberry` audio visualizer:

- **File I/O** - Reading WAV files
- **Buffering** - Efficient file reading
- **Result** - Error propagation with ?
- **Ownership** - Managing audio data
- **Borrowing** - Passing data without copying
- **Arc<Mutex<>>** - Sharing audio buffer between threads
- **Threading** - Audio playback + visualization in parallel
- **Traits** - Custom Source trait for audio
- **Generics** - SampleCapture works with any source
- **Iterators** - Processing audio samples
- **Closures** - Iterator methods (map, filter)

## Next Steps

After understanding these examples, read through `src/main.rs` and see how all these concepts work together in a real project!
