## APAS User Rules — Canonical (UserRules4)

### Code Elegance and Minimalism

#### Always choose the minimal solution
- Extend existing types/traits rather than creating new ones
- When sketching designs, start with the smallest possible change
- Avoid over-engineering - most example code online is unnecessarily complex
- Elegance comes from simplicity, not from adding features or abstractions

#### Closure Mutation Patterns
- **FnMut vs Fn**: When closures need to mutate captured variables, they require `FnMut` trait bounds
- **Vec-based workaround**: If a function requires `Fn` but you need mutation (like `tabulate`), replace closure-based implementation with explicit `Vec` loops
- **Pattern**: `let mut acc = init; for i in 0..n { acc = f(&acc, &data[i]); results.push(acc.clone()); }`
- This avoids closure capture issues while maintaining functional semantics

### Imports and Module Scope

#### Standard Library Imports and Result usage (module-top; no aliasing)
- Put needed std imports at the top of each module. Don’t write `std::…` inside trait/impl bodies.
- Don’t alias std items. Import by their real names.
- Bounds in code should read minimally (see baseline rules below).
- Prefer importing frequently used std items (`Iter`, `Formatter`, etc.) rather than repeating long paths.
- Import order: after the module declaration add a blank line, then all `use std::…` lines, then a blank line, then `crate::…` imports.

Result guidance
- Formatting-only files (no generic `Result<T, E>`):
  - Import `Display`, `Debug`, `Formatter`, `Result` and use bare `Result` in `fmt` methods.
  - Example:
    ```rust
    use std::fmt::{Display, Debug, Formatter, Result};
    impl Display for Foo {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { /* … */ }
    }
    ```
- Files that also use generic `Result<T, E>`:
  - Do NOT import `fmt::Result`. Keep generic `Result<T, E>` bare for APIs, and use `std::fmt::Result` only in `fmt` methods.
  - Example:
    ```rust
    use std::fmt::{Display, Debug, Formatter};
    fn do_work() -> Result<u32, &'static str> { /* … */ }
    impl Display for Foo {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { /* … */ }
    }
    ```

#### Specialized import rules
- Inside `src/` (library code):
  - Use `crate::…` for all intra-crate paths.
  - Prefer wildcard imports for your own modules: `use crate::Mod1::Mod1::*;`.
  - Macros exported at crate root: import with `use crate::FooSeqLit;` (or call as `crate::FooSeqLit![…]`).
- Outside the crate (`tests/`, `benches/`, `examples/`, dependents):
  - Use the crate identifier derived from `[package] name` with hyphens replaced by underscores. Example: package `apas-ai` → `apas_ai` in code.
  - Prefer wildcard imports: `use apas_ai::Mod1::Mod1::*;`.
  - Macros: `use apas_ai::FooSeqLit;` then `FooSeqLit![…]`.
- Unit tests inside `src/` modules (`#[cfg(test)] mod tests`):
  - Treat as inside-crate: `use crate::…` and `use crate::FooSeqLit;`.
- Never use `extern crate`. Do not add re-exports.

#### Module import style for this project
- In user modules, avoid importing individual symbols. Don’t use `use Foo::{Bar,Baz}`.
- Prefer wildcard imports `use Foo::*` for your own modules (includes traits); let the module control what’s public.
- Minimize repeated `use crate::...` lines: group them once with braces, e.g. `use crate::{Types::Types::*, LinkedListStPer::LinkedListStPer::*, LinkedListStPerChap18::LinkedListStPerChap18::*};`.
- Fall back to explicit symbol imports only to resolve name collisions.
- `PartialEq` and `Eq` definitions should be inside the file’s single module.

#### No trailing per-file re-exports (use lib.rs instead)
- Do not place lines like `pub use FooMod::FooModTrait;` at the end of source files.
- If a re-export is desired for public API ergonomics, add it in `src/lib.rs` only.
- Inside modules, import items via their module paths (e.g., `use crate::FooMod::FooMod::*;`) rather than relying on per-file re-exports.
- Macros remain defined inside their modules with `#[macro_export]`; do not add extra re-exports for them.

#### Wildcard-first imports; group to minimize `use crate`
- Default: wildcard-import module contents (including traits): `use crate::SomeMod::SomeMod::*;`.
- Prefer a single grouped import per file: `use crate::{A::A::*, B::B::*};`.
- Only name symbols explicitly when disambiguating or when a wildcard would pull conflicting items into scope.

