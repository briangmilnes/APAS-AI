## APAS Project Rules

### Assistant Vec Prohibition
- You may write `Vec` code yourself; I may not introduce new `Vec` usage at call sites.
- In sweeps, never add `Vec`/`vec![]`/`to_vec()`/`into_vec()` or equivalent conversions—stick to the provided sequence APIs (`tabulate`, `nth`, `length`, `set`, `iter`, literals).
- Outside sweeps, I need explicit permission before adding any new `Vec` usage; default assumption is “no”.
- Inside a type’s own module or impl, reuse existing `Vec` representations only when already established; do not broaden their footprint.
- Core operations must be exposed as inherent methods on the public type (no private/free-function wrappers) so tests can exercise them directly.
- Inherent conversion helpers (`from_vec`, `to_vec`) are allowed inside the defining module when they keep the `Vec` hidden from callers.
- When existing call sites already use `Vec`, keep changes local and prefer constructors/macros over expanding that usage.

### Element Shorthands and Delegation
- Use the APAS shorthands to avoid repeated bounds: `StT` (`Eq + Clone + Display + Debug + Sized`) for single-threaded data, `MtT` (`Sized + Send + Sync`) for multi-threaded contexts.
- Prefer `Pair` from `crate::Types::Types` over raw tuples in public APIs.
- Maintain chapter delegation rules: Chap.19 ST traits delegate to Chap.18 ST traits; Chap.19 MT traits delegate to Chap.18 MT traits. Never mix ST and MT traits in delegation paths because their bounds differ.

### Iterator-based Assertions
- Validate sequence contents via `iter()`, `iter_mut()`, or `into_iter()` instead of exposing backing storage so tests stay aligned with the APAS abstractions.

### Criterion Bench Configuration
- Supply representative iterator benchmarks (e.g., `iter_sum_*`).
- Use APAS timing parameters: warm-up ≤ 1 s, measurement ≈ 6 s, sample size ≈ 30, total run ≤ 10 s.

### Chapter Trait Hoisting (Option A)
- Hoist shared bounds such as `T: StT`/`T: MtT` to chapter trait headers when every method shares the element type.
- Keep extra method generics only when the method truly changes the element type (e.g., `map<U: StT>`).
- Chap.19 traits may add local bounds, but prefer reusing the hoisted bounds whenever possible.

### Parallel Spawn/Join Model
- Implement multi-threaded chapter algorithms using `std::thread::spawn` for recursive branches and `join` to synchronize completion.
- Avoid alternative thread-pool abstractions (e.g., rayon) so the parallel structure mirrors the textbook and remains amenable to Verus proofs.

### MT Module Discipline
- Any module whose filename contains `Mt` MUST deliver actual multi-threaded semantics: structure definitions must rely on `MtT` elements and internal synchronization (`Send + Sync`) rather than single-threaded `StT` shortcuts.
- Treat wrapper structs in `*Mt*` files as genuine MT types: their fields should employ `MtT` or thread-safe containers (e.g., mutexes, atomic references) and expose APIs safe for concurrent use.
- Never mirror a single-threaded implementation inside an `Mt` module; if functionality cannot be parallelised safely, move it to the `St` counterpart instead of duplicating it under the MT name.

### Iteration vs. Recursion Hygiene
- When code naturally descends a structure or mirrors the textbook recursion, opt for a compact recursive implementation (often as a nested function) instead of piling logic into a `loop { … }`.
- Straightforward iterative loops are still fine for generators or linear scans; switch only when the recursion matches the idea more clearly.
- If only one call site uses the recursive routine, keep it local to that function; hoist it out only when multiple entry points need it shared.

### Benchmark Macro Usage Patterns
- When using `StructLit!` macros in benchmark files, follow struct-specific patterns based on suffix:
  - `*Per` structs (persistent): Use `from_vec` pattern—collect data into `Vec`, then use `from_vec` or let macro handle `from_vec` internally
  - `*Eph` structs (ephemeral): Use constructor + set pattern—create with initial size/value using macro, then set individual values via `.set()` calls
  - Literal cases: Use direct macro form `StructLit![x, y, z]` when values are compile-time known
- **CRITICAL: Never replace the actual operation being benchmarked**—only replace setup/construction code that isn't the performance measurement target
- Benchmark files that specifically test `tabulate`, `map`, or other API methods must preserve those exact calls to maintain measurement validity
- This preserves performance characteristics and design patterns of persistent vs ephemeral data structures while maintaining benchmark accuracy.
