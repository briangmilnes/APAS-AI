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
