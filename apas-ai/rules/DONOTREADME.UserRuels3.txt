APAS User Rules — Consolidated (UserRuels3)

Imports and Module Scope

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

Rule: Specialized import rules
- Inside src/ (library code):
  - Use crate::… for all intra-crate paths.
  - Prefer wildcard imports for your own modules: use crate::Mod1::Mod1::*;
  - Macros exported at crate root: import with use crate::FooSeqLit; (or call as crate::FooSeqLit![…]).
- Outside the crate (tests/, benches/, examples/, dependents):
  - Use the crate identifier derived from [package] name with hyphens replaced by underscores. Example: package apas-ai → apas_ai in code.
  - Prefer wildcard imports: use apas_ai::Mod1::Mod1::*;
  - Macros: use apas_ai::FooSeqLit; then FooSeqLit![…].
- Unit tests inside src/ modules (#[cfg(test)] mod tests):
  - Treat as inside-crate: use crate::… and use crate::FooSeqLit;.
- Never use extern crate. Do not add re-exports.

Rule: Module import style for this project
- In user modules, avoid importing individual symbols. Don’t use use Foo::{Bar,Baz}.
- Prefer use Foo::* wherever possible (applies to your own modules; std items should be imported by name, without aliasing).
- PartialEq and Eq definitions should be inside the file’s single module.

Rule: Module/file layout
- Each file should have a single module.


Types, Bounds, and Lifting

Rule: Types and Bounds (Baseline + Minimal Additions)
- Baseline (project‑wide): Public types, traits, and impls must bind element type parameters to Eq + Clone + Copy + Display + Debug + Sized at the declaration site (not on every method).
- Hash is opt‑in: Add Hash only when required by a specific API (e.g., HashMap/HashSet or hashing). Prefer the narrowest scope (a single method or specific impl). Do not hoist Hash to a trait header unless every item needs it.
- Minimal additions: Add extra bounds only when strictly required by stdlib contracts or called code, and keep them as local as possible (method/impl where‑clause).
- Associated types: When an associated type represents the element type, ensure it satisfies Eq + Clone + Copy + Display + Debug + Sized; add Hash only when that associated type is used in hashing contexts.

Rule: Generalized lifting rule (applies to every trait/impl)

Baseline
- Project baseline for public element type parameters is: Eq + Clone + Copy + Display + Debug + Sized
- Bind the baseline at the declaration site (trait/struct/impl header). Do not repeat baseline bounds on every method.

What to hoist
- Hoist only bounds that are:
  - Common to all trait items’ signatures/where-clauses (the intersection), and
  - Satisfiable at all call sites.
- With this project’s baseline, Display/Debug are part of the baseline and are hoisted.
- Do NOT hoist Hash unless every item requires it (see “Method‑only extras” below).

Method‑only extras (keep local)
- Keep bounds that are only needed by specific items on those items:
  - Examples: Hash, Ord, Default, From/Into conversions, Borrow/AsRef, Send/Sync, iterator‑specific bounds, etc.
- Prefer the narrowest scope:
  - First choice: the item’s own where‑clause
  - Second choice: a specific impl block (only if all items in that impl need the bound)
  - Avoid putting such extras on the trait header unless truly common to all items.

Mirror in impls
- Mirror any hoisted bounds from the trait header exactly on every corresponding impl header.
- If you add an impl‑wide extra bound because all items in that impl need it, put it on that impl header (not on the trait).

Associated types
- If a trait declares an associated “element” type, ensure it satisfies the baseline (Eq + Clone + Copy + Display + Debug + Sized). Add Hash only when used in hashing contexts.

Lifetimes and coherence
- Do not hoist lifetimes. Keep lifetime parameters where they logically belong.
- Validate hoisting against typical element types, macro empty forms, and existing blanket impls/coherence.

Unifying duplicates
- If multiple impl blocks for the same type/trait repeat identical bounds, unify them on the impl header instead of per‑method duplication.
- Do not re‑state baseline bounds on each method once they are on the header.

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

Rule: Type Creation Traits (align with baseline)
- For new public concrete types:
  - Derive Copy, Clone, Debug, PartialEq, Eq.
  - Implement Display.
  - Add Hash only if the type is used in hashed contexts.
- Sequence‑like types (wrap or behave like a collection):
  - Provide iter() and iter_mut().
  - Implement IntoIterator for owned, &Self, and &mut Self.
  - Implement ExactSizeIterator/DoubleEndedIterator where applicable.


APIs, Macros, Constructors, Encapsulation

Rule: Macro Normalization (Exported and Type‑Checked)
- Define at crate root:
  - Use #[macro_export] and place macro_rules! at crate root (end of the module file).
  - Inside the macro body, use $crate:: fully qualified paths to all types/functions.
- One definition:
  - No module‑local duplicates or wrappers; one canonical macro per type family.
- Call‑site ergonomics:
  - Non‑empty forms rely on element types (ints default to i32); empty forms require a minimal type annotation at the call site.
  - Import macros from the crate root in tests/benches with use my_crate::MacroName; inside src you can use crate::MacroName or invoke directly.
- Macros are all pub; used to make datatype literals (Lit!).
- Dead‑code type‑check helper (required; must include empty form):
```rust
#[allow(dead_code)]
fn _MyMacro_type_checks() {
    let _ = MyMacro![1];               // non‑empty infers (e.g., i32)
    let _: MyType<i32> = MyMacro![];   // empty form requires explicit type
}
```
- Naming: Keep macro names consistent and descriptive (e.g., FooSeqLit!), aligned with the type they construct.

Rule: Constructor No Raw Backing Collections
- Never construct sequence types via raw backing collections at call sites (e.g., Vec::new, vec![…], or T { data: … }).
- Always use the type’s inherent constructors or macros: T::new(), T::from_vec(vec), or TSeqLit![…].
- If the type lacks an inherent constructor, add one in its module (it may delegate internally), then update call sites to use it.
- Keep any direct T { data: … } or vec![…] usage confined to the type’s own module/impls only (preserve invariants; avoid representation leaks).
- UFCS constructors (<T as Trait>::new/…) are prohibited at call sites; prefer inherent or macro forms.

Rule: Struct Field Encapsulation
- Default: struct fields are non‑public; hide representation by default.
- Access via API: expose state through inherent methods and trait impls (e.g., iter, iter_mut, len, as_slice), not public fields.
- Construction: use constructors/macros; disallow struct literals outside the defining module.
- Exceptions: make a field public only with explicit user approval and documented invariants.
- Visibility scope: prefer private; use pub(crate) only when necessary and justified in docs.
- Testing: write tests against the public API, not internal fields.

Rule: No Free‑Function Wrappers
- Do not create free functions that merely forward to trait or inherent methods (e.g., fn select(a,b,i) → <Type as Trait>::method). Call the method on the receiver instead.
- If a method isn’t available on the receiver, add an extension trait implemented for the concrete type to expose value.method(...).
- Allowed: free functions only when they add real semantics (compose multiple types, add logic not tied to a single receiver, or break dependency cycles). Do not duplicate method names as free functions; avoid two entry points for the same behavior.

Rule: Type Conversions and Naming
- Prefer traits over ad‑hoc functions:
  - Implement From<Src> for Dst or TryFrom<Src> for Dst.
  - Call via Dst::from(src) or src.into().
- Use to_* only when cloning/allocating an owned value is required (e.g., to_string, to_vec, to_owned).
- Use as_* for cheap borrows/views with no allocation (e.g., as_str, as_slice).
- Use into_* only when consuming self clarifies intent or returns a non‑obvious type (e.g., into_inner, into_boxed_slice).
- Do not add to_Type or extra from_* helpers if From/Into/TryFrom/TryInto suffices.
- Constructor exception: allow inherent from_vec where idiomatic; otherwise prefer trait‑based conversions.


Iteration, Iterators, and Tests

Rule: Iterator impls (three forms)
- Implement all three IntoIterator forms for your sequence type (owned, &Self, &mut Self).
- Provide inherent iter() and iter_mut() helpers that delegate to the backing collection.
- Avoid unnecessary bounds; add T: Default only if methods (e.g., set) require it.
- Add ExactSizeIterator when length is known in O(1) and stable during iteration.
- Add DoubleEndedIterator when items can be traversed from both ends without extra allocation or violating semantics.

Rule: Iterator‑based Assertions
- Use iter()/iter_mut()/into_iter() to assert sequence contents instead of exposing storage.

Rule: Tests Format
- One test per public function/trait item in a module (include iterator and formatting coverage).
- Equality: test PartialEq/Eq behavior explicitly where defined.
- Iterator tests: cover forward and reverse traversal where applicable; assert lengths for ExactSizeIterator.
- Formatting:
  - Display: assert exact formatted string.
  - Debug: assert it contains the type name or key structure elements, as appropriate.
- Prefer assert_eq! on data values wherever possible.
- Prefer <MacroName>Lit![…] for non‑empty literals; use T::new() for empty cases instead of <MacroName>Lit![].

Rule: Test via Public API Only
- Write tests against exposed methods/traits/macros; never against private fields.

Rule: Benches (criterion, short runs)
- Provide representative iterator benches (e.g., iter_sum_*).
- Configuration: warm‑up ≤ 1s; measurement ≈ 6s; sample size ≈ 30; total ≤ 10s.


Naming and Inference

Rule: CamlCase not SnakeCase
- The user prefers all functions and structures of more than one English word to be written in CamlCase.
- Functions of one word may be all lower case.
- File names should be in CamlCase and start with a capital.

Rule: Type Inference Cleanup
- Avoid UFCS/turbofish unless required: don’t use <Type as Trait>::method(...) or method::<T, _>(...) at call sites if method‑call syntax with the trait in scope suffices.
- Prefer inferred bindings: use let x = expr; when the type is deducible from the expression or later usage in the same function.
- Minimal annotations: when needed, use let x: T = expr; (keep the annotation local and narrow).
- Constructors: prefer inherent constructors (e.g., Type::new, Type::from_vec) or Lit! macros for literals.
- Numeric guidance: rely on defaults (i32 for integers, f64 for floats); add literal suffixes (e.g., 1_i64, 0usize) only when necessary.
- Iteration ergonomics: favor iter()/iter_mut() and for loops over explicit into_iter() unless consuming ownership is intentional.
- Eliminate redundant annotations: remove left‑hand type annotations and turbofish where later statements already constrain the type.

Rule: Contain UFCS (call‑site elimination)
- Replace <Type as Trait>::item(...) at call sites with method‑call syntax wherever possible.
- Ensure traits are imported; add inherent constructors or small extension traits and impls if needed.
- Keep UFCS inside impls/traits for disambiguation; minimize UFCS in callers (tests/benches should not need UFCS).


Tooling, Execution Protocol, and Transparency

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

Rule: Tool Calls — Batching, Retries, Background
- Parallel: print a single "ToolCalls:" block, one line per tool in the batch.
- Retries: append "(attempt X/3)" to the tool line.
- Background: append "(background)" for background jobs.
- Clear policy: for shell commands, always use a clear‑first in a separate invocation before running the command.

Rule: Terminal Clear & Output Visibility
- Clear‑first, separate calls: run the terminal clear in its own invocation, then run the actual command in a new invocation. Never chain clear + command with &&.
  - Clear: printf '\033[3J\033[H\033[2J'
  - Then: <your command> 2>&1 | cat
- One command per terminal window: do not combine multiple commands in one terminal call (except trivial redirections). This guarantees output renders in the chat terminal window.
- Pager safety: always pipe command output via | cat.
- Exception: if you explicitly say "no‑clear", skip the clear step; still keep one command per terminal invocation.

Rule: Non‑terminal Step Timing (Edits/Searches/Planning)
- Provide precise timing for non‑terminal work on request.
- On request, bracket each non‑terminal step with Start/End timestamps in chat status updates.
- Obtain timestamps via a shell date call; use system timezone unless PDT is explicitly requested.
  - Start line: date +"Start: %a %b %d %T %Z %Y"
  - End line:   date +"End: %a %b %d %T %Z %Y"
- One Start/End pair per distinct step; for multi‑step sequences, repeat per step.
- Optionally include Total (H:M:S) after End if helpful.
- Do not run extra commands solely for timing unless timing has been requested.

Rule: Verification Trace (imports & bounds hoist)
- For each file verification, print:
  - Verifying: <ABS‑PATH>
  - Start: HH:MM:SS (system TZ)
  - Checks (one line each):
    - imports: inside src use crate::; wildcard module imports; no extern crate
    - macros: exported at crate root; $crate paths; type‑check helper present
    - bounds hoist: gather per‑method bounds; hoist union to trait/impl; do not lift lifetimes; mirror impls
    - baseline: public APIs include Clone + Sized + Display + Debug at declaration sites
    - naming: CamlCase for multi‑word items; files start with a capital
  - Tool calls announced via Tool Usage Transparency (read_file, grep/codebase_search, read_lints, etc.).
  - If divergence: print "Plan edit: …" then announce the edit tool call.
  - End: HH:MM:SS (system TZ); Total: H:MM:SS

Rule: emacs_sync
- Trigger: when you say you’ll "fix/edit in Emacs," pause all write edits immediately.
- Pause behavior: do read‑only work only (reviews/plans), no file edits until you say "Emacs: done".
- Status: acknowledge pause and note waiting for "Emacs: done" to resume.
- Resync on done:
  - Propose to refresh repo state, then re‑open changed files.
  - Suggested commands:
```bash
git status -s | cat
git diff --name-only | cat
```
- Reconcile: prefer your Emacs changes; never overwrite. If conflicts arise, summarize diffs and ask how to proceed.
- Style guarantees: keep existing indentation, whitespace, and your CamelCase module naming intact.
- Scope hygiene: avoid global refactors while you’re editing; target only files you didn’t modify unless you approve.
- Follow‑ups: after resync, run lint/diagnostics on changed files and resume the pending task.

Rule: do not load from rules or prompts files
- Do not read files from the rules or prompts directory without being explicitly told to do so; many rules and plans there may be out of date.


TODO Flow and Execution Discipline

Rule: Relentless TODO Flow (3‑Attempt, Non‑Blocking, Autopilot, Completion Guard)
- Scope: applies to sweeps and non‑sweeps.
- Execution basics:
  - Run synchronously.
  - Clear terminal (screen + scrollback) in a separate call; never chain clear + command.
  - Use non‑interactive flags; always pipe command output via "| cat".
  - One command per terminal invocation (ensures output renders in the chat terminal window).
  - Timestamps are assistant text only from the system clock; never print terminal timestamps or shell transcripts.
- Start / End / Total:
  - Print "Start: <timestamp>" before the first attempt.
  - On success or after attempt 3, print "End: <timestamp>" and "Total: h:m:s".
  - Provide a brief 1–3 sentence summary of what was done.
- Three attempts before asking:
  - On any error in the active TODO, make up to 3 self‑directed fix attempts (adjust flags/filters, fix command syntax, add imports, minimal/local code edits, re‑run builds/tests). Clear the terminal before each attempt. Keep edits minimal and localized; no broad refactors during automatic attempts.
- Autopilot (no review pauses):
  - Never prompt for optional confirmations during TODOs or sweeps; proceed automatically unless blocked after 3 attempts or a fix would be destructive/ambiguous.
- Completion guard (eager completion):
  - In the same message and immediately after the summary: mark the active TODO completed. If another TODO exists, set it in_progress and print its Start line; otherwise print "All TODOs completed". End‑of‑message self‑check: if the active TODO is finished but still in_progress, complete it before sending the message. No dangling active TODOs; the only exception is explicit "[long‑run]".
- Failure path (blocker):
  - After 3 failed attempts, or if a fix would be destructive/ambiguous: mark the TODO failed and print exact diagnostics; create a pending "Resume …" TODO; continue to the next independent TODO.
- Long‑run tasks:
  - Only run in the background if explicitly tagged "[long‑run]"; exclude from totals until stopped via a paired "Stop …" TODO; then include Actual; do not mark complete until the stop action finishes.
- Task granularity & hygiene: prefer splitting multi‑step work into separate TODOs so each can be completed and marked immediately.

Rule: Per‑TODO Execution (hard gate, every TODO)
- On setting a TODO in_progress: print Start via terminal:
  - date +"Start: %a %b %d %T %Z %Y" && date +%s
- Execute exactly one synchronous action (or a clearly batched set): edits and/or 1 command, piped with 2>&1 | cat. If multiple attempts are needed, reuse the SAME Start (do not reset Start per attempt).
- On finishing the TODO (success or final failure): print End via terminal:
  - date +"End: %a %b %d %T %Z %Y" && date +%s
- Compute and print Total: H:M:S as End_epoch − Start_epoch (includes think/idle/retries). Provide a brief 1–3 sentence summary. Immediately mark this TODO completed (or failed). If another TODO exists, set it in_progress and immediately print its Start line.

Rule: TODO Completion Guard (Last Item)
- Scope: applies to sweeps and non‑sweeps.
- Atomic order (same message):
  1) Print End: <timestamp> and Total: h:m:s (assistant text only).
  2) Provide a brief 1–3 sentence summary.
  3) Immediately mark the active TODO completed.
  4) If another TODO exists, set it in_progress and print its Start line; otherwise print "All TODOs completed".
