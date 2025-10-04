# Python Execution Rule

## MANDATORY PROTOCOL FOR PYTHON SCRIPTS

### 🚨 CRITICAL REQUIREMENT
**Claude MUST NEVER run a Python script that modifies files without explicit user approval and proper testing protocol.**

### 📋 REQUIRED PROCESS

#### 1. **ASK FIRST**
- **ALWAYS ask the user for permission** before running any Python script that modifies files
- Explain what the script will do and why it's needed
- Get explicit approval before proceeding

#### 2. **TEST ON SINGLE FILE FIRST**
- **NEVER run on entire codebase initially**
- Test on ONE representative file first
- Verify the script works correctly on the test case
- Show results to user for approval

#### 3. **INCREMENTAL VALIDATION**
After single-file success:
- **Build**: Run `cargo check` or `cargo build`
- **Test**: Run `cargo test` for affected modules
- **Compile benches**: Run `cargo bench --no-run` if benchmarks are modified
- **Verify**: Ensure no regressions introduced

#### 4. **SCOPE-APPROPRIATE TESTING**
Based on what the Python script modifies:

| **Modification Type** | **Required Testing** |
|----------------------|---------------------|
| Source files (`src/`) | `cargo check` + `cargo test` |
| Test files (`tests/`) | `cargo test --no-run` + specific test runs |
| Benchmark files (`benches/`) | `cargo bench --no-run` |
| Multiple file types | All applicable tests above |
| Configuration files | Full build + test cycle |

### ⚠️ CONSEQUENCES OF VIOLATION
- **Lost development time** (as experienced: 2 hours)
- **Broken codebase** requiring extensive manual fixes
- **Compilation errors** cascading across modules
- **Test failures** requiring individual debugging

### ✅ ACCEPTABLE ALTERNATIVES
Instead of Python scripts, prefer:
1. **Manual targeted fixes** for small changes
2. **Rust tooling** (`cargo fix`, `rustfmt`, etc.)
3. **sed/awk** for simple text transformations
4. **Interactive approval** for each change when using scripts

### 🎯 GOAL
Prevent mass automated changes that break the codebase and require extensive manual recovery.

---
**Rule Created**: In response to 2-hour debugging session caused by automated Python script modifications
**Severity**: CRITICAL - Must be followed without exception
**Scope**: Applies only to Python scripts that modify files (write, edit, delete operations). Read-only Python scripts for analysis or search are permitted without approval.