#### Use Lit! macros for literal data construction
- Always construct fixed, small literal values using the provided `...Lit!` macros (e.g., `SetLit!`, `RelationLit!`, `MappingLit!`).
- For pair-like elements, use `Pair` inside the literal: `SetLit![Pair(a, b), Pair(c, d)]`.
- Do not hand-build literals with temporary vars, loops, or manual inserts in tests or examples. Prefer the literal macro for clarity and brevity.
- If a macro cannot express the literal you need, prefer adding/updating that macro rather than open-coding a one-off constructor.

#### Helper function extraction threshold
- Do not introduce a helper function unless it will be used in at least 3 distinct call sites (or across 2+ modules), or it eliminates clearly error-prone duplication.
- Otherwise, keep the code inline or use an existing macro/constructor. Exceptions: readability for >10 lines of complex logic.

#### Module/file layout
- Each file should have a single module.

---

### Types, Bounds, and Lifting

#### Types and Bounds (Baseline + Minimal Additions)
- Baseline (project‑wide): Public types, traits, and impls must bind element type parameters to `Eq + Clone + Copy + Display + Debug + Sized` at the declaration site (not on every method).
- Hash is opt‑in: Add `Hash` only when required by a specific API (e.g., `HashMap`/`HashSet` or hashing). Prefer the narrowest scope (a single method or specific impl). Do not hoist `Hash` to a trait header unless every item needs it.
- Minimal additions: Add extra bounds only when strictly required by stdlib contracts or called code, and keep them as local as possible (method/impl where‑clause).
- Associated types: When an associated type represents the element type, ensure it satisfies `Eq + Clone + Copy + Display + Debug + Sized`; add `Hash` only when that associated type is used in hashing contexts.

#### Generalized lifting rule (applies to every trait/impl)
- Project baseline for public element type parameters is: `Eq + Clone + Copy + Display + Debug + Sized`.
- Bind the baseline at the declaration site (trait/struct/impl header). Do not repeat baseline bounds on every method.
- Hoist only bounds that are common to all trait items’ signatures/where‑clauses and satisfiable at all call sites.
- Do NOT hoist `Hash` unless every item requires it.
- Method‑only extras stay local (on that item). If every item in an impl needs an extra bound, put it on that impl header.
- Mirror hoisted bounds from the trait header exactly on every corresponding impl header.
- Do not hoist lifetimes. Keep lifetime parameters where they logically belong.
- If multiple impl blocks repeat identical bounds, unify them on the impl header; avoid per‑method duplication.

Example
```rust
pub trait Foo<T: Eq + Clone + Copy + Display + Debug + Sized> {
    fn show(&self, x: T) -> String;

    fn bucket(&self, x: T) -> usize
    where
        T: std::hash::Hash;
}

impl<T: Eq + Clone + Copy + Display + Debug + Sized> Foo<T> for Bar<T> {
    fn show(&self, x: T) -> String { /* … */ }

    fn bucket(&self, x: T) -> usize
    where
        T: std::hash::Hash,
    {
        /* … */
    }
}
```

#### Type Creation Traits (align with baseline)
#### Type shorthands: `StT` and `MtT` (Elem split)
- Purpose: avoid repeating `T: Eq + Clone + Copy + Display + Debug + Sized` by using a blanket `Elem` trait.
- Definition (in `crate::Types::Types`):
  ```rust
  use std::fmt::{Display, Debug};
  pub trait Elem: Eq + Clone + Display + Debug + Sized {}
  impl<T> Elem for T where T: Eq + Clone + Display + Debug + Sized {}
  ```
- Default hoist site: prefer hoisting `T: Elem` on public traits and their general impls. For struct headers, you may hoist `T: Elem` now that `Copy` was dropped; if you must support `Mutex<…>` elements, use the exception below.
- Pair preferred over raw tuples: use `crate::Types::Types::Pair<A, B>` instead of `(A, B)`.
- StT: single-threaded elements = `Eq + Clone + Display + Debug + Sized`. Use on public data structures like `ArrayPerS<T: StT>`.
- MtT: multi-threaded elements = `Sized + Send + Sync` only. Use in Chap. 19 when storing `Mutex<...>` inside structures.
  No custom wrapper types; use `std::sync::Mutex<T>` directly with `T: mtT`.
