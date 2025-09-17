APAS User Rules — Update Bundle

Rule: Macro Normalization (Exported and Type-Checked)
- Define at crate root:
  - Use #[macro_export] and place macro_rules! at crate root (end of the module file).
  - Inside the macro body, use $crate:: fully qualified paths to all types/functions.
- One definition:
  - No module-local duplicates or wrappers; one canonical macro per type family.
- Call-site ergonomics:
  - Non-empty forms rely on element types (ints default to i32); empty forms require a minimal type annotation at the call site.
  - Import macros from the crate root in tests/benches with use my_crate::MacroName; inside src you can use crate::MacroName or invoke directly.
- Macros are all to be made pub as I am using them only to make datatype literals (Lit!).
- Dead-code type-check helper (required; must include empty form):
  - Immediately after the macro, add a private helper to force expansion/type checks at compile time.
  - Always include BOTH a non-empty form and an empty form test.
```rust
#[allow(dead_code)]
fn _MyMacro_type_checks() {
    let _ = MyMacro![1];               // non-empty infers (e.g., i32)
    let _: MyType<i32> = MyMacro![];   // empty form requires explicit type
}
```
- Naming:
  - Keep macro names consistent and descriptive (e.g., FooSeqLit!), aligned with the type they construct.

Rule: Constructor No Raw Backing Collections
  - Never construct sequence types via raw backing collections at call sites (e.g., Vec::new, vec![…], or T { data: … }).
  - Always use the type’s inherent constructors or macros: T::new(), T::from_vec(vec), or TSeqLit![…].
  - If the type lacks an inherent constructor, add one in its module (it may delegate internally), then update call sites to use it.
  - Keep any direct T { data: … } or vec![…] usage confined to the type’s own module/impls only (to preserve invariants and avoid leaks of representation).
  - UFCS constructors (<T as Trait>::new/…) are prohibited at call sites; prefer inherent or macro forms.


Rule: No Free-Function Wrappers
  - Do not create free functions that merely forward to trait or inherent methods (e.g., fn select(a,b,i) → <Type as Trait>::method). Call the method on the receiver instead.
  - If a method isn’t available on the receiver, add an extension trait implemented for the concrete type to expose value.method(...).
  - Try to keep UFCS only inside impls/traits for disambiguation; this should minimize UFCS at call sites.
#  - Allowed: free functions only when they add real semantics (compose multiple types, add logic not tied to a single receiver, or break dependency cycles).
#  - Do not duplicate method names as free functions; avoid two entry points for the same behavior.

Rule: Iterator impls (three forms)
- Implement all three `IntoIterator` forms for your sequence type.
- Provide inherent `iter()` and `iter_mut()` helpers that delegate to the backing collection.
- Avoid unnecessary bounds; add `T: Default` only if methods (e.g., `set`) require it.
- Add `ExactSizeIterator`/`DoubleEndedIterator` to iterator types when applicable.

#- Formatting and equality
#  - Implement `Display` with a clear, canonical representation.
#  - Ensure `Debug` is implemented (derive or manual).
#  - Ensure `PartialEq` and `Eq` are implemented/derived.

Rule: Tests Format 
  - one test function per function for all functions define in a modules traits.
  - EQ: Test the EQ. 
  - Iterator: Test the iterators.
  - Formatting:
    - `Display`: assert formatted string exactly.
    - `Debug`: assert it contains the type name or key structure (as appropriate).
  - assert_eq! on data values wherever possible.
  - Maximalize use of <MacroName>Lit![...] where ever possible, except for <MacroName>Lit![] which should be new().

Rule: Benches (criterion, short runs)
  - Provide representative iterator benches (e.g., `iter_sum_*`).
  - Configuration: warm-up ≤ 1s; measurement ≈ 6s; sample size ≈ 30; total ≤ 10s.

Rule: No Perl; awk acceptable
  - Use rg/grep; awk only if needed; never Perl.

