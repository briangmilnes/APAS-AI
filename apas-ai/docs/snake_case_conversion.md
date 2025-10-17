# Snake Case Conversion Plan

## Overview

This document describes the conversion of all PascalCase and camelCase function/method names to snake_case throughout the APAS codebase, aligning with standard Rust naming conventions.

## Rationale

1. **Rust Convention Compliance**: Rust's official style guide (rustfmt) specifies snake_case for function and method names
2. **Consistency**: 95.5% of the codebase already uses snake_case (4,581 out of 4,795 functions)
3. **Tool Compatibility**: Better integration with Rust tooling (clippy, rust-analyzer, rustfmt)
4. **Community Standards**: Makes the code more familiar to Rust developers

## Updated APAS Naming Rules

The `rules/APASRules.md` has been updated with:

```markdown
### APAS Naming Conventions
- **Function and Method Names**: Use `snake_case` for all function and method names, following Rust conventions.
  - Examples: `from_vec`, `from_set`, `cartesian_product`, `is_empty`, `is_singleton`
  - Convert textbook notation to snake_case: N⁺(v) → `n_plus`, N⁻(v) → `n_minus`
- **Struct and Trait Names**: Use `PascalCase` for types, following Rust conventions.
- **APAS Text Fidelity**: When the APAS textbook specifies algorithm names, mirror the semantics exactly but apply Rust naming conventions.
```

## Conversion Examples

### Mathematical Operations (Chap05)
| Original (PascalCase) | New (snake_case) |
|----------------------|------------------|
| `FromVec` | `from_vec` |
| `FromSet` | `from_set` |
| `FromRelation` | `from_relation` |
| `CartesianProduct` | `cartesian_product` |

### Graph Operations (Chap06)
| Original (PascalCase) | New (snake_case) |
|----------------------|------------------|
| `FromSets` | `from_sets` |
| `NPlus` | `n_plus` |
| `NMinus` | `n_minus` |
| `NGOfVertices` | `ng_of_vertices` |
| `NPlusOfVertices` | `n_plus_of_vertices` |
| `NMinusOfVertices` | `n_minus_of_vertices` |
| `InDegree` | `in_degree` |
| `OutDegree` | `out_degree` |
| `Neighbor` | `neighbor` |
| `Incident` | `incident` |
| `Degree` | `degree` |

### Predicates (Chap17-19, Chap37)
| Original (camelCase) | New (snake_case) |
|---------------------|------------------|
| `isEmpty` | `is_empty` |
| `isSingleton` | `is_singleton` |

### Hash Table Operations (Chap47)
| Original (camelCase) | New (snake_case) |
|---------------------|------------------|
| `createTable` | `create_table` |
| `loadAndSize` | `load_and_size` |

### Utility Functions
| Original | New |
|----------|-----|
| `ArraySeqSetEq` | `array_seq_set_eq` |

## Impact Analysis

### Files Affected: 29 with mixed naming
- **Chap05**: 4 files (SetStEph, MappingStEph, RelationStEph, SetStEphClean)
- **Chap06**: 6 files (DirGraph, UnDirGraph, LabUnDirGraph variants)
- **Chap17-19**: 11 files (MathSeq, ArraySeq, LinkedList variants)
- **Chap37**: 4 files (AVLTreeSeq variants)
- **Chap47**: 2 files (ParaHashTableStEph, HashFunctionTraits)
- **Types.rs**: 1 file

### Functions to Convert
- **PascalCase**: 115 functions
- **camelCase**: 99 functions
- **Total**: 214 function name conversions

### What Gets Updated
1. Function definitions (`fn name`)
2. Method definitions in traits
3. Method implementations
4. Function/method calls (`.method()`)
5. UFCS-style calls (`Type::method()`)
6. Trait method calls (`<Type as Trait>::method()`)
7. Macro invocations

## Conversion Tools

### Two-Step Process

**Script 1: `scripts/rust/review_change_to_snake_case.py`**
- Reviews what needs to change
- Shows all conversions without modifying files
- Scans `src/`, `tests/`, and `benches/`

**Script 2: `scripts/rust/fix_to_snake_case.py`**
- Actually applies the changes
- Has confirmation prompt for safety
- Supports `--dry-run` flag for additional safety

**Usage:**

```bash
# Step 1: Review what will change (safe, read-only)
python scripts/rust/review_change_to_snake_case.py

# Step 2: Apply changes (requires confirmation)
python scripts/rust/fix_to_snake_case.py

# Or dry-run first for extra safety
python scripts/rust/fix_to_snake_case.py --dry-run
```

**Coverage:**
- ✅ `src/` directory (source code)
- ✅ `tests/` directory (unit and integration tests)
- ✅ `benches/` directory (benchmarks)

**Safety Features:**
- Separate review and fix scripts
- Confirmation prompt before applying
- Shows all conversions before applying
- Skips standard Rust trait methods (new, clone, etc.)
- Skips common function names (empty, size, etc.)

## Recommended Workflow

1. **Backup**: Commit all current changes
   ```bash
   git add -A
   git commit -m "Pre-conversion checkpoint"
   ```

2. **Review**: See what will change
   ```bash
   python scripts/rust/review_change_to_snake_case.py > conversion_review.txt
   ```

3. **Examine**: Read `conversion_review.txt`

4. **Apply**: Run conversion
   ```bash
   python scripts/rust/fix_to_snake_case.py
   ```

5. **Verify**: Check compilation
   ```bash
   cargo check
   ```

6. **Test**: Run test suite
   ```bash
   cargo test
   ```

7. **Review**: Examine changes
   ```bash
   git diff
   ```

8. **Commit or Revert**:
   - If successful: `git commit -m "Convert to snake_case naming convention"`
   - If issues: `git reset --hard HEAD`

## Testing Strategy

After conversion:

1. **Compilation**: `cargo check` should pass
2. **Unit Tests**: `cargo test` should pass
3. **Benchmarks**: `cargo bench --no-run` should compile
4. **Linting**: `cargo clippy` should show no new warnings
5. **Manual Review**: Check a few key files:
   - `src/Chap05/SetStEph.rs`
   - `src/Chap06/DirGraphStEph.rs`
   - `src/Chap18/ArraySeq.rs`

## Potential Issues and Solutions

### Issue 1: External Documentation References
**Problem**: Textbook references may use PascalCase  
**Solution**: Update comments to mention both forms or note the conversion

### Issue 2: Macro-generated Code
**Problem**: Some macros may generate function names  
**Solution**: Update macro definitions to use snake_case

### Issue 3: Test Assertions
**Problem**: Tests may assert on function names in error messages  
**Solution**: Update test expectations

## Rollback Plan

If issues arise:

```bash
# Quick rollback
git reset --hard HEAD

# Or if already committed
git revert HEAD
```

The conversion is fully automated and reversible through git.

## Post-Conversion Updates

After successful conversion:

1. Update any documentation that references old names
2. Check for comments that mention specific function names
3. Update any external documentation or tutorials
4. Notify team members of the naming convention change

## Benefits

1. **Standards Compliance**: Aligns with Rust RFC 430 naming conventions
2. **Tool Support**: Better rust-analyzer, clippy, and rustfmt behavior
3. **Consistency**: Single naming convention across entire codebase
4. **Readability**: Familiar patterns for Rust developers
5. **Maintainability**: Easier onboarding for new contributors

## Notes

- The script preserves the semantics of the APAS textbook while applying Rust conventions
- Mathematical notation like N⁺(v) naturally converts to `n_plus(v)`
- This change is purely stylistic and doesn't affect functionality
- All type names (PascalCase) remain unchanged