- **Trait delegation rule**: `StT` and `MtT` have incompatible bounds and cannot cross-convert. Therefore:
  - `*StPerChap19` traits must delegate to `*StPerChap18` traits (both use `StT`)
  - `*MtPerChap19` traits must delegate to `*MtPerChap18` traits (both use `MtT`)
  - Never delegate from Mt* to St* traits or vice versa due to incompatible trait bounds

#### Trait-bound hoisting for chapter traits (Option A)

- Prefer generic trait parameters to avoid repeating method-scoped type params when the element type stays the same across methods.
- For Chap. 18 sequence algorithms, define the trait as `pub trait ArraySeqPerChap18Trait<T: MtT> { ... }` and implement with `impl<T: MtT + Clone + Eq> ArraySeqPerChap18Trait<T> for ArrayPerS<T> { ... }`.
- Keep additional method generics only when the method changes the element type (e.g., `map<U: StT>(...) -> ArrayPerS<U>`).
- Chap. 19 remains free to introduce extra bounds locally, but prefer hoisting `MtT` where possible on trait-level generics.

#### Function argument bounds without where-clauses (new)
- Prefer inline generic bounds directly on the function’s generic parameters and arguments; avoid trailing `where` clauses unless:
  - The bounds are too long to read inline (≥ 2 traits on ≥ 2 params), or
  - You need higher-ranked trait bounds/lifetimes making inline form unreadable.
- For methods returning sequence types, put the element bound inline on the method generic: `fn map<U: StT + Clone>(...) -> Seq<U>` not `fn map<U>(...) -> Seq<U> where U: StT + Clone`.
- For inherent/trait methods that repeat a single bound across many items in the same impl, hoist to the impl header per the hoisting rules above; otherwise keep inline.

#### Callable parameter style (`impl Fn` in parameter position)
- Prefer `impl Fn` in parameter position when a function takes a callable and you do not need to name its concrete type, unify it across parameters, or return it. This keeps signatures short and avoids a separate `where` clause.
- Use a named generic like `F: Fn(...) -> _` if the callable's type must be referenced in multiple places (e.g., two parameters must be the same closure type) or if the function returns the callable.
- Use a trait object like `&dyn Fn(...) -> _` for dynamic dispatch or heterogeneous storage behind a pointer; accept the virtual call overhead.
- Pick the correct trait: `Fn` for non‑mutating closures, `FnMut` if the closure mutates captured state, `FnOnce` if it consumes captured values.

Example (applies to `reduce`):
```rust
fn reduce<T: MtT + Clone + Eq>(
    a: &ArrayPerS<T>,
    f: impl Fn(&T, &T) -> T,
    id: T,
) -> T
```

#### Default element bound (StT by default)

- Default: Use `StT` (`Eq + Clone + Display + Debug + Sized`) for public data structures and chapter traits.
- MtT is exceptional: Use `MtT` (`Sized + Send + Sync`) only when concurrency primitives are stored (e.g., `Mutex`, parallel chap19 algorithms) or thread-safety is otherwise required.
- For `ArrayPerS<T>`: if parallel algorithms store thread-safe wrappers, constrain chapter traits/methods to `MtT` locally; otherwise keep the core type and common traits `StT`.
- `Hash` remains opt‑in; not part of `Elem`.
- `Hash` stays opt‑in: add `T: Hash` only on the specific methods/impls that use hashing; do not include `Hash` in `Elem`.

- For new public concrete types:
  - Derive `Copy`, `Clone`, `Debug`, `PartialEq`, `Eq`.
  - Implement `Display`.
  - Add `Hash` only if the type is used in hashed contexts.
- Sequence‑like types (wrap or behave like a collection):
  - Provide `iter()` and `iter_mut()`.
  - Implement `IntoIterator` for owned, `&Self`, and `&mut Self`.
  - Implement `ExactSizeIterator`/`DoubleEndedIterator` where applicable.

---

### APIs, Macros, Constructors, Encapsulation

#### Macro Normalization (Exported and Type‑Checked)
- Define at crate root:
  - Use `#[macro_export]` and place `macro_rules!` at crate root (end of the module file).
  - Inside the macro body, use `$crate::` fully qualified paths to all types/functions.
