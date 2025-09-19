# Rust Terms (Programming-Languages Alignment)

This reference crosswalk pairs Rust vocabulary with analogous programming-language
terminology. Use it as a quick translation table when you move between systems
discussions and PL theory notes.

This is a comparison of the technical terms used in Rust against standard programming terminology.

Rust Term                    | Description                                        | Programming Language Term       
---------------------------- | -------------------------------------------------- | --------------------------------
Rust Term                      | Description                                          | Programming Language Term       
------------------------------ | ---------------------------------------------------- | --------------------------------
Ad-hoc Polymorphism            | behavior specialization via traits and impls         | typeclass-style overloading     
Arc<T>                         | atomic reference counting smart pointer              | concurrent shared pointer       
Associated Type                | trait-declared abstract type placeholder             | type family member              
Associated Type                | trait-defined type member                            | type family member              
Blanket Implementation         | impl block applying to many target types at once     | typeclass blanket instance      
Block Scope                    | region controlled by a block for locals              | lexical scope extent            
Borrow                         | temporary access without taking ownership            | alias with affine capability    
Borrow Checker                 | compile-time analysis enforcing borrow rules         | affine type checker             
Box<T>                         | owning pointer allocating on the heap                | owned heap allocation           
Cargo                          | build system and dependency manager                  | package manager toolchain       
Clone Trait                    | explicit duplication via clone()                     | copy constructor interface      
Closure                        | lambda capturing lexical environment                 | Closure                         
Const Generic                  | compile-time constant parameter                      | indexed type parameter          
Copy Trait                     | marker for implicit bitwise copy semantics           | implicit copy qualifier         
Crate                          | primary compilation unit and package                 | top-level module artifact       
Debug Trait                    | formatting for developer diagnostics                 | debug-print interface           
Deref Trait                    | overloads * to follow smart pointers                 | implicit dereference protocol   
Display Trait                  | formatting as user-facing text                       | pretty-print interface          
Drop                           | deterministic cleanup when scope ends                | RAII finalizer                  
Dynamic Dispatch               | runtime dyn Trait method dispatch                    | subtype polymorphism            
Enum                           | tagged union storing one of several variants         | algebraic sum type              
Eq Trait                       | marks PartialEq as total equality                    | equivalence relation marker     
Free Function                  | standalone function not tied to a type or impl       | top-level function definition   
Function Type Signature        | input and output type of a function                  | function type annotation        
Future                         | value representing an asynchronous computation       | promise/future type             
Generic Parameter              | type-level variable for polymorphism                 | parametric type variable        
Impl Block                     | impl of trait or inherent methods                    | instance declaration            
Inner<T>                       | crate-internal struct housing shared storage         | backing container handle        
Inherent Method                | method defined in an impl without a trait            | method definition               
Iterator                       | object producing a sequence of items lazily          | iterator combinator             
Iterator Adapter               | combinator transforming iterators                    | higher-order sequence transformer
lib.rs                         | crate root file aggregating public modules           | module root file                
Lifetime                       | static region describing borrow validity             | region typing interval          
Macro                          | macro_rules! or macro definition                     | Macro                           
Macro Definition               | macro rules block expanding to new syntax            | macro definition (hygienic)     
Macro Rules                    | compile-time code expansion mechanism                | hygienic macro system           
Match Expression               | branching by destructuring values                    | pattern matching expression     
MaybeUninit<T>                 | wrapper allowing staged initialization of memory     | uninitialized memory guard      
Module                         | module declaration introducing a namespace           | module definition               
Module                         | namespace grouping items                             | hierarchical module             
Module Scope                   | visibility region of a module item                   | namespace scope extent          
Move                           | transferring ownership to a new binding              | linear move semantics           
mut                            | mutable binding qualifier allowing reassignment      | mutable binding qualifier       
NoStd                          | crate mode without the standard library              | freestanding runtime profile    
Option<T>                      | value that may be present or absent                  | maybe type constructor          
Ord Trait                      | total ordering compatible with PartialOrd            | total ordering interface        
Ownership                      | each value has one controlling owner                 | affine resource ownership       
Parametric Polymorphism        | generic code uniform for all type parameters         | uniform polymorphism            
PartialEq Trait                | equality comparison that may be partial              | partial equality predicate      
PartialOrd Trait               | ordering comparison that may be partial              | partial ordering interface      
Pattern                        | syntax for destructuring values in bindings          | structural pattern descriptor   
PhantomData                    | zero-sized marker for ownership or variance          | phantom type witness            
Pin                            | wrapper forbidding moves after initialization        | pointer stability guarantee     
Place Expression               | expression referring to a storage location           | l-value expression              
Pragma                         | attribute applied to items or modules                | Pragma                          
Pub                            | visibility modifier for public items                 | Public                          
RAII                           | ensures resources released when scope ends           | scope-bound resource management 
Range<usize>                   | half-open interval used for slicing indices         | integer range descriptor        
Rc<T>                          | reference counting smart pointer without atomics     | single-thread shared pointer    
Result<T, E>                   | success-or-error value                               | either type for error handling  
Self                           | type alias for the implementing type                 | self type (type-level)          
self                           | method receiver representing the current value       | self parameter (value-level)    
Send Trait                     | marker indicating a type can move across threads     | thread-transfer capability      
Slice                          | non-owning view into contiguous elements             | array view type                 
std::cell::Cell                | interior mutability for copy types                   | copy cell                       
std::cell::RefCell             | runtime borrow-checked interior mutability           | dynamic borrow cell             
std::sync::atomic::AtomicBool  | lock-free boolean with atomic operations             | atomic boolean cell             
std::sync::atomic::AtomicUsize | lock-free usize with atomic operations               | atomic integer cell             
std::sync::atomic::Ordering    | memory ordering specifier for atomics                | memory ordering lattice         
std::sync::Barrier             | thread rendezvous synchronization                    | barrier synchronization primitive
std::sync::Condvar             | condition variable for blocking waits                | condition variable primitive    
std::sync::mpsc::Receiver      | multi-producer, single-consumer channel receiver     | channel receive endpoint        
std::sync::mpsc::Sender        | multi-producer, single-consumer channel sender       | channel send endpoint           
std::sync::Mutex               | mutual exclusion primitive with poisoning            | blocking lock abstraction       
std::sync::Once                | one-time initialization guard                        | single-assignment initializer   
std::sync::RwLock              | reader-writer synchronization primitive              | shared/exclusive lock abstraction
std::thread                    | module containing Rust threading APIs                | threading library namespace     
std::thread::Builder           | configure thread name/stack before spawn             | thread builder utility          
std::thread::current           | handle to the currently running thread               | thread descriptor handle        
std::thread::JoinHandle        | handle used to join the spawned thread               | thread join handle              
std::thread::park              | suspend current thread until unparked                | thread parking primitive        
std::thread::scope             | spawn threads borrowing stack data safely            | scoped threading construct      
std::thread::sleep             | block current thread for a duration                  | thread sleep primitive          
std::thread::spawn             | start a new OS thread running a closure              | thread creation primitive       
std::thread::unpark            | resume a parked thread via its handle                | thread unparking primitive      
std::thread::yield_now         | hint scheduler to run another thread                 | scheduler yield hint            
Struct                         | collection of named fields stored together           | product record type             
Sync Trait                     | marker allowing shared references across threads     | thread-sharing capability       
Trait                          | shared behavior specification for multiple types     | typeclass interface             
Trait Item                     | function/constant/type member in a trait             | interface member                
Trait Object                   | dynamically-dispatched trait reference               | existential interface value     
Triggerfish Syntax             | <Type as Trait>::method UFCS notation                | fully-qualified call syntax     
Type Alias                     | binding a new name to an existing type               | type synonym declaration        
Type Inference                 | local inference requiring explicit generics          | Type Inference                  
Unsafe Block                   | region where compiler safety checks are relaxed      | unchecked region                
Use                            | bringing modules or items into scope                 | import statement                
Where Clause                   | trailing generic constraints for types or functions  | type constraint clause          
Matches: 4 of 94 Rust terms share the same name as their programming-language counterparts.
Matches detail: Closure, Macro, Pragma, Type Inference.
The terms that match are: Closure, Macro, Pragma, and Type Inference.

