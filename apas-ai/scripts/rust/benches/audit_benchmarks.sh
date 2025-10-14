#!/bin/bash
# Audit benchmarks: run first 50 files with 2*B timeout
# Reports which specific benchmarks exceed 1.5s estimated time

echo "Benchmark Audit - First 50 Files (2*B timeout)"
echo "Shows all benchmark times and flags slow ones (>1.5s)"
echo "========================================================="

total=0
ok=0
slow=0
timeout=0
error=0

while IFS= read -r bench_file; do
    bench_name=$(basename "$bench_file" .rs)
    total=$((total + 1))
    
    B=$(python3 scripts/benches/count_benchmarks.py "$bench_file")
    timeout_sec=$((2 * B))
    
    printf "\n[%2d/50] %-40s B=%-2d timeout=%2ds\n" "$total" "$bench_name" "$B" "$timeout_sec"
    
    logfile="/tmp/bench_${bench_name}.log"
    if timeout ${timeout_sec}s cargo bench --bench "$bench_name" > "$logfile" 2>&1; then
        # Show all benchmark times
        grep -E "Benchmarking [^:]+: Collecting" "$logfile" | while read line; do
            bench_item=$(echo "$line" | sed 's/Benchmarking \([^:]*\): Collecting.*/\1/')
            est_time=$(echo "$line" | grep -o "estimated [^ ]*")
            
            # Check if slow (>1.5s)
            if echo "$est_time" | grep -qE "[2-9]\.[0-9]|[1-9][0-9]"; then
                echo "  ⚠️  $bench_item ($est_time)"
                slow=$((slow + 1))
            else
                echo "  ✓ $bench_item ($est_time)"
            fi
        done
        
        # If no benchmarks shown, mark as OK
        if ! grep -q "Benchmarking" "$logfile"; then
            echo "  ✓ OK"
        fi
        ok=$((ok + 1))
    else
        EXIT_CODE=$?
        if [ $EXIT_CODE -eq 124 ]; then
            echo "  ⏱️  TIMEOUT"
            timeout=$((timeout + 1))
        else
            echo "  ❌ ERROR"
            error=$((error + 1))
        fi
    fi
done < <(find benches/ -name "*.rs" -type f | sort | head -50)

echo ""
echo "========================================================="
echo "Summary: $ok OK, $slow slow benchmarks, $timeout timeout, $error error"
