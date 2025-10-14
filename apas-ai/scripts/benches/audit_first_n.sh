#!/bin/bash
# Usage: audit_first_n.sh [N]
# Audits the first N benchmark files (default: 10)

N=${1:-10}

bench_files=($(find benches -name 'Bench*.rs' -type f | sort | head -n $N))

echo "Auditing first $N benchmark files"
echo "=================================="
echo ""

slow_count=0
timeout_count=0
ok_count=0

for i in "${!bench_files[@]}"; do
    file="${bench_files[$i]}"
    num=$((i + 1))
    echo -n "[$num/$N] $(basename $file): "
    
    result=$(scripts/benches/audit_one_benchmark.sh "$file" 2>&1)
    
    if echo "$result" | grep -q "TIMEOUT"; then
        echo "TIMEOUT"
        timeout_count=$((timeout_count + 1))
    elif echo "$result" | grep -q "SLOW"; then
        echo "SLOW"
        echo "$result" | grep "SLOW"
        slow_count=$((slow_count + 1))
    elif echo "$result" | grep -q "ERROR"; then
        echo "ERROR"
    else
        completed=$(echo "$result" | grep "Completed:" | awk '{print $2}')
        echo "OK ($completed benchmarks)"
        ok_count=$((ok_count + 1))
    fi
done

echo ""
echo "=================================="
echo "Summary:"
echo "  ✓ OK: $ok_count"
echo "  ⚠ SLOW: $slow_count"
echo "  ⏱ TIMEOUT: $timeout_count"