- No dangling active TODOs. End‑of‑message self‑check applies.
- Long‑run exception: only background tasks explicitly tagged "[long‑run]"; don’t complete until stopped via the paired "Stop …" TODO. Exclude from totals until then.

Rule: Clearing TO‑DOS
- Purpose: clear the todo list instantly on request.
- Protocol: overwrite the entire list via the todo tool with merge=false and todos=[]. No analysis or confirmation; clearing supersedes any in_progress items. After clearing, state "All TODOs cleared." and stop.

Rule: Update todo status eagerly
- If a TODO is done, check it off immediately; set the next TODO to in_progress.

Rule: Pause Question
- When asked if you are paused, answer "Yes I am paused" or "No I am not".
- If Yes: say why you stopped.
- If No: detail what you are working on.
- Then continue with the task at hand or the to‑dos.


Environment, Tools, and Misc

Rule: Cargo Nextest
- Cargo nextest writes to stderr; use 2>&1 | cat to capture its output in a terminal window.

Rule: No Perl; awk acceptable
- Use rg/grep; awk only if needed; never Perl.

Rule: CPR rule (heartbeat lines)
- Always write brief heartbeat lines (e.g., "Thinking" or "Planning") so progress is visible. At minimum: before a tool batch, after edits/commands, and at completion.

Rule: Minimal Constructor Surface
- Prefer inherent constructors/macros (new, from_vec, Lit![]) over struct literals in tests.