### Syntax Examples

Name                         | Syntax                                                                          
---------------------------- | --------------------------------------------------------------------------------
Associated function call     | Type::function()                                                                
Method call                  | value.method()                                                                  
Method signature             | fn method(&self, arg: ArgType) -> ReturnType                                    
Mutable method signature     | fn update(&mut self, arg: ArgType)                                              
Self return type             | fn clone(&self) -> Self                                                         
UFCS call                    | <Type as Trait>::method(receiver, args)                                         
Struct literal               | Type { field: value }                                                           
Tuple struct literal         | Type(value0, value1)                                                            
Enum variant construction    | Enum::Variant(value)                                                            
Reference type               | &T                                                                              
Mutable reference type       | &mut T                                                                          
Raw pointer type             | *const T / *mut T                                                               
Slice type                   | &[T]                                                                            
Array type                   | [T; N]                                                                          
Function type                | fn(ArgType) -> ReturnType                                                       
Function type declaration    | fn name<T>(args: Type) -> ReturnType                                            
FnOnce bound                 | F: FnOnce(Args) -> Return                                                       
FnMut bound                  | F: FnMut(Args) -> Return                                                        
Fn bound                     | F: Fn(Args) -> Return                                                           
Type alias declaration       | type AliasName = ExistingType;                                                  
Associated type declaration  | type Item;                                                                      
Module declaration (inline)  | mod module_name { /* items */ }                                                 
Module declaration (file)    | mod module_name;                                                                
Trait declaration            | pub trait TraitName { fn method(&self); }                                       
Inherent impl block          | impl TypeName { /* methods */ }                                                 
Trait impl block             | impl TraitName for TypeName { /* methods */ }                                   
Public item                  | pub fn function_name() { /* body */ }                                           
Macro definition (macro_rules!) | macro_rules! name { (pattern) => { expansion }; }                               
Module macro definition      | pub macro_rules! name { ... } // add #[macro_export] for exports                
Macro invocation             | macro_name!(arguments);                                                         
Use declaration (crate root) | use crate::module::{Item, *};                                                   
Use declaration (external)   | use external_crate::module::{Type, *};                                          
Crate root path              | crate::module::Item                                                             
External crate path          | external_crate::module::Item                                                    
Relative path (super)        | super::{Item, *}                                                                
Relative path (self)         | self::{Item, *}                                                                 
Where clause                 | fn foo<T>(x: T) where T: Trait                                                  