- One definition: no module‑local duplicates or wrappers; one canonical macro per type family.
- Call‑site ergonomics:
  - Non‑empty forms rely on element types (ints default to `i32`); empty forms require a minimal type annotation at the call site.
  - Import macros from the crate root in tests/benches with `use my_crate::MacroName;`; inside `src` you can use `crate::MacroName` or invoke directly.
- Macros are all `pub`; used to make datatype literals (`Lit!`).
- Dead‑code type‑check helper (required; must include empty form):
```rust
#[allow(dead_code)]
fn _MyMacro_type_checks() {
    let _ = MyMacro![1];               // non‑empty infers (e.g., i32)
    let _: MyType<i32> = MyMacro![];   // empty form requires explicit type
}
```
- Naming: keep macro names consistent and descriptive (e.g., `FooSeqLit!`), aligned with the type they construct.

#### Constructor No Raw Backing Collections
- Never construct sequence types via raw backing collections at call sites (e.g., `Vec::new`, `vec![…]`, or `T { data: … }`).
- Always use the type’s inherent constructors or macros: `T::new()`, `T::from_vec(vec)`, or `TSeqLit![…]`.
- If the type lacks an inherent constructor, add one in its module, then update call sites to use it.
- Keep any direct `T { data: … }` or `vec![…]` usage confined to the type’s own module/impls only (preserve invariants; avoid representation leaks).
- UFCS constructors (`<T as Trait>::new/…`) are prohibited at call sites; prefer inherent or macro forms.

#### Assistant Vec prohibition (sweeps vs non‑sweeps)
- The user may write `Vec` code; the assistant may not introduce new `Vec` usage at call sites.
- Sweeps: do not propose or add `Vec`/`vec![]`/`to_vec()`/`into_vec()` or equivalent conversions anywhere. Use only existing APIs (`tabulate`, `nth`, `length`, `set`, `iter`, macros).
- Non‑sweeps: ask explicit permission before adding any `Vec` usage; default answer assumed “no”. If denied or unclear, avoid it.
- Exceptions limited to internals: inside a type’s own module/impl, using `Vec` to implement `from_vec`, `set`, or builders is allowed if already the established representation. Do not expand its footprint.
- Pre‑existing `Vec` at a call site: do not propagate or broaden it; keep changes local and prefer replacing with constructors/macros when practical.

#### Struct Field Encapsulation
- Default: struct fields are non‑public; hide representation by default.
- Access via API: expose state through inherent methods and trait impls (e.g., `iter`, `iter_mut`, `len`, `as_slice`), not public fields.
- Construction: use constructors/macros; disallow struct literals outside the defining module.
- Exceptions: a field may be public only with explicit user approval and documented invariants.
- Visibility scope: prefer private; use `pub(crate)` only when necessary and justified in docs.
- Testing: write tests against the public API, not internal fields.

#### No Free‑Function Wrappers
- Do not create free functions that merely forward to trait or inherent methods (e.g., `fn select(a,b,i) → <Type as Trait>::method`).
- If a method isn’t available on the receiver, add an extension trait implemented for the concrete type to expose `value.method(...)`.
- Allowed: free functions only when they add real semantics (compose multiple types, add logic not tied to a single receiver, or break dependency cycles). Do not duplicate method names as free functions.

#### Type Conversions and Naming
- Prefer traits over ad‑hoc functions:
  - Implement `From<Src>` for `Dst` or `TryFrom<Src>` for `Dst`.
  - Call via `Dst::from(src)` or `src.into()`.
- Use `to_*` only when cloning/allocating an owned value is required (e.g., `to_string`, `to_vec`, `to_owned`).
- Use `as_*` for cheap borrows/views with no allocation (e.g., `as_str`, `as_slice`).
- Use `into_*` only when consuming `self` clarifies intent or returns a non‑obvious type (e.g., `into_inner`, `into_boxed_slice`).
- Do not add `to_Type` or extra `from_*` helpers if `From/Into/TryFrom/TryInto` suffices.
- Constructor exception: allow inherent `from_vec` where idiomatic; otherwise prefer trait‑based conversions.

---

### Iteration, Iterators, and Tests

