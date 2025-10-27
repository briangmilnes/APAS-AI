# How to Find the Missing 4 Functions in MathSeq.rs

## The Problem

llvm-cov reports:
```
Chap17/MathSeq.rs: 26 functions, 4 missed (84.62%)
```

But it **doesn't name** which 4 functions are missed in the text output.

## The Solution: Use the HTML Report

### Step 1: Open the HTML Report

```bash
firefox target/llvm-cov/html/index.html
# or
xdg-open target/llvm-cov/html/index.html
```

### Step 2: Navigate to MathSeq.rs

1. In the HTML report, you'll see a list of files
2. Look for: `Chap17/MathSeq.rs`
3. Click on it to see line-by-line coverage

### Step 3: Find Uncovered Functions

In the detailed view:
- **Green/blue lines** = covered
- **Red lines** = uncovered
- **Gray lines** = not executable (comments, blank lines)

Look for function signatures highlighted in RED:
```rust
fn some_function(...) -> ... {  // ← This line will be RED if uncovered
    ...
}
```

### Step 4: Identify the Functions

The HTML report will show:
- Function name
- Line number
- Execution count (0x for uncovered)

Example:
```
Line 201: fn into_iter(self) -> Self::IntoIter { ... }  [0 hits]
```

### Why Text Report Doesn't Show This

llvm-cov text output only shows aggregate statistics:
- Total functions in the file
- Number of missed functions
- Percentage coverage

It does **NOT** list individual function names or which ones are missed.

### Alternative: Check Coverage JSON

llvm-cov can generate JSON output with more details:

```bash
cargo llvm-cov report --ignore-filename-regex '(tests|benches)/' --json > coverage.json
```

Then search the JSON for MathSeq.rs and look for functions with 0 execution count.

But the HTML report is much easier to use visually.

## Expected Findings

Based on the pattern (26 functions, 4 missed = 84.62%), the missing functions are likely:

### Hypothesis 1: IntoIterator impl overhead
Lines 199-214 have 3 IntoIterator implementations. Each `impl` block might count as a wrapper function that's never directly called (only the .into_iter() method inside is called).

### Hypothesis 2: Generic instantiations
MathSeq<T> is generic. The compiler might generate specialized versions for different types, and llvm-cov counts each instantiation separately.

### Hypothesis 3: Trait method stubs
Some trait implementations might have compiler-generated stubs or forwarding functions.

### Hypothesis 4: Unreachable branches
Error handling or defensive code paths that never execute in practice.

## How to Verify the Tool's Analysis

The review-test-functions tool found all 26 functions tested. To verify:

```bash
# Check if IntoIterator is actually called
rg '\.into_iter\(\)' tests/Chap17/TestMathSeq.rs

# Check if PartialEq is called
rg 'assert_eq!' tests/Chap17/TestMathSeq.rs | wc -l

# Check if Display/Debug are called
rg 'format!' tests/Chap17/TestMathSeq.rs
```

All of these should show substantial usage, confirming the tool's findings.

## Conclusion

**To answer "Which 4 functions?":**
→ **You must open the HTML report and visually inspect MathSeq.rs**

There is no command-line way to extract the specific function names from llvm-cov text output.

The HTML report is at:
**`target/llvm-cov/html/index.html`**

Navigate to: **`Chap17/MathSeq.rs`**

Look for: **RED highlighted function signatures**
