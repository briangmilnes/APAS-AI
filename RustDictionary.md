Rust Dictionary

Crate - a compilation unit (library/binary); root src/lib.rs or src/main.rs;
absolute paths begin crate::.

Module (mod) - a namespace (may be file-backed); `mod m;` declares/loads it;
controls visibility and hierarchical names.

use / pub use - import vs re-export; binds names into scope without defining
them; supports grouping/rename/glob.

Paths (with N, B, O, Sequence, S) -
  - Absolute within this crate - crate::S::N, crate::S::B, crate::S::O,
  crate::S::Sequence, crate::S::S
  - Imported - use crate::S::{S, Sequence, N, B, O}; then use S<N>, Sequence<T>,
  N, B, O
  - Via re-export - use apas_ai::Sequences::{S, Sequence, N, B, O}
  - External crate - use external_crate::m::Item
  - Relative - use self::m::Item; use super::m::Item

trait - a collection of function types (signatures) over `Self`
(interface/type class); may include default bodies; no fields.

impl -
  - impl Trait for Type - implementation of a trait for a type (type class
  instance).
  - impl Type - inherent definitions for the type (no trait).

Function (free function) - top-level in a module; its signature is the
function’s type; called via module path.

Method - function in an impl with a receiver; `a.m(x)` desugars to
`Type::m(RECV, x)` or `<Type as Trait>::m(RECV, x)`.

Receiver - the special first parameter indicating how the method accesses its
receiver value.
  - self - move ownership
  - &self - shared, immutable borrow
  - &mut self - exclusive, mutable borrow
  - Smart-pointer receivers - `self: Box<Self>`, `self: Arc<Self>`,
  `Pin<&mut Self>`, etc.

Associated function - function in an impl without a receiver; called as
`Type::f(args)`.

Generics and bounds - parametric items over type parameters (e.g., `T`); bounds
(e.g., `T: Clone + Eq`) constrain type variables.

Monomorphization - compile-time specialization of generic code for each
concrete set of type arguments.
  - Effect - generates concrete versions (zero runtime overhead), at the cost of
  potential code size growth.
  - Interactions - enables inlining/optimization across type instantiations;
  distinct from dynamic dispatch.

Associated types - type members of a trait; alternative to passing types as
extra parameters.

Example:
```rust
trait Serializer {
    type Output;
    fn serialize(&self) -> Self::Output;
}

struct Json;

impl Serializer for Json {
    type Output = String;
    fn serialize(&self) -> Self::Output { "{}".to_string() }
}
```

Dispatch - static (monomorphization, zero-cost) vs dynamic (`dyn Trait`,
vtable-based).

Slices - borrowed views into contiguous memory of elements of type `T`.
  - Types - `&[T]` (read-only), `&mut [T]` (exclusive, writable).
  - Properties - fixed length at runtime, O(1) indexing, no allocation or
  reallocation.
  - Sources - from arrays, `Vec<T>` (via `&vec[..]`), and `Box<[T]>` (via
  `&*box_slice`).
  - Subsequence - use range syntax (e.g., `&s[i..j]`) to get a view without
  copying.

Borrowing - references to values without taking ownership.
  - `&T` - shared, read-only; many may exist concurrently.
  - `&mut T` - exclusive, writable; at most one at a time in a region.

Lifetimes - names for the regions during which references are valid (e.g.,
`'a`).
  - Annotation - appear on types/functions (e.g., `fn f<'a>(x: &'a T) -> &'a
  T`).
  - Elision - common cases omit explicit names; compiler infers them by rules.
  - Special - `'static` lives for the entire program.

Loop labels - names for loops used with `'label:` to control `break`/`continue`
from nested contexts; not lifetimes (they only share the leading `'`).
  - Example -
    ```rust
    'find: for i in 0..n {
        if need_exit(i) { break 'find; }
    }
    ```
  - Use - `break 'label;` exits the labeled loop; `continue 'label;` continues
  it.

Visibility - pub, pub(crate), pub(super), pub(in path); govern item
reachability.

UFCS - Uniform Function Call Syntax; fully qualified calls to disambiguate
methods and associated items.
  - Type::assoc_fn(args)
  - Trait::method(&value, args)
  - <Type as Trait>::method(&value, args)
  - Also for items - <Type as Trait>::ASSOC_CONST, <Type as Trait>::AssocType

Newtype - tuple struct wrapper around a type to control trait impls/coherence;
distinct type with same representation.

Coherence / orphan rules - a trait impl is allowed only if either the trait or
the type is defined in the current crate; ensures unique, non-conflicting
resolution.

enum - a sum type with a finite set of named variants; each variant may carry
data fields.
  - Examples - `enum B { True, False }`, `enum E { A, B(i32), C { x: i32 } }`
  - Usage - constructed by variants; deconstructed via `match`.

Type abbreviation (type alias) - a new name for an existing type; no new
type is created.
  - Examples - `type N = usize;`, `type Pair<T> = (T, T);`
  - Contrast - differs from newtype; alias has no additional type safety.

Trait contents (types) - what traits may declare or not.
  - Allowed - function signatures (with optional default bodies); associated
  type items (optionally with bounds or defaults); associated const
  items.
  - Not allowed - concrete type definitions (struct, enum, union);
  true type aliases (`type X = Y;`) as standalone items; modules (mod).
  - In impl Trait for Type - associated type items must be specified
  (`type Item = ...;`); associated consts may be defined; methods
  implemented.