Rule: Type Inference Cleanup
  - Avoid UFCS/turbofish unless required: don’t use `<Type as Trait>::method(...)` or `method::<T, _>(...)` at call sites if method-call syntax with the trait in scope suffices.
  - Prefer inferred bindings: use `let x = expr;` when the type is deducible from the expression or later usage in the same function.
  - Minimal annotations: when needed, use `let x: T = expr;` (keep the annotation local and narrow).
  - Constructors: prefer inherent constructors (e.g., `Type::new`, `Type::from_vec`) or *Lit! macros for literals. 
  - Numeric guidance: rely on defaults (`i32` for integers, `f64` for floats); add literal suffixes (e.g., `1_i64`, `0usize`) only when necessary.
  - Iteration ergonomics: favor `iter()/iter_mut()` and `for` loops over explicit `into_iter()` unless consuming ownership is intentional.
  - Eliminate redundant annotations: remove left-hand type annotations and turbofish where later statements already constrain the type.

Rule: Contain UFCS (call-site elimination)
  - Replace `<Type as Trait>::item(...)` at call sites with method-call syntax wherever possible.
  - Ensure traits are imported; add inherent constructors or small extension traits and impls if needed.
  - Maximally use UFCS only impls/traits for so that callers like test/* and bench/* don't have to use UFCS.

Rule: CPR rule (heartbeat lines)
  - Always write brief heartbeat lines (Thinking or planning) so progress is visible.

Rule: UFCS terminology and usage
  - Always refer to the `<Type as Trait>::item` form as UFCS (Universal Function Call Syntax).
  - Implicit methods may use `<X as Y>` syntax to allow maximal type inference.

Rule: Module import style for this project
  - In user modules, avoid importing individual symbols. Don’t use `use Foo::{Bar,Baz}`.
  - Prefer `use Foo::*` wherever possible.
  - The agent may not add a re-export of anything.
  - PartialEq and Eq definitions should be inside the file’s single module.

Rule: Module/file layout
  - Each file should have a single module.

Rule: Tooling messages
  - Print: `Applying patch_tool to <FILENAME>` or `Applying edit_tool to <FILENAME>` when performing edits. Do not print a shell transcript of how edits are applied.


Rule: Update todo status eagerly.  Update todo status eagerly: that is if it's done check
  it off right away and show the arrow on the next todo to state that it is in
  progress. And remember to check off that last todo when it is done.

Rule: Cargo Nextest:
 - Cargo nextest writes to stderr, so use 2>&1 | cat to catch it's output in a terminal window.

Rule: CamlCase not SnakeCase.
 - The user prefers all functions and structures of more than one english word,
  should be written in CamlCase.
 - functions of one word may be all lower case.
 - File name should be in CamlCase and start with a capitol.

Rule: do not load from rules or prompts files
 - The agent must not read files from the rules or promtps directory
  without being explicitly told to do so. Many rules and plans in
  there may be out of date.

Rule: emacs_sync
- **Trigger**: When you say you’ll “fix/edit in Emacs,” I pause all write edits immediately.
- **Pause behavior**: Do read-only work only (reviews/plans), no file edits until you say “Emacs: done”.
- **Status**: Acknowledge pause and note I’m waiting for “Emacs: done” to resume.
- **Resync on done**:
  - Propose to refresh repo state, then re-open changed files.
  - Suggested commands:
```bash
git status -s | cat
git diff --name-only | cat
```
- **Reconcile**: Prefer your Emacs changes; never overwrite. If conflicts arise, summarize diffs and ask how to proceed.
- **Style guarantees**: Keep existing indentation, whitespace, and your CamelCase module naming intact.
- **Scope hygiene**: Avoid global refactors while you’re editing; target only files you didn’t modify unless you approve.
- **Follow-ups**: After resync, run lint/diagnostics on changed files and resume the pending task.


Rule: Struct Field Encapsulation
- Default: struct fields are non-public; hide representation by default.
- Access via API: expose state through inherent methods and trait impls (e.g., `iter`, `iter_mut`, `len`, `as_slice`), not public fields.
- Construction: use constructors/macros; disallow struct literals outside the defining module.
- Exceptions: make a field public only with explicit user approval and documented invariants.
- Visibility scope: prefer private; use `pub(crate)` only when necessary and justified in docs.
- Testing: write tests against the public API, not internal fields.

Rule: Type Conversions and Naming
- Prefer traits over ad-hoc functions:
  - Implement `From<Src> for Dst` or `TryFrom<Src> for Dst`.
  - Call via `Dst::from(src)` or `src.into()`.
- Use `to_*` only when cloning/allocating an owned value is required.
  - Examples: `to_string`, `to_vec`, `to_owned`.
- Use `as_*` for cheap borrows/views with no allocation.
  - Examples: `as_str`, `as_slice`.
- Use `into_*` only when consuming `self` clarifies intent or returns a non-obvious type.
  - Examples: `into_inner`, `into_boxed_slice`.
- Do not add `to_Type` or extra `from_*` helpers if `From/Into/TryFrom/TryInto` suffices.
- Constructor exception: allow inherent `from_vec` where idiomatic; otherwise prefer trait-based conversions.


Rule: Test via Public API Only
  - Write tests against exposed methods/traits/macros; never against private fields.

Rule: Iterator-based Assertions
  - Use `iter()/iter_mut()/into_iter()` to assert sequence contents instead of exposing storage.

Rule: Minimal Constructor Surface
  - Prefer inherent constructors/macros (`new`, `from_vec`, `Lit![]`) over struct literals in tests.

Rule: Immediate Completion for Non‑Sweep TODOs
- Scope: Any TODO not under a Sweep plan.
- Execution: Run synchronously; clear terminal first; use non-interactive flags; pipe output via | cat.
- Completion: In the same message after success, give a 1–3 sentence summary, then mark the active TODO completed immediately.
- Next task: If another TODO exists, set it to in_progress and print its Start line.
- Last item: If no further TODOs remain, explicitly state all TODOs are completed.
- Failure: If a step fails, do not complete; print exact diagnostics and ask one minimal, concrete question.

#- Timestamps: Print Start/End/Total as assistant text only (use system clock).

Rule: TODO Completion Guard (Last Item)

- Scope: Applies to sweeps and non‑sweeps.

- Atomic order (same message):
  1) Print End: <timestamp> and Total: h:m:s (assistant text only).
  2) Provide a brief 1–3 sentence summary.
  3) Immediately mark the active TODO completed.
  4) If another TODO exists, set it in_progress and print its Start line; otherwise print “All TODOs completed”.

- No dangling active TODOs:
  - Never end a message with an active TODO unless it is marked failed and you are proceeding per the failure rules.

- End‑of‑message self‑check:
  - If all work for the active TODO is finished and it is still in_progress, complete it before sending the message.

- Long‑run exception:
  - Only background tasks explicitly tagged “[long‑run]”; don’t complete until stopped via the paired “Stop …” TODO. Exclude from totals until then.

Rule: Terminal Clear & Output Visibility
- Clear-first, separate calls: Run the terminal clear in its own invocation, then run the actual command in a new invocation. Never chain clear + command with &&.
  - Clear: printf '\033[3J\033[H\033[2J'   (no terminal echo needed; assistant won’t show timestamps here)
  - Then: <your command> 2>&1 | cat
- One command per terminal window: Do not combine multiple commands in one terminal call (except trivial redirections). This guarantees output renders in the chat terminal window.
- Pager safety: Always pipe command output via | cat.
- Exception: If you explicitly say “no-clear”, skip the clear step; still keep one command per terminal invocation.

Rule: Relentless TODO Flow (3‑Attempt, Non‑Blocking, Autopilot, Completion Guard)

- Scope
  - Applies to sweeps and non‑sweeps.

- Execution basics
  - Run synchronously.
  - Clear terminal (screen + scrollback) in a separate call; never chain clear + command.
  - Use non‑interactive flags; always pipe command output via “| cat”.
  - One command per terminal invocation (ensures output renders in the chat terminal window).
  - Timestamps are assistant text only, from the system clock; never print terminal timestamps or shell transcripts.

- Start / End / Total
  - Print “Start: <timestamp>” before the first attempt.
  - On success or after attempt 3, print “End: <timestamp>” and “Total: h:m:s”.
  - Provide a brief 1–3 sentence summary of what was done.

- Three attempts before asking
  - On any error in the active TODO, make up to 3 self‑directed fix attempts:
    - Adjust flags/filters, fix command syntax, add imports, apply minimal/local code edits, re‑run builds/tests.
  - Clear the terminal before each attempt.
  - Keep edits minimal and localized; no broad refactors during automatic attempts.

- Autopilot (no review pauses)
  - Never prompt for optional confirmations during TODOs or sweeps.
  - Proceed automatically to the next step/task unless blocked after 3 attempts or a fix would be destructive/ambiguous.

- Completion guard (eager completion)
  - In the same message and immediately after the summary:
    - Mark the active TODO completed.
    - If another TODO exists, set it in_progress and print its Start line; otherwise print “All TODOs completed”.
  - End‑of‑message self‑check: If the active TODO is finished but still in_progress, complete it before sending the message.
  - No dangling active TODOs; the only exception is explicit “[long‑run]”.

- Failure path (blocker)
  - After 3 failed attempts, or if a fix would be destructive/ambiguous:
    - Mark the TODO failed and print exact diagnostics.
    - Create a pending “Resume …” TODO.
    - Continue to the next independent TODO (sweeps do not pause).

- Long‑run tasks
  - Only run in the background if explicitly tagged “[long‑run]”.
  - Exclude from totals until stopped via a paired “Stop …” TODO; then include Actual.
  - Do not mark completed until the stop action finishes.

- Task granularity & hygiene
  - Prefer splitting multi‑step work into separate TODOs so each can be completed and marked immediately.
  - Keep outputs visible via “| cat”; avoid pagers; do not mix clear operations with commands; avoid background jobs except “[long‑run]”.

Rule: Pause Question
 - When the user asks if you are paused, tell them "Yes I am paused" or "No I am not".
 - In the Yes case, say why you stopped.
 - In the No case, detail what you are working on.
 - Don't just say "I'll continue with .." that does not answer the question and
   allow the user to understand and correct pauses in sweeps and other to-do
   processing.
 - Then continue with the task at hand or the to-dos.


Rule: Per‑TODO Execution (hard gate, every TODO)

- On setting a TODO in_progress:
  - Print Start via terminal:
    - date +"Start: %a %b %d %T %Z %Y" && date +%s

- Execute exactly one synchronous action (or a clearly batched set):
  - Edits and/or 1 command, piped with 2>&1 | cat.
  - If multiple attempts are needed, reuse the SAME Start (do not reset Start per attempt).

- On finishing the TODO (success or final failure):
  - Print End via terminal:
    - date +"End: %a %b %d %T %Z %Y" && date +%s
  - Compute and print Total: H:M:S as End_epoch − Start_epoch (includes think/idle/retries).
  - Print a 1–3 sentence summary.
  - Immediately:
    - Mark this TODO completed (or failed, if applicable).
    - If another TODO exists, set it in_progress and immediately print its Start line.

Rule: Sweep Planning and Execution (Incremental, 3‑Attempt, Autopilot, Metrics, Final Full Runs)

- Trigger
  - Plan when you say “Sweep”; do not execute until you say “execute”.
  - Fast Sweep: on at Sweep start; off at Sweep end.

- Plan & ETAs
  - Create “Sweep plan (sum of ETAs) [ETA HH:MM:SS]”.
  - Include per‑task ETAs in each TODO title; sum into EstimatedTotal.
  - Freeze ETAs at kickoff. If scope changes, create “Sweep plan (Rev N)” with updated EstimatedTotal and note the revision in final results.

- Execution protocol (synchronous, incremental; wall‑clock timestamps)
  - One command per terminal call; pipe via “2>&1 | cat”. Clear-first in a separate call only if the clear rule is active.
  - Timestamps (wall‑clock):
    - Start: when a TODO is set in_progress, print:
      - date +"Start: %a %b %d %T %Z %Y" && date +%s
    - End: immediately before completing the TODO, print:
      - date +"End: %a %b %d %T %Z %Y" && date +%s
    - Total: compute as End_epoch − Start_epoch (H:M:S), including think/idle/retries.
  - Summary‑first, immediate completion: After End/Total, print a 1–3 sentence summary, then mark the TODO completed. If another TODO exists, set it in_progress and immediately print its Start line.
  - No review pauses (autopilot): Never prompt for optional confirmations during sweeps; proceed automatically unless blocked after 3 attempts or a fix would be destructive/ambiguous.

- Incremental runs per change (narrowest possible compile)
  - src/<File>.rs:
    - If src/main.rs: cargo check --bin rustexp 2>&1 | cat
    - Otherwise (library): cargo check --lib 2>&1 | cat
    - In workspaces, add -p <package> as needed.
  - tests/<TestFile>.rs (compile only, no run):
    - Prefer: cargo nextest run --no-run --test <TestBinary> 2>&1 | cat
    - Alternative: cargo test --no-run --test <TestBinary> 2>&1 | cat
    - Note: digits-leading stems may be sanitized (often prefixed “_”).
  - benches/<BenchFile>.rs (compile only, no run):
    - cargo bench --no-run --bench <BenchBinary> 2>&1 | cat

- Failure handling & retries (3‑attempt, non‑blocking)
  - Up to 3 self‑directed attempts (flags/filters, minimal/local code edits, imports, re-run). Each attempt remains under the original Start; do not reset Start.
  - On success: include “Succeeded on attempt X/3” in the summary; complete and continue.
  - On persistent failure (3/3) or destructive/ambiguous fix: mark failed; print diagnostics; add “Resume sweep from <item>” pending TODO; continue to the next independent TODO.

- Final full runs (mandatory; each in its own terminal window)
  - cargo build
  - cargo nextest run 2>&1 | cat
  - cargo bench 2>&1 | cat
  - Apply the same Start/End/Total wall‑clock and summary‑first completion rules for each of these runs.

- Metrics & results
  - Per task: record Actual; report Diff = Actual − ETA and Error% = (Diff/ETA)×100 (1 decimal; “n/a” if EstimatedTotal=0).
  - Final TODO “Sweep results (totals + error)” must include EstimatedTotal, ActualTotal, Difference (signed), PercentError, and any plan revisions.

- Final‑results guard (no regressions)
  - Immediately after printing the final results block, complete the “Sweep results (totals + error)” TODO in the same message.
  - Self‑check: If all other sweep TODOs are complete and this isn’t, compute any missing fields (including PercentError) and complete it automatically.

- Long‑run tasks
  - Only “[long‑run]” runs in background; exclude from totals until stopped via paired “Stop …” TODO; then include Actual.
  - Fast Sweep: off at Sweep end.
  
Rule: Clearing TO-DOS

Intent:
- Clear the todo list instantly on request.

Protocol:
- Overwrite the entire list via the todo tool with merge=false and todos=[].
- No analysis or confirmation; clearing supersedes any in_progress items.
- After clearing, state “All TODOs cleared.” and stop.

Example:
- todo_write: { merge: false, todos: [] }


Clarified rule: Specialized import rules
- Inside `src/` (library code):
  - Use `crate::…` for all intra-crate paths.
  - Prefer wildcard imports for your modules: `use crate::Mod1::Mod1::*;`.
  - Macros exported at crate root: import with `use crate::FooSeqLit;` (or call as `crate::FooSeqLit![…]`).
- Outside the crate (`tests/`, `benches/`, `examples`, dependents):
  - Use the crate identifier derived from `[package] name` with hyphens replaced by underscores. Example: package `apas-ai` → `apas_ai` in code.
  - Prefer wildcard imports: `use apas_ai::Mod1::Mod1::*;`.
  - Macros: `use apas_ai::FooSeqLit;` then `FooSeqLit![…]`.
- Unit tests inside `src/` modules (`#[cfg(test)] mod tests`):
  - Treat as inside-crate: use `use crate::…` and `use crate::FooSeqLit;`.
- Never use `extern crate`. Do not add re-exports.

Small examples
Inside `src/SomeFeature.rs`:
```rust
use crate::Bag::Bag::*;      // wildcard import from your own module
use crate::BagSeqLit;        // macro exported at crate root

pub fn demo() {
    let _b = BagSeqLit![1, 2, 3];
}
```

Integration test `tests/test_bag.rs`:
```rust
use apas_ai::Bag::Bag::*;    // package name apas-ai → crate id apas_ai
use apas_ai::BagSeqLit;      // macro from crate root

#[test]
fn bag_macro_builds() {
    let b = BagSeqLit![42];
    assert_eq!(b.len(), 1);
}
```

Rule: Heartbeat Line Terminal
 - prepend every terminal line with:
 - date +'"Start: %a %b %d %T %Z %Y" 2>&1 | cat ;'
 - and postpend every terminal line with:
 - date +'"End: %a %b %d %T %Z %Y" 2>&1 | cat ;'
 - So I get a start/end time.


Rule: Non-terminal Step Timing (Edits/Searches/Planning)

Intent:
- Provide precise timing for non-terminal work (edits, reviews, searches, planning) on request.

Protocol:
- On request, bracket each non-terminal step with Start/End timestamps in the chat status updates.
- Obtain timestamps via a shell date call; print PDT only, no epoch.
  - Start line: date +"Start: %a %b %d %T %Z %Y"
  - End line:   date +"End: %a %b %d %T %Z %Y"
- One Start/End pair per distinct step; for multi-step sequences, repeat per step.
- Optionally include Total (H:M:S) after End if helpful.
- Do not run extra commands solely for timing unless timing has been requested.

Example (in chat status):
Start: Fri Sep 12 17:20:05 PDT 2025
[editing src/SetEphChap5_1.rs: normalize imports]
End: Fri Sep 12 17:22:11 PDT 2025


Rule: Tool Calls — Batching, Retries, Background
- Parallel: print a single "ToolCalls:" block, one line per tool in the batch.
- Retries: append "(attempt X/3)" to the tool line.
- Background: append "(background)" for background jobs.
- Clear policy: for shell commands, a clear-first

Rule: Tool Usage Transparency
- Announce every tool call before it runs (single line).
- Formats:
  - Tool: read_file — path: /abs/path/file.rs
  - Tool: edit_file — file: /abs/path/file.rs
  - Tool: apply_patch — file: /abs/path/file.rs
  - Tool: codebase_search — query: "…"; target: ["…"]
  - Tool: grep — pattern: '…'; path: /abs/path; type/glob: rs
  - Tool: todo_write — merge: true; items: N
  - Tool: read_lints — paths: ["…"]
  - Tool: list_dir — target: /abs/dir
  - Tool: glob_file_search — target: /abs/dir; glob: "**/*.rs"
  - Tool: run_terminal_cmd — command: "…"
