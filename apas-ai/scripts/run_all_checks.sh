#!/bin/bash
# APAS Rules Validation Runner
# Runs all lint and convention checks

set -e  # Exit on first error

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"

echo "============================================"
echo "APAS Rules Validation"
echo "Project: $PROJECT_ROOT"
echo "============================================"
echo ""

# Track results
TOTAL_CHECKS=0
PASSED_CHECKS=0
FAILED_CHECKS=0

run_check() {
    local name="$1"
    local script="$2"
    
    TOTAL_CHECKS=$((TOTAL_CHECKS + 1))
    echo "[$TOTAL_CHECKS] Running $name..."
    
    if python3 "$script"; then
        PASSED_CHECKS=$((PASSED_CHECKS + 1))
        echo ""
    else
        FAILED_CHECKS=$((FAILED_CHECKS + 1))
        echo "FAILED: $name"
        echo ""
        return 1
    fi
}

# Run all checks
run_check "Naming Conventions" "scripts/APAS/src/review_naming.py"
run_check "Structure Checks" "scripts/APAS/src/review_structure.py"
run_check "Import Checks" "scripts/APAS/src/review_imports.py"
run_check "Convention Checks" "scripts/APAS/src/review_conventions.py"
run_check "Benchmark Timing Parameters" "scripts/APAS/benches/review_timing_params.py"

# Check for existing scripts
if [ -f "scripts/APAS/benches/review_duplicate_ids.py" ]; then
    run_check "Benchmark Duplicate IDs" "scripts/APAS/benches/review_duplicate_ids.py"
fi

if [ -f "scripts/APAS/benches/review_cargo_bench_names.py" ]; then
    run_check "Benchmark Cargo Registration" "scripts/APAS/benches/review_cargo_bench_names.py"
fi

echo "============================================"
echo "Results: $PASSED_CHECKS/$TOTAL_CHECKS checks passed"
if [ $FAILED_CHECKS -gt 0 ]; then
    echo "FAILED: $FAILED_CHECKS check(s) failed"
    exit 1
else
    echo "SUCCESS: All checks passed ✓"
    exit 0
fi

