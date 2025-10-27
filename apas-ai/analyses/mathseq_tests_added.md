# MathSeq Tests Added for Coverage Gaps

Based on the HTML coverage report showing `new` and the `into_iter` forms as uncovered.

## Tests Added

### 1. test_new_impl_via_trait (Line 314-321)
```rust
#[test]
fn test_new_impl_via_trait() {
    // Call the impl block's new directly via trait bound
    let seq = <MathSeqS<i32> as MathSeqTrait<i32>>::new(10, 99);
    assert_eq!(seq.length(), 10);
    for i in 0..10 {
        assert_eq!(*seq.nth(i), 99);
    }
}
```
**Target**: `impl MathSeqTrait<T> for MathSeqS<T>::new()` (line 102)

### 2. test_into_iter_by_ref_explicit (Line 324-332)
```rust
#[test]
fn test_into_iter_by_ref_explicit() {
    // Explicitly test IntoIterator for &MathSeqS
    let seq = MathSeqSLit![1, 2, 3, 4, 5];
    let mut sum = 0;
    for val in &seq {
        sum += val;
    }
    assert_eq!(sum, 15);
}
```
**Target**: `impl IntoIterator for &'a MathSeqS<T>::into_iter()` (line 201)

### 3. test_into_iter_by_mut_ref_explicit (Line 335-344)
```rust
#[test]
fn test_into_iter_by_mut_ref_explicit() {
    // Explicitly test IntoIterator for &mut MathSeqS
    let mut seq = MathSeqSLit![1, 2, 3];
    for val in &mut seq {
        *val *= 2;
    }
    assert_eq!(*seq.nth(0), 2);
    assert_eq!(*seq.nth(1), 4);
    assert_eq!(*seq.nth(2), 6);
}
```
**Target**: `impl IntoIterator for &'a mut MathSeqS<T>::into_iter()` (line 207)

### 4. test_into_iter_by_value_explicit (Line 347-352)
```rust
#[test]
fn test_into_iter_by_value_explicit() {
    // Explicitly test IntoIterator for MathSeqS (by value)
    let seq = MathSeqSLit![10, 20, 30];
    let collected: Vec<i32> = seq.into_iter().collect();
    assert_eq!(collected, vec![10, 20, 30]);
}
```
**Target**: `impl IntoIterator for MathSeqS<T>::into_iter()` (line 213)

## Expected Impact

### Before:
- Functions: 26 total, 4 missed → **84.62%**
- Missing: new (line 102), into_iter x3 (lines 201, 207, 213)

### After:
- Functions: 26 total, 0 missed → **100%** (hopefully!)
- All impl block functions explicitly exercised

## Why These Tests Were Needed

The existing tests called these functions indirectly:
- `MathSeqS::new()` was called 1,064 times via associated function syntax
- `.into_iter()` was called 54 times via `for` loops

But llvm-cov wasn't counting those as covering the impl block methods themselves.

These new tests call the functions in ways that llvm-cov recognizes:
- Via explicit trait bounds
- Via explicit `for val in &seq` syntax that desugars to into_iter()
- Via explicit `.into_iter().collect()` chains

## Status

✅ All 4 tests added
✅ All tests passing (32/32 in TestMathSeq)
⏳ Awaiting coverage regeneration to verify improvement
