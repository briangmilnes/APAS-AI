# Rust Interface Patterns for Functional Modules

This document explains different patterns for creating interfaces on functional modules in Rust.

## Pattern 1: Zero-Sized Type (ZST) with Traits
**File**: `FunctionalInterface.rs`

This is the most idiomatic Rust pattern for functional modules:

```rust
pub trait Calculator {
    fn add(&self, a: u32, b: u32) -> u32;
    fn multiply(&self, a: u32, b: u32) -> u32;
    fn identity() -> u32;
}

pub struct BasicCalc;  // Zero-sized type

impl Calculator for BasicCalc {
    fn add(&self, a: u32, b: u32) -> u32 { a + b }
    // ...
}
```

### Advantages:
- **Zero runtime cost**: ZST compiles away completely
- **Multiple implementations**: Easy to swap implementations
- **Type safety**: Compile-time checking
- **Generic programming**: Works with trait bounds

### Use when:
- You want module-level "namespaces" of functions
- You need swappable implementations
- You want static dispatch (monomorphization)

## Pattern 2: Trait Objects (Dynamic Dispatch)
**File**: `ModuleInterface.rs`

Uses `dyn Trait` for runtime polymorphism:

```rust
pub fn process_data(processor: &dyn DataProcessor, data: &str) -> Result<String, &'static str> {
    processor.process(data)
}
```

### Advantages:
- **Runtime flexibility**: Choose implementation at runtime
- **Type erasure**: Different implementations can be stored together
- **Plugin-like architecture**: Easy to extend

### Use when:
- Implementation choice happens at runtime
- You need to store different implementations in a collection
- Binary size is more important than speed

## Pattern 3: Blanket Implementation
**File**: `TypeLessTraitWithImpl.rs` (your original)

Implements a trait for all types:

```rust
impl<T> MyTrait for T {
    fn method(&self) { ... }
}
```

### Advantages:
- Methods available on all types
- Can add "extension" functionality

### Cautions:
- **Orphan rule conflicts**: Can't implement foreign traits for foreign types
- **Coherence issues**: May conflict with other implementations
- **Usually avoid**: Unless creating extension traits for specific use cases

## Comparison: Your Original Experiments

### TypeLessTrait.rs
❌ **Issue**: Trait defined but never implemented; free functions unconnected

### TypeLessTraitWithImpl.rs
⚠️ **Issue**: Blanket impl for all `<T>` is too broad and can cause conflicts

### TypeLessTraitWithImplU32.rs
✅ **Better**: Specific implementation for concrete type

## Recommended Pattern for Functional Modules

```rust
// 1. Define the interface
pub trait ModuleInterface {
    fn operation1(&self, x: u32) -> u32;
    fn operation2(&self, x: u32, y: u32) -> u32;
}

// 2. Create a ZST implementation
pub struct DefaultImpl;

impl ModuleInterface for DefaultImpl {
    fn operation1(&self, x: u32) -> u32 { x + 1 }
    fn operation2(&self, x: u32, y: u32) -> u32 { x + y }
}

// 3. Provide convenience functions
pub fn operation1(x: u32) -> u32 {
    DefaultImpl.operation1(x)
}

// 4. Generic functions for flexibility
pub fn compute_with<M: ModuleInterface>(impl: &M, x: u32) -> u32 {
    impl.operation1(x) + impl.operation2(x, x)
}
```

## Testing Your Code

Run the tests to see the patterns in action:
```bash
cargo test
```

## Further Reading

- [Rust Book: Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [Rust Book: Advanced Traits](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html)
- [Zero-Sized Types](https://doc.rust-lang.org/nomicon/exotic-sizes.html#zero-sized-types-zsts)