#### Iterator impls (three forms)
- Implement all three `IntoIterator` forms for your sequence type (owned, `&Self`, `&mut Self`).
- Provide inherent `iter()` and `iter_mut()` helpers that delegate to the backing collection.
- Avoid unnecessary bounds; add `T: Default` only if methods (e.g., `set`) require it.
- Add `ExactSizeIterator` when length is known in O(1) and stable during iteration.
- Add `DoubleEndedIterator` when items can be traversed from both ends without extra allocation or violating semantics.

#### Iterator‑based Assertions
- Use `iter()/iter_mut()/into_iter()` to assert sequence contents instead of exposing storage.

#### Tests Format
- One test per public function/trait item in a module (include iterator and formatting coverage).
- Equality: test `PartialEq`/`Eq` behavior explicitly where defined.
- Iterator tests: cover forward and reverse traversal where applicable; assert lengths for `ExactSizeIterator`.
- Formatting:
  - `Display`: assert exact formatted string.
  - `Debug`: assert it contains the type name or key structure elements, as appropriate.
- Prefer `assert_eq!` on data values wherever possible.
- Prefer `<MacroName>Lit![…]` for non‑empty literals; use `T::new()` for empty cases instead of `<MacroName>Lit![]`.

#### Test via Public API Only
- Write tests against exposed methods/traits/macros; never against private fields.

#### Benches (criterion, short runs)
- Provide representative iterator benches (e.g., `iter_sum_*`).
- Configuration: warm‑up ≤ 1s; measurement ≈ 6s; sample size ≈ 30; total ≤ 10s.

---

### Naming and Inference

#### CamlCase not SnakeCase
- Functions/structures of more than one English word use CamlCase.
- One‑word functions may be all lower case.
- File names should be in CamlCase and start with a capital.

#### Type Inference Cleanup
- Avoid UFCS/turbofish unless required: don’t use `<Type as Trait>::method(...)` or `method::<T, _>(...)` at call sites if method‑call syntax with the trait in scope suffices.
- Prefer inferred bindings: `let x = expr;` when the type is deducible from the expression or later usage.
- Minimal annotations: when needed, use `let x: T = expr;`.
- Constructors: prefer inherent constructors (`Type::new`, `Type::from_vec`) or `Lit!` macros for literals.
- Numeric guidance: rely on defaults (`i32` for integers, `f64` for floats); add literal suffixes only when necessary.
- Iteration ergonomics: favor `iter()/iter_mut()` and `for` loops over explicit `into_iter()` unless consuming ownership is intentional.
- Eliminate redundant annotations/turbofish where later statements already constrain the type.

#### Contain UFCS (call‑site elimination)
- Replace `<Type as Trait>::item(...)` at call sites with method‑call syntax wherever possible.
- Ensure traits are imported; add inherent constructors or small extension traits and impls if needed.
- Keep UFCS inside impls/traits for disambiguation; minimize UFCS in callers (tests/benches should not need UFCS).

---

### Tooling, Execution Protocol, and Transparency

#### Tool Usage Transparency
- Announce every tool call before it runs (single line). Formats:
  - `Tool: read_file — path: /abs/path/file.rs`
  - `Tool: edit_file — file: /abs/path/file.rs`
  - `Tool: apply_patch — file: /abs/path/file.rs`
  - `Tool: codebase_search — query: "…"; target: ["…"]`
  - `Tool: grep — pattern: '…'; path: /abs/path; type/glob: rs`
  - `Tool: todo_write — merge: true; items: N`
  - `Tool: read_lints — paths: ["…"]`
  - `Tool: list_dir — target: /abs/dir`
  - `Tool: glob_file_search — target: /abs/dir; glob: "**/*.rs"`
  - `Tool: run_terminal_cmd — command: "…"`
- Redact secrets; truncate commands >160 chars. Stop announcements on STOP.
 - Sweeps (hard rule, SEV2 on miss): During sweeps, echo EVERY tool call and batch related calls per step. Missing tool‑echo lines or silent tool usage during a sweep is a SEV2 violation; resume immediately and continue relentlessly.

#### Tool Calls — Batching, Retries, Background
- Parallel: print a single `ToolCalls:` block, one line per tool in the batch.
- Retries: append `(attempt X/3)` to the tool line.
- Background: append `(background)` for background jobs.
- Clear policy: for shell commands, always use a clear‑first in a separate invocation before running the command.