## Namespaces and Calling Syntax

UFCS highlights:
- Disambiguates trait methods when multiple traits share a name.
- Uses the syntax <Type as Trait>::method(receiver, args...); the receiver may be moved,
  borrowed, or mutably borrowed depending on the trait signature.
- Useful when the trait is not imported into scope or when inherent method names collide.

Use declarations and namespaces:
- use brings modules, types, or traits into the current scope.
- Paths inside the current crate start with crate::; paths to external crates begin with the
  Cargo package name once it is in scope.
- Relative paths use self:: for the current module and super:: for the parent module.
- Group imports with braces to reduce repetition, for example use crate::foo::{Bar, Baz};

Typographical conventions:

Rust rule: when referencing a Rust-specific term, include the corresponding programming-language term in parentheses.
- snake_case for functions, modules, variables, and file names (per Rust naming conventions).
- UpperCamelCase for types, traits, and enum variants.
- SCREAMING_SNAKE_CASE for constants and statics.
- Leading underscores mark intentionally unused bindings.

## Macros

Rust macros (syntactic macros) are defined with the macro_rules! form or the newer
macro form. They can appear as pub macro_rules! inside a module, but exported macros
usually add the #[macro_export] attribute (macro export pragma). Macro expansion happens
before type checking, so the paths and names inside a macro are validated only at the call
site where it expands.

Macro notes:
- Macros expand syntactically; they are not type-checked until the expansion is inserted at the use site.
- Attributes such as #[macro_export] or #[macro_use] control visibility, while procedural
  macros rely on #[proc_macro], #[proc_macro_attribute], or #[proc_macro_derive].