- Redact secrets; truncate commands >160 chars.
- Stop announcements on STOP.


Rule: Verification Trace (imports & bounds hoist)
- For each file verification, print:
  - Verifying: <ABS-PATH>
  - Start: HH:MM:SS PDT
  - Checks (one line each):
    - imports: inside src use crate::; wildcard module imports; no extern crate
    - macros: exported at crate root; $crate paths; type-check helper present
    - bounds hoist: gather per-method bounds; hoist union to trait/impl; do not lift lifetimes; mirror impls
    - baseline: public APIs include Clone + Sized + Display + Debug at declaration sites
    - naming: CamlCase for multi-word items; files start with a capital
  - Tool calls announced via Tool Usage Transparency (read_file, grep/codebase_search, read_lints, etc.).
  - If divergence: print “Plan edit: …” then announce the edit tool call.
  - End: HH:MM:SS PDT; Total: H:MM:SS


Rule: Baseline Bounds via Element Trait (minimize repeated std bounds)
- Define a single baseline trait once, then use it everywhere instead of repeating bounds.
- Location: `crate::Types::Types`
- Definition:
```rust
pub trait Elem: Eq + std::hash::Hash + std::fmt::Display + std::fmt::Debug + Clone + Sized {}
impl<T> Elem for T where T: Eq + std::hash::Hash + std::fmt::Display + std::fmt::Debug + Clone + Sized {}
```
- Usage (inside src/): add at module top:
```rust
use crate::Types::Types::Elem;
use std::{fmt, hash, collections};
```
- Public APIs: prefer `X: Elem` (and `Y: Elem`) in trait/struct/impl headers.
- Method-specific extras (e.g., `Ord`, `Default`) go on that method’s where-clause only.