#### Terminal Clear & Output Visibility
- Clear‑first, separate calls: run the terminal clear in its own invocation, then run the actual command in a new invocation. Never chain clear + command with `&&`.
  - Clear: `printf '\033[3J\033[H\033[2J'`
  - Then: `<your command> 2>&1 | cat`
- One command per terminal window. Always pipe command output via `| cat`.
- Exception: if you explicitly say “no‑clear”, skip the clear step; still keep one command per terminal invocation.

#### Non‑terminal Step Timing (Edits/Searches/Planning)
- Provide precise timing for non‑terminal work on request.
- On request, bracket each non‑terminal step with Start/End timestamps in chat status updates.
- Obtain timestamps via a shell date call; use system timezone unless PDT is explicitly requested.
  - Start: `date +"Start: %a %b %d %T %Z %Y"`
  - End:   `date +"End: %a %b %d %T %Z %Y"`
- One Start/End pair per distinct step; repeat per step as needed.
- Optionally include Total (H:M:S) after End if helpful.
- Do not run extra commands solely for timing unless requested.

#### Verification Trace (imports & bounds hoist)
- For each file verification, print:
  - `Verifying: <ABS-PATH>`
  - `Start: HH:MM:SS (system TZ)`
  - Checks (one line each):
    - imports: inside `src` use `crate::`; wildcard module imports; no `extern crate`
    - macros: exported at crate root; `$crate` paths; type‑check helper present
    - bounds hoist: gather per‑method bounds; hoist union to trait/impl; do not lift lifetimes; mirror impls
    - baseline: public APIs include `Clone + Sized + Display + Debug` at declaration sites
    - naming: CamlCase for multi‑word items; files start with a capital
  - Tool calls announced via Tool Usage Transparency.
  - `End: HH:MM:SS (system TZ); Total: H:MM:SS`

#### emacs_sync
- Trigger: when you say you’ll “fix/edit in Emacs,” pause all write edits immediately.
- Pause behavior: read‑only work only (reviews/plans), no file edits until “Emacs: done”.
- Resync on done: propose refreshing repo state, then re‑open changed files.
  - Suggested:
    ```bash
    git status -s | cat
    git diff --name-only | cat
    ```
- Reconcile: prefer your Emacs changes; never overwrite. If conflicts arise, summarize diffs and ask how to proceed.
- Style guarantees: keep existing indentation, whitespace, and CamelCase module naming intact.
- Scope hygiene: avoid global refactors while you’re editing; target only files you didn’t modify unless you approve.
- Follow-ups: after resync, run lint/diagnostics on changed files and resume the pending task.

#### Do not load from rules or prompts files
- Do not read files from the rules or prompts directory without explicit instruction; content may be out of date.

---

### TODO Flow and Execution Discipline

#### Relentless TODO Flow (No‑Pause Autopilot; 3‑Attempt; Completion Guard)
- SEV levels:
  - SEV1: Critical breakage (data loss/destructive edits).
  - SEV2: Workflow breach (stopping/asking during sweeps; missed TODO completion; failure to continue relentlessly).
  - SEV3: Minor style/process deviations.
- Scope: applies to sweeps and non‑sweeps.
- Execution: synchronous; clear terminal in a separate call; non‑interactive flags; always `| cat`; one command per terminal invocation.
- Timing: assistant‑text Start/End/Total from the system clock.
- Attempts: on any error, make up to 3 self‑directed fix attempts; clear before each; keep edits minimal/local.
- No‑pause Autopilot (hard rule): do not ask for confirmation or pause for review. Continue automatically unless and until one of the two hard stops occurs:
  1) After 3 failed attempts, or
  2) A fix would be destructive/ambiguous.
  - Violation during sweeps is SEV2: Treat any stop/question/confirmation prompt during a sweep as a SEV2 bug. Immediately resume, log the violation in the status, and proceed relentlessly.
  In either stop, mark the TODO failed, print exact diagnostics, create a “Resume …” TODO, and immediately continue to the next independent TODO.
- Completion guard: after a successful step, immediately complete the active TODO; if another exists, set it `in_progress` and print its Start; otherwise print “All TODOs completed”. No dangling TODOs (except explicit “[long‑run]”).
- Long‑run tasks: only when tagged “[long‑run]”; exclude from totals until stopped; don’t mark complete until stop finishes.

#### Per‑TODO Execution (hard gate)
- On setting a TODO `in_progress`: print Start via terminal:
  - `date +"Start: %a %b %d %T %Z %Y"`
