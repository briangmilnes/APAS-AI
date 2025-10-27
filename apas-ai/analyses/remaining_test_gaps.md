# Remaining Test Coverage Gaps

After tool improvements, only **10 functions** remain uncovered (down from 191!)

## Analysis of Remaining 10 Functions

### Category 1: Display fmt() - 6 functions (likely FALSE POSITIVES)

**Quadruple::fmt** (src/Types.rs:257)
- Status: ✅ **ALREADY TESTED** in tests/TestTypes.rs
- Test: `test_quadruple_display()` uses `format!("{}", quad)`
- Conclusion: Tool still not detecting this specific case

**KeyVal::fmt** (src/Types.rs:262)
- Status: ✅ **ALREADY TESTED** in tests/TestTypes.rs
- Test: `test_keyval_display()` uses `format!("{}", kv)`
- Conclusion: Tool still not detecting this specific case

**KeyProb::fmt** (4 files in Chap50)
- src/Chap50/OptBinSearchTreeMtEph.rs:298
- src/Chap50/OptBinSearchTreeMtPer.rs:228
- src/Chap50/OptBinSearchTreeStEph.rs:203
- src/Chap50/OptBinSearchTreeStPer.rs:165
- Status: ❓ KeyProb is used in tests but format!() not called
- Conclusion: Genuinely not tested via Display

### Category 2: Operator traits - 4 functions (likely FALSE POSITIVES)

**Probability arithmetic** (src/Chap50/Probability.rs)
- Line 118: `Probability::add` (impl Add)
- Line 124: `Probability::sub` (impl Sub)
- Line 130: `Probability::mul` (impl Mul)
- Line 136: `Probability::div` (impl Div)
- Status: ❓ Need to check if +,-,*,/ operators are used in tests
- Conclusion: Need verification

## Action Plan

### 1. Verify False Positives (2 functions)
Check if Quadruple/KeyVal Display detection is a tool bug.

### 2. Add Genuine Tests (8 functions)

**Priority: HIGH - KeyProb Display (4 files)**
Add tests to call `format!("{}", key_prob)` for KeyProb structs.

**Priority: MEDIUM - Probability operators (4 functions)**
If not already tested, add tests using +, -, *, / operators on Probability values.

## Test Plan Details

### Test 1: KeyProb Display (4 files)
Files to modify:
- tests/Chap50/TestOBSTMtEph.rs
- tests/Chap50/TestOBSTMtPer.rs
- tests/Chap50/TestOptBinSearchTreeStEph.rs
- tests/Chap50/TestOptBinSearchTreeStPer.rs

Add to each:
```rust
#[test]
fn test_keyprob_display() {
    let kp = KeyProb {
        key: 1,
        prob: Probability::new(0.5),
    };
    let display_str = format!("{}", kp);
    assert!(display_str.contains("1"));
    assert!(display_str.contains("0.5"));
}
```

### Test 2: Probability Operators (1 file)
File: tests/Chap50/TestProbability.rs (create if doesn't exist)

```rust
#[test]
fn test_probability_add() {
    let p1 = Probability::new(0.3);
    let p2 = Probability::new(0.4);
    let sum = p1 + p2;  // Tests Add trait
    assert_eq!(sum.value(), 0.7);
}

#[test]
fn test_probability_sub() {
    let p1 = Probability::new(0.7);
    let p2 = Probability::new(0.3);
    let diff = p1 - p2;  // Tests Sub trait
    assert_eq!(diff.value(), 0.4);
}

#[test]
fn test_probability_mul() {
    let p1 = Probability::new(0.5);
    let p2 = Probability::new(0.4);
    let prod = p1 * p2;  // Tests Mul trait
    assert_eq!(prod.value(), 0.2);
}

#[test]
fn test_probability_div() {
    let p1 = Probability::new(0.8);
    let p2 = Probability::new(0.4);
    let quot = p1 / p2;  // Tests Div trait
    assert_eq!(quot.value(), 2.0);
}
```

### Test 3: Verify Quadruple/KeyVal (already done)
No action needed - already tested, tool bug.

## Expected Outcome

After adding these tests:
- **Actual uncovered**: 10 → 2 (just Quadruple/KeyVal tool bugs)
- **Reported uncovered**: 10 → 0 (if tool detects new tests)
- **True coverage**: ~99.6% → ~99.9%

## Tool Improvement Needed

The tool still doesn't detect:
- Display tests for structs defined in src/Types.rs but tested in tests/TestTypes.rs
- This might be a path/module matching issue