Examples:
- Before:
```rust
pub trait RelationEphChap5_2Trait<X: Eq + hash::Hash + fmt::Display + fmt::Debug + Clone + Sized,
                                  Y: Eq + hash::Hash + fmt::Display + fmt::Debug + Clone + Sized> { /* … */ }
```
- After:
```rust
pub trait RelationEphChap5_2Trait<X: Elem, Y: Elem> { /* … */ }
impl<X: Elem, Y: Elem> RelationEphChap5_2Trait<X, Y> for Relation<X, Y> { /* … */ }
```

Notes:
- Keep std imports module-top; no inline `std::…` in trait/impl bodies.
- Outside-crate (tests/benches): `use apas_ai::Types::Types::Elem;`.


```text
Delete these rules:
- Rule: Baseline Bounds via Element Trait (minimize repeated std bounds)
- Any other sections that introduce an “Elem” baseline trait

Replace/add the following full rules:

Rule: Standard Library Imports and Result usage (module-top; no aliasing)
- Put needed std imports at the top of each module. Don’t write std::… inside trait/impl bodies.
- Don’t alias std items. Import by their real names.
- Bounds in code should read minimally (see baseline rules below).
- Prefer importing frequently used std items (Iter, Formatter, etc.) rather than repeating long paths.
- Import order: after the module declaration add a blank line, then all use std::… lines, then a blank line, then crate::… imports.

Result guidance
- Formatting-only files (no generic Result<T, E>):
  - Import Display, Debug, Formatter, Result and use bare Result in fmt methods.
  - Example:
    use std::fmt::{Display, Debug, Formatter, Result};
    impl Display for Foo {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { /* … */ }
    }

- Files that also use generic Result<T, E>:
  - Do NOT import fmt::Result. Keep generic Result<T, E> bare for APIs, and use std::fmt::Result only in fmt methods.
  - Example:
    use std::fmt::{Display, Debug, Formatter};
    fn do_work() -> Result<u32, &'static str> { /* … */ }
    impl Display for Foo {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { /* … */ }
    }

Rule: Types and Bounds (Baseline + Minimal Additions)
- Baseline (project-wide): Public types, traits, and impls must bind element type parameters to
  Eq + Clone + Copy + Display + Debug + Sized at the declaration site (not on every method).
- Hash is opt‑in: Add Hash only when required by a specific API (e.g., HashMap/HashSet or hashing).
  Prefer the narrowest scope (a single method or specific impl). Do not hoist Hash to a trait header
  unless every item needs it.
- Minimal additions: Add extra bounds only when strictly required by stdlib contracts or called code,
  and keep them as local as possible (method/impl where‑clause).
- Associated types: When an associated type represents the element type, ensure it satisfies
  Eq + Clone + Copy + Display + Debug + Sized; add Hash only when that associated type is used in hashing contexts.

Example
  pub trait Foo<T: Eq + Clone + Copy + Display + Debug + Sized> {
      fn show(&self, x: T) -> String;
      fn bucket(&self, x: T) -> usize
      where
          T: std::hash::Hash; // Hash only where needed
  }
  impl<T: Eq + Clone + Copy + Display + Debug + Sized> Foo<T> for Bar<T> { /* … */ }

Rule: Type Creation Traits (align with baseline)
- For new public concrete types:
  - Derive Copy, Clone, Debug, PartialEq, Eq.
  - Implement Display.
  - Add Hash only if the type is used in hashed contexts.
- Sequence-like types (wrap or behave like a collection):
  - Provide iter() and iter_mut().
  - Implement IntoIterator for owned, &Self, and &mut Self.
  - Implement ExactSizeIterator/DoubleEndedIterator where applicable.

Example
  #[derive(Copy, Clone, Debug, PartialEq, Eq)]
  pub struct Point { x: i32, y: i32 }
  impl std::fmt::Display for Point {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
          write!(f, "({}, {})", self.x, self.y)
      }
  }
  // Derive Hash only if needed:
  // #[derive(Hash)]

Rule: Generalized lifting rule (applies to every trait/impl)

Baseline
- Project baseline for public element type parameters is:
  Eq + Clone + Copy + Display + Debug + Sized
- Bind the baseline at the declaration site (trait/struct/impl header). Do not repeat baseline bounds on every method.

What to hoist
- Hoist only bounds that are:
  - Common to all trait items’ signatures/where-clauses (the intersection), and
  - Satisfiable at all call sites.
- With this project’s baseline, Display/Debug are part of the baseline and are hoisted.
- Do NOT hoist Hash unless every item requires it (see “Method-only extras” below).

Method-only extras (keep local)
- Keep bounds that are only needed by specific items on those items:
  - Examples: Hash, Ord, Default, From/Into conversions, Borrow/AsRef, Send/Sync, iterator-specific bounds, etc.
- Prefer the narrowest scope:
  - First choice: the item’s own where-clause
  - Second choice: a specific impl block (only if all items in that impl need the bound)
  - Avoid putting such extras on the trait header unless truly common to all items.

Mirror in impls
- Mirror any hoisted bounds from the trait header exactly on every corresponding impl header.
- If you add an impl-wide extra bound because all items in that impl need it, put it on that impl header (not on the trait).

Associated types
- If a trait declares an associated “element” type, ensure it satisfies the baseline
  (Eq + Clone + Copy + Display + Debug + Sized). Add Hash only when used in hashing contexts.

Lifetimes and coherence
- Do not hoist lifetimes. Keep lifetime parameters where they logically belong.
- Validate hoisting against typical element types, macro empty forms, and existing blanket impls/coherence.

Unifying duplicates
- If multiple impl blocks for the same type/trait repeat identical bounds, unify them on the impl header instead of per-method duplication.
- Do not re-state baseline bounds on each method once they are on the header.

Examples

Before:
pub trait Foo<T> {
    fn show(&self, x: T) -> String
    where
        T: Eq + Clone + Copy + Display + Debug + Sized;

    fn bucket(&self, x: T) -> usize
    where
        T: Eq + Clone + Copy + Display + Debug + Sized + std::hash::Hash;
}

After (hoist baseline; keep Hash local):
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