- Module-level macros may be declared with pub macro_rules! and invoked via module::macro_name!(...).

### Common macro attributes

### General attributes (pragmas)

Attribute                      | Purpose
------------------------------ | --------------------------------------------------------------
#[allow(...)]                 | adjust lint level to allow selected warnings
#[deny(...)]                  | escalate selected warnings to hard errors
#[warn(...)]                  | enable additional warnings for selected lints
#[forbid(...)]                | prohibit selected lints even for downstream code
#[derive(Trait)]              | generate default implementations for the listed traits
#[must_use]                   | emit a warning if the annotated value is unused
#[inline] / #[inline(always)] | hint to the optimizer about inlining behavior
#[cold]                       | mark a function as unlikely to be executed
#[repr(...)]                  | control the layout or calling convention of a type
#[non_exhaustive]             | prevent downstream crates from exhaustively matching the item
#[cfg(...)]                   | conditionally include code based on compile-time configuration
#[macro_export]                  | export macro at the crate root
#[macro_use]                     | import macros from a module or crate
#[proc_macro]                    | mark function as procedural macro entry point
#[proc_macro_attribute]          | procedural attribute macro entry point
#[proc_macro_derive(Name)]       | derive macro entry point for Name

## Limitations

Rust’s type aliases (`type Alias = ExistingType;`) are purely compile-time nicknames: they inherit exactly the methods and trait implementations of the underlying type, but you cannot add new inherent methods or blanket trait impls to the alias itself. As a result, using a type abbreviation to replace a wrapper struct prevents you from extending the API—any new behavior must still be implemented on the original type. Whenever you need a distinct method/trait surface (for example, to expose multithreaded semantics or tailor bounds per module), you must keep a real `struct` newtype rather than a type alias.

Modules may contain:
- type definitions (struct, enum, type alias)
- trait definitions
- impl blocks
- free functions
- constants and statics
- nested modules

A file named foo.rs is treated as mod foo { … } automatically, while lib.rs and explicit
mod statements create module boundaries. Module items must be declarations, not loose
statements. Keep source lines no longer than 120 characters.

Traits may contain:
- associated types
- associated constants
- method signatures (required or default bodies)

Traits may not contain:
- stored fields or data members
- arbitrary executable statements outside default method bodies
- implementations for types (those go in impl blocks)

Impl blocks may include:
- method definitions (with bodies)
- associated type defaults
- associated constant definitions

Impl blocks may not include:
- new trait item declarations
- free functions unrelated to the impl target

Type inference limitations:
- Rust performs local, statement-by-statement inference; function signatures must state
  generics explicitly, unlike ML’s global inference.
- Trait methods that return concrete associated types (for example, fn build() -> Widget)
  do not identify the implementor, so callers often need explicit type annotations or
  turbofish syntax (explicit type application).
- Trait object upcasts (dyn Trait) erase concrete types, so methods that return Self are
  disallowed and callers must rely on associated types or where bounds.
- Inference does not speculate across trait obligations; when multiple traits provide the
  same method name, you must use UFCS or explicit annotations to disambiguate.

Rust vs. ML type inference:
- Rust requires explicit generic parameters and monomorphizes code; ML keeps polymorphic
  functions generic via Hindley-Milner inference.
- Rust traits provide ad-hoc polymorphism; ML modules or functors offer similar
  flexibility, but core ML inference does not account for trait-style obligations.
- Rust’s associated types and trait bounds often need manual annotations, whereas ML
  freely infers type variables without trait coherence constraints.

## Rust vs. ML polymorphism:

- Rust combines ad-hoc polymorphism (traits/impls) with monomorphized generics; ML leans
  on parametric polymorphism with global type inference.
- Rust allows multiple trait implementations per type (subject to coherence) and default
  methods; ML module signatures describe structure and functors instantiate behavior
  differently.
- Rust generics require explicit type parameters (no global inference) but permit
  specialization; ML infers type variables but lacks trait-style overloading.
