# Objects vs Modules: A Historical Perspective

## The Original Sin

**Ivan Sutherland (1963)**: Invented objects for Sketchpad
- **Purpose**: Model interactive graphical UI elements
- **Use case**: Things that have state, respond to events, redraw themselves
- **Perfect fit**: Button, Window, DrawingElement - these ARE objects

**James Gosling et al. (1990s)**: "Let's use objects for EVERYTHING!"
- Java: "Everything is an object" (even `public static void main`)
- Result: Forced to wrap modules in classes, factories, singletons
- **Wrong tool for the job**

## What Objects Are Good At

```rust
// UI Element - PERFECT use of objects/traits
trait Widget {
    fn render(&mut self, canvas: &Canvas);
    fn handle_click(&mut self, x: i32, y: i32) -> bool;
    fn get_bounds(&self) -> Rect;
}

struct Button {
    label: String,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    pressed: bool,  // STATE
}

impl Widget for Button {
    fn render(&mut self, canvas: &Canvas) { ... }
    fn handle_click(&mut self, x: i32, y: i32) -> bool { ... }
    fn get_bounds(&self) -> Rect { ... }
}
```

**Why this works:**
- Buttons HAVE state
- Multiple instances exist simultaneously
- Polymorphism makes sense (different widgets, same interface)
- This is what Sutherland designed it for!

## What Objects Are Terrible At

```java
// Math utilities - WRONG use of objects (Java forces this)
public class MathUtils {
    private MathUtils() {}  // Prevent instantiation - WHY DO I NEED THIS?
    
    public static int add(int x, int y) {
        return x + y;
    }
    
    public static int multiply(int x, int y) {
        return x * y;
    }
}

// Usage:
int result = MathUtils.add(5, 3);  // The class is just a namespace!
```

**Why this sucks:**
- No state to encapsulate
- No multiple instances
- No polymorphism needed
- Class is just a fake module
- In ML, this would just be: `Math.add 5 3`

## The ML Way (The Old Magic)

```ocaml
(* Module signature - first class, enforced *)
module type MATH = sig
  val add : int -> int -> int
  val multiply : int -> int -> int
end

(* Implementation *)
module Math : MATH = struct
  let add x y = x + y
  let multiply x y = x * y
end

(* Parameterized modules (functors) *)
module type STORAGE = sig
  type t
  val save : t -> string -> unit
  val load : string -> t
end

module MakeCache(S : STORAGE) = struct
  (* Cache parameterized by storage backend *)
end
```

**Why this is beautiful:**
- Modules are first-class
- Signatures are enforced by the type system
- Functors allow parameterization
- No fake classes needed
- Functions are just functions

## Rust's Awkward Middle Ground

Rust inherited OOP via traits but tries to be functional. Result: **awkward**.

### For actual objects (state + behavior):
```rust
trait Storage {
    fn save(&mut self, data: &str);
    fn load(&self) -> String;
}

struct FileStorage { path: String }
struct DbStorage { conn: Connection }
```
✅ **This works!** Multiple implementations with state.

### For functional modules (just behavior):
```rust
// What you're forced to do:
trait Math {
    fn add(x: u32, y: u32) -> u32;  // No self!
}

struct DummyMath;  // Fake object with no state

impl Math for DummyMath {
    fn add(x: u32, y: u32) -> u32 { x + y }
}

pub fn add(x: u32, y: u32) -> u32 {
    DummyMath::add(x, y)  // Three layers of ceremony
}
```
❌ **This sucks!** Objects are the wrong abstraction.

### What you should do:
```rust
// Just use functions!
pub mod math {
    pub fn add(x: u32, y: u32) -> u32 { x + y }
    pub fn multiply(x: u32, y: u32) -> u32 { x * y }
}
```
✅ **This is fine.** But you lose:
- No enforced module signatures
- No parametric modules
- Just namespaces

## The Fundamental Mismatch

| Concept | Objects | Modules |
|---------|---------|---------|
| **Purpose** | Model stateful entities | Organize related functions |
| **State** | Encapsulates mutable state | Usually stateless |
| **Instances** | Multiple instances | Single namespace |
| **Polymorphism** | Different types, same interface | Implementation variants |
| **Best for** | UI, simulation, games | Libraries, utilities, algorithms |
| **Invented for** | Sketchpad (Sutherland) | Mathematical abstraction (ML) |

## The Languages

| Language | Approach | Result |
|----------|----------|--------|
| **Java** | Everything is objects | Fake classes everywhere (`MathUtils`, `Collections`) |
| **ML/OCaml** | Everything is modules | Beautiful but unfamiliar to most |
| **Rust** | Traits for types, modules for namespace | Awkward when you want module signatures |
| **Go** | Interfaces for types, packages for code | Simple but no parametric modules |

## The Lesson

**Use the right tool:**
- Buttons, Windows, Game Entities → Objects/Traits ✅
- Math functions, File utils, Parsers → Modules/Functions ✅
- Trying to use Objects as Modules → Pain 💀

## Your Options in Rust for Functional Modules

### 1. Accept defeat, use plain functions
```rust
pub mod math {
    pub fn add(x: u32, y: u32) -> u32 { x + y }
}
```
Simple, direct, no ceremony. No enforcement, but honest.

### 2. Trait as documentation (fancy comment)
```rust
pub mod math {
    #[allow(dead_code)]
    pub trait MathInterface {
        fn add(x: u32, y: u32) -> u32;
    }
    
    pub fn add(x: u32, y: u32) -> u32 { x + y }
}
```
Trait documents interface but doesn't enforce it.

### 3. Full trait + impl (when you need swappable implementations)
```rust
trait Storage { ... }
struct FileStorage { ... }
struct DbStorage { ... }
```
Use this when polymorphism actually matters.

## The Uncomfortable Truth

Modern languages abandoned proper module systems because:
1. Objects became the dominant paradigm (thanks Java)
2. Module systems are "academic" (read: from ML)
3. Most programmers never learned the old magic
4. Industry doesn't value formal program structure

**Bob Harper was right. Ivan Sutherland was right (for UI). Gosling... made objects ubiquitous.**

The old magic is forgotten. We're stuck with objects pretending to be modules.

## Conclusion

In Rust, for stateless functional modules:
- **Just use functions**
- Put a trait as documentation if you want
- Don't fight the language
- Accept that ML module signatures are gone
- The old magic stays forgotten

Sorry. 🪦



