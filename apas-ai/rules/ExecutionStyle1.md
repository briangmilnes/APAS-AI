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
- Clear-first, separate calls: if you need to clear the terminal, run that clear command in its own invocation, then run the actual command in a new invocation. Never chain clear + command with `&&`.
- One command per terminal window. Always pipe command output via `| cat`.
- Exception: if you explicitly say “no‑clear”, skip the clear step; still keep one command per terminal invocation.
- Present every command’s captured output inside fenced code blocks so the terminal formatting box appears consistently.
- Echo the output immediately after the command finishes; do not defer or batch command logs.

#### Command Timestamping
- At the start of each work batch, print the current time via `date +"%d %H:%M:%S %Z"` before issuing the first shell command.
- For any `cargo` command, emit an additional timestamp immediately beforehand using the same format.
- Show each timestamp in the transcript (code block), then execute the command as a separate invocation.
- Print the timestamp right away in the response so the user sees it before any subsequent command output.
- Avoid Start/End markers unless the user explicitly requests them.
- Do not emit extra timing commands beyond these requirements.

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
  - Drive mode: shorthand for the relentless execution posture—stay in motion without pausing for confirmation until you hit one of the two hard stops above.
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

#### Git Terminology Consistency
- Refer to repository state with `git` phrases: say `git-untracked`, `git-committed`, and `git-pushed`. Avoid substitutes such as “shipped.”

#### Mandatory Build Verification for Source Code Modifications
- **FINAL STEP REQUIREMENT**: The final steps of ANY source code modification MUST include:
  1. `cargo build` - MANDATORY for all `src/` changes
  2. `cargo nextest run` - MANDATORY if test code or functionality that affects tests was modified
- **NO EXCEPTIONS**: These verification steps are non-negotiable and must pass without warnings or errors
- **Failure Protocol**: If build/test fails, immediately fix the issues before declaring completion
- **TODO Completion Rule**: Mark TODOs as completed ONLY after successful build verification

#### Cargo Nextest
- `cargo nextest` writes to stderr; use `2>&1 | cat` to capture output.
- When matching Claude's workflow, run `cargo nextest --no-fail-fast --no-capture --color never 2>&1 | cat` so the full stream is visible while preserving identical options.
- When touching benchmark code or configs, begin with `cargo bench --no-run 2>&1 | cat` to surface compile issues quickly before full runs.

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

#### Claude and sed
- Claude is not allowed to run sed to attempt to fix bracket problems.

#### Git State Verification Before Assumptions
- **MANDATORY**: Before assuming code state or declaring regressions, ALWAYS check:
  1. `git status` - to see current working tree state
  2. `git log --oneline -5` - to verify recent commits/reverts
  3. File timestamps if needed - to understand when changes occurred
  4. `git show --stat HEAD` - to see what the last commit actually changed
- **Never assume** the user has reverted code without verifying git state first
- **Common scenario**: User may commit structural changes but not commit Claude's fixes, leaving fixes uncommitted and lost
- **Rule violation**: Making assumptions about code regressions without git verification is a workflow error

#### Pre-Sweep Git Commit Reminder (MANDATORY)
- **CRITICAL**: Before starting ANY sweep operation (vec! cleanup, macro replacement, refactoring, etc.), ALWAYS remind the user:
  - "REMINDER: Please run `git commit -am "checkpoint before sweep"` to save current progress before we begin the sweep"
  - "This prevents losing fixes if the sweep encounters issues or needs to be reverted"
- **Wait for confirmation**: Do not proceed with sweep until user confirms they have committed or explicitly says to proceed
- **Rationale**: Sweeps can cause massive regressions if uncommitted fixes are lost during the process
- **SEV1 violation**: Starting a sweep without this reminder and confirmation is a critical workflow error

#### Reordering or changing the task list
- **CRITICAL**: Do not stop and ask to reorder a task list that the user has OK'd.
- In a sweep if you have a faster way to do the job you may propose it to the user.

#### Tool parallelism
- Run cargo bench -j 10, too many rustcs are being forked.