- Execute exactly one synchronous action (or a clearly batched set): edits and/or 1 command, piped with `2>&1 | cat`. Reuse SAME Start for retries.
- On finishing: print End via terminal:
  - `date +"End: %a %b %d %T %Z %Y"`
- Provide a brief summary. Immediately complete or fail the TODO; if another exists, set it `in_progress` and print its Start.

#### TODO Completion Guard (Last Item)
- Same‑message order:
  1) End and Total
  2) Brief summary
  3) Complete active TODO
  4) If another exists, set `in_progress` and print Start; else “All TODOs completed”.
- Only `[long‑run]` can remain active without completion.

#### Immediate TODO-completion guards (hard rules)
- Immediate completion on success (mandatory): immediately after any successful edit/command that fulfills the active TODO, call todo_write (merge=true) to mark it completed before any further tool calls or sending the message.
- No-call invariant: you may not start another tool call while any fulfilled TODO remains in_progress.
- No-send invariant: you may not send a message with a fulfilled TODO still in_progress (except explicit “[long‑run]”).
- Single-active: at most one TODO may be in_progress. To start the next task, first complete the current one; only mark the next TODO in_progress when its first tool call starts.
- Start-of-turn reconcile: before the first tool call each turn, if any prior in_progress TODO is already fulfilled, complete it via todo_write immediately.
- Batch completions: if one operation satisfies multiple TODOs, complete all of them in a single todo_write batch right after that operation.
- Escalation on miss (SEV2): if a fulfilled TODO is found in_progress at the start of a turn, immediately complete it and create a “Follow‑up: missed TODO completion (SEV2)” TODO, then continue.

#### Clearing TO‑DOS
- Overwrite the entire list via the todo tool with `merge=false` and `todos=[]`. State “All TODOs cleared.” and stop.

#### Update todo status eagerly
- If a TODO is done, check it off immediately and set the next TODO to `in_progress`.

#### Never Stop Until All TODOs Complete (Absolute Completion Rule)
- **NEVER declare victory or completion until ALL TODO items are marked as "completed"**
- Always check the TODO list status before claiming a task is finished
- If any TODOs remain pending or in_progress, continue working relentlessly until every single item shows status "completed"
- Do not ask for permission to continue - just keep executing until 100% completion is achieved
- **NO EXCEPTIONS** for declaring success until the todo list shows 100% completion status
- This rule enhances all existing TODO Flow rules with an absolute completion requirement

#### Pause Question
- When asked if you are paused, answer explicitly: “Yes I am paused” or “No I am not”; explain why or what you’re doing; continue with the work.

---

### Environment, Tools, and Misc

#### Cargo Nextest
- `cargo nextest` writes to stderr; use `2>&1 | cat` to capture output.

#### No Perl; awk acceptable
- Use `rg/grep`; `awk` only if needed; never Perl.

#### CPR rule (heartbeat lines)
- Always write brief heartbeat lines (e.g., “Thinking” or “Planning”) so progress is visible—before a tool batch, after edits/commands, and at completion.

#### High‑Signal Live Tracing (mandatory)
- Heartbeat after every batch: After every tool call batch and every code edit, write a 1–2 line status update naming the active TODO, files touched, and the next action.
- No‑quiet window: If 60 seconds elapse or 2 consecutive tool batches occur without user‑visible output, send a status update anyway (e.g., “still building/searching…”).
- Command echo: Before running a non‑trivial shell command, print the exact command in one line; after it finishes, note success/failure.
- Build/test digest: After any build/test, output a 3–8 line digest including the first failing file:line with error code, total error count (approximate OK), and the concrete next fix.
- Edit diff signal: After any file edit, report the file path and a short change label (e.g., “hoisted T: StT; tuples→Pair”).
- Search signal: For broad searches, report match counts and top 1–3 relevant paths (or “no matches”).
- TODO‑anchored tracing: Each status must reference the active TODO name and state, plus the immediate next step.
- Failure sentinel: On repeat of the same error, state the suspected root cause and the specific change being applied next.
- Blocked state: If input is required, state the exact question and mark the TODO as blocked until answered.

#### Minimal Constructor Surface
- Prefer inherent constructors/macros (`new`, `from_vec`, `Lit![]`) over struct literals in tests.


