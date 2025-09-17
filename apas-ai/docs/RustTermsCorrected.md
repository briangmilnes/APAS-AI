# Rust Terms (Programming-Languages Alignment)

Rust Term                        | Description                                                    | Programming Language Term         
---------------------------------+----------------------------------------------------------------+-----------------------------------
1. Arc<T>                           | atomic reference counting smart pointer                        | concurrent shared pointer.        
2. Associated Type                  | trait-declared abstract type placeholder                       | type family member                
3. Associated Type                  | trait-defined type member                                      | type family member.               
4. Blanket Implementation           | impl block applying to many target types at once               | typeclass blanket instance        
5. Block Scope                      | region controlled by a block for locals                        | lexical scope extent              
6. Borrow                           | temporary access to a value without taking ownership           | alias with affine capability.     
7. Borrow Checker                   | compile-time analysis enforcing borrow rules                   | affine type checker.              
8. Box<T>                           | owning pointer allocating on the heap                          | owned heap allocation.            
9. Cargo                            | build system and dependency manager                            | package manager toolchain.        
10. Clone Trait                      | explicit duplication via clone()                               | copy constructor interface        
11. Closure                          | inline function capturing an environment                       | lambda with lexical environment.  
12. Const Generic                    | compile-time constant parameter                                | indexed type parameter.           
13. Copy Trait                       | marker for implicit bitwise copy semantics                     | implicit copy qualifier           
14. Crate                            | primary compilation unit and package                           | top-level module artifact.        
15. Debug Trait                      | formatting for developer diagnostics                           | debug-print interface             
16. Deref Trait                      | overloads `*` to follow smart pointers                         | implicit dereference protocol.    
17. Display Trait                    | formatting as user-facing text                                 | pretty-print interface            
18. Drop                             | deterministic cleanup that runs when a value goes out of scope | RAII finalizer.                   
19. Enum                             | tagged union that can hold one of several variants             | algebraic sum type.               
20. Eq Trait                         | marker indicating total equality compatible with PartialEq     | equivalence relation marker       
21. Free Function                    | standalone function not tied to a type or impl                 | top-level function definition     
22. Future                           | value representing an asynchronous computation                 | promise/future type.              
23. Generic Parameter                | type-level variable for polymorphism                           | parametric type variable.         
24. Impl Block                       | concrete implementation of a trait or inherent methods         | instance declaration.             
25. Inherent Method                  | method defined in an impl without a trait                      | method definition                 
26. Iterator                         | object producing a sequence of items lazily                    | iterator combinator.              
27. Iterator Adapter                 | combinator transforming iterators                              | higher-order sequence transformer.
28. Lifetime                         | a static region describing how long a borrow is valid          | region typing interval.           
29. Macro Definition                 | macro rules block expanding to new syntax                      | macro definition (hygienic)       
30. Macro Rules                      | compile-time code expansion mechanism                          | hygienic macro system.            
31. Match Expression                 | branching by destructuring values                              | pattern matching expression.      
32. Module                           | module declaration introducing a namespace                     | module definition                 
33. Module                           | namespace grouping items                                       | hierarchical module.              
34. Module Scope                     | visibility region of a module item                             | namespace scope extent            
35. Move                             | transferring ownership to a new binding                        | linear move semantics.            
36. NoStd                            | crate mode without the standard library                        | freestanding runtime profile.     
37. Option<T>                        | value that may be present or absent                            | maybe type constructor.           
38. Ord Trait                        | total ordering comparison compatible with PartialOrd           | total ordering interface          
39. Ownership                        | each value has exactly one controlling place at a time         | affine resource ownership.        
40. PartialEq Trait                  | equality comparison that may be partial                        | partial equality predicate        
41. PartialOrd Trait                 | ordering comparison that may be partial                        | partial ordering interface        
42. Pattern                          | syntax for destructuring values in bindings                    | structural pattern descriptor.    
43. PhantomData                      | zero-sized marker for ownership or variance                    | phantom type witness.             
44. Pin                              | wrapper forbidding moves after initialization                  | pointer stability guarantee.      
45. Place Expression                 | expression referring to a storage location                     | l-value expression                
46. RAII                             | guaranteeing resource release when an owner leaves scope       | scope-bound resource management.  
47. Rc<T>                            | reference counting smart pointer without atomics               | single-thread shared pointer.     
48. Result<T, E>                     | success-or-error value                                         | either type for error handling.   
49. Self                             | type alias for the implementing type inside traits/impls       | self type (type-level)            
50. Send Trait                       | marker indicating a type can move across threads               | thread-transfer capability.       
51. Slice                            | non-owning view into contiguous elements                       | array view type.                  
52. Struct                           | collection of named fields stored together                     | product record type.              
53. Sync Trait                       | marker indicating shared references are thread-safe            | thread-sharing capability.        
54. Trait                            | shared behavior specification for multiple types               | typeclass interface.              
55. Trait Item                       | function/constant/type member declared inside a trait          | interface member                  
56. Trait Object                     | dynamically-dispatched trait reference                         | existential interface value.      
57. Triggerfish Syntax               | <Type as Trait>::method UFCS notation                          | fully-qualified call syntax       
58. Type Alias                       | binding a new name to an existing type                         | type synonym declaration          
59. Unsafe Block                     | region where compiler safety guarantees are relaxed            | unchecked region.                 
60. `std::cell::Cell`                | interior mutability for copy types                             | copy cell.                        
61. `std::cell::RefCell`             | runtime borrow-checked interior mutability                     | dynamic borrow cell.              
62. `std::sync::Barrier`             | thread rendezvous synchronization                              | barrier synchronization primitive 
63. `std::sync::Condvar`             | condition variable for blocking waits                          | condition variable primitive      
64. `std::sync::Mutex`               | mutual exclusion primitive with poisoning                      | blocking lock abstraction.        
65. `std::sync::Once`                | one-time initialization guard                                  | single-assignment initializer     
66. `std::sync::RwLock`              | reader-writer synchronization primitive                        | shared/exclusive lock abstraction 
67. `std::sync::atomic::AtomicBool`  | lock-free boolean with atomic operations                       | atomic boolean cell               
68. `std::sync::atomic::AtomicUsize` | lock-free usize with atomic operations                         | atomic integer cell               
69. `std::sync::atomic::Ordering`    | memory ordering specifier for atomics                          | memory ordering lattice           
70. `std::sync::mpsc::Receiver`      | multi-producer, single-consumer channel receiver               | channel receive endpoint          
71. `std::sync::mpsc::Sender`        | multi-producer, single-consumer channel sender                 | channel send endpoint             
72. `std::thread::Builder`           | configure thread name/stack before spawn                       | thread builder utility            
73. `std::thread::JoinHandle`        | handle used to join the spawned thread                         | thread join handle                
74. `std::thread::current`           | handle to the currently running thread                         | thread descriptor handle          
75. `std::thread::park`              | suspend current thread until unparked                          | thread parking primitive          
76. `std::thread::scope`             | spawn threads borrowing stack data safely                      | scoped threading construct        
77. `std::thread::sleep`             | block current thread for a duration                            | thread sleep primitive            
78. `std::thread::spawn`             | start a new OS thread running a closure                        | thread creation primitive         
79. `std::thread::unpark`            | resume a parked thread via its handle                          | thread unparking primitive        
80. `std::thread::yield_now`         | hint scheduler to run another thread                           | scheduler yield hint              
81. `std::thread`                    | module containing Rust threading APIs                          | threading library namespace       
82. lib.rs                           | crate root file aggregating public modules                     | module root file                  
83. mut                              | mutable binding qualifier allowing reassignment                | mutable binding qualifier         
84. self                             | method receiver representing the current value                 | self parameter (value-level)      

## Limitations

Modules may host type/trait/impl definitions, free functions, constants, and nested
modules. Filename modules (e.g., `foo.rs`) implicitly wrap contents in `mod foo { … }`,
while `lib.rs` or explicit `mod` blocks provide the module shell. Only item declarations
are permitted at module top level—no loose statements.

Traits may declare associated types, constants, and method signatures, but cannot store
fields or contain executable statements. Impl blocks supply concrete method bodies (and
associated type defaults) for a receiver type or trait, yet cannot introduce new trait
items or standalone functions.
