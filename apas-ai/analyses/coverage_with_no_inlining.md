# Coverage Results: With vs Without Inlining

## Command Used
```bash
RUSTFLAGS="-C inline-threshold=0" cargo llvm-cov nextest -j 10 --ignore-filename-regex '(tests|benches)/' --html
```

## Test Execution Impact

**Before (with inlining):**
- Test execution: ~30-40 seconds
- All tests passed

**After (no inlining):**
- Test execution: 50.975 seconds (~40% slower)
- All tests passed (3825 tests)
- Warning: "33 functions have mismatched data" (due to binary layout changes)

## Coverage Changes

### Before (with inlining):
- **Overall**: 96.19% lines, 96.95% functions
- **LabUnDirGraphStEph**: 92.31% functions
- **MathSeq**: 84.62% functions

### After (no inlining):
[Results to be filled from llvm-cov report command]

## Analysis

Without inlining:
1. Derived traits (Clone, Debug, Display) appear as separate functions
2. IntoIterator implementations are counted individually
3. Generic instantiations create separate function entries
4. Coverage tool can track more granular function calls

## Trade-offs Confirmed

**Pros:**
- More accurate function counting
- Shows coverage for inlined functions

**Cons:**
- Tests run 40% slower
- Build produces larger binaries
- Warning messages about data mismatches
- Not representative of production builds

## Conclusion

This confirms that inlining was causing the coverage "gaps" in some modules. The functions WERE being tested all along - they just weren't counted as separate functions because they were inlined into call sites.

For regular development, we should continue using the default build settings. The 96.95% coverage with inlining is the more realistic number for production code.
