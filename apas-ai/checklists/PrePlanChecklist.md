- [ ] Is the plan clearly understood? If not, get clarification from the user before proceeding.
- [ ] Are plan steps written as TODO items down to the specific files (or folders) that will be created or modified?
- [ ] Is it clear which new `src` files will be created and what each file should be named?
- [ ] Is it clear which existing `src` files the new work depends upon or must integrate with?
- [ ] Does the plan schedule creating each *Per file before the *Eph files?
- [ ] Does the plan schedule creating each file in its own step so files are generated one at a time?
- [ ] For every `src` file, does the plan list which methods will be implemented and which existing methods will be delegated to?
- [ ] If multiple `src` files are being added, are the tasks ordered bottom-up so incremental `cargo build` runs succeed after each file addition?
- [ ] Is it clear which data structures to create and their types?
- [ ] Do not use Vec for anything of known length, use a sequence type. 
- [ ] Does the plan plan know which functions and methods in an Mt module are to be parallel?
- [ ] Does the plan schedule running the RustRules and APAS checklists after each new `src` file is created or edited?
- [ ] Does the plan schedule `cargo build` after each new `src` file, performed one at a time until the build is clean (no warnings/errors)?
- [ ] Is it clear which test files will be created?
- [ ] Does the plan schedule running the RustRules and APAS checklists after each new test file is created or edited?
      It is crucial that these are run FILE BY FILE as it prevents you from injecting many defects.
      There should be a to-do item for this PER FILE.
- [ ] Are there uses of Vec that are of known length and should be a sequence type in the new files? Fix them.
      It is crucial that these are run FILE BY FILE as it prevents you from injecting many Vec defects.
      There should be a to-do item for this PER FILE.
- [ ] Does the plan schedule `cargo nextest` (targeting the new test file) after each test file is added, run one at a time until clean?
      There should be a to-do item for this PER FILE.
- [ ] Does the plan ensure each test file is added to Cargo.toml with a [[test]] declaration for test discovery?
      There should be a to-do item for this PER FILE.
- [ ] Is it clear which benchmark files will be created?
- [ ] Does the plan schedule running the RustRules and APAS checklists after each new benchmark file is created or edited?
- [ ] Does the plan schedule `cargo bench -bench <FILENAME> --no-run` (targeting the new benchmark file) after each benchmark is added, run one at a time until clean?
      There should be a to-do item for this PER FILE.
- [ ] Does the plan include a full `cargo build` at the end of the work?
- [ ] Does the plan include a full `cargo nextest` run at the end of the work?
- [ ] Does the plan reserve a final step to summarize the changes and call out any outstanding issues?
- [ ] Estimate the time to execute this plan.
- [ ] If the user says 'Execute relentlessly without pause' can you for this plan?
- [ ] If the user says check the todos on the first file, just execute them and ask for review. 
- [ ] Add a step of running the the AlgorithmicAnalaysis to the rules/AlgorithmicAnalysisRules.md for only src files.
- [ ] It is critical that the todo list is detailed to the file and each task on each file getting
     their own todo. 
- [ ] Add the last step of running the PostPlanChecklist.
- [ ] Show the plan to the user and wait for an execute command.
