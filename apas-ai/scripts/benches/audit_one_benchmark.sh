#!/bin/bash
# Usage: audit_one_benchmark.sh <bench_file.rs>
# Example: audit_one_benchmark.sh benches/Chap03/BenchInsertionSortSt.rs

if [ $# -eq 0 ]; then
    echo "Usage: $0 <bench_file.rs>"
    exit 1
fi

bench_file="$1"

# Look up the registered name in Cargo.toml
bench_name=$(grep -B1 "path = \"$bench_file\"" Cargo.toml | grep "name = " | sed 's/.*name = "\(.*\)".*/\1/')

if [ -z "$bench_name" ]; then
    echo "ERROR: $bench_file not found in Cargo.toml"
    exit 1
fi

# Count actual benchmark runs
bench_count=$(python3 scripts/benches/count_benchmark_runs.py "$bench_file")
timeout_sec=$((2 * bench_count + 5))

echo "Auditing: $bench_name"
echo "Benchmarks: $bench_count"
echo "Timeout: ${timeout_sec}s (2s per benchmark + 5s overhead)"
echo ""

# Precompile the benchmark
echo -n "Compiling... "
compile_start=$(date +%s)
if cargo bench --bench "$bench_name" --no-run -j 10 > /dev/null 2>&1; then
    compile_end=$(date +%s)
    compile_time=$((compile_end - compile_start))
    echo "done (${compile_time}s)"
else
    echo "FAILED"
    echo "ERROR: Compilation failed"
    exit 1
fi

# Run the benchmark and show live output
echo "Running..."
echo ""
run_start=$(date +%s)
logfile="/tmp/bench_${bench_name}.log"

if timeout ${timeout_sec}s cargo bench --bench "$bench_name" 2>&1 | tee "$logfile" | grep -E "(^[^ ].*time:|Benchmarking)"; then
    run_end=$(date +%s)
    run_time=$((run_end - run_start))
    
    echo ""
    echo "=================================="
    echo "Summary:"
    echo "  Compile: ${compile_time}s"
    echo "  Run: ${run_time}s"
    echo "  Total: $((compile_time + run_time))s"
    
    # Check for slow benchmarks
    slow=$(grep -B1 "time:" "$logfile" | grep -v "time:" | grep -v "^--$" | while read name; do
        time_line=$(grep -A1 "^$name$" "$logfile" | grep "time:" | head -1)
        mean=$(echo "$time_line" | grep -oE "[0-9]+\.[0-9]+ [a-zµμ]+" | head -2 | tail -1)
        value=$(echo "$mean" | awk '{print $1}')
        unit=$(echo "$mean" | awk '{print $2}')
        
        if [ "$unit" = "s" ]; then
            seconds=$value
        elif [ "$unit" = "ms" ]; then
            seconds=$(echo "$value / 1000" | bc -l)
        else
            seconds=0
        fi
        
        if echo "$seconds > 1.3" | bc -l | grep -q 1; then
            echo "  SLOW: $name ($mean)"
        fi
    done)
    
    if [ -n "$slow" ]; then
        echo ""
        echo "⚠ Slow benchmarks:"
        echo "$slow"
    else
        echo "  ✓ All benchmarks < 1.3s"
    fi
else
    EXIT_CODE=$?
    if [ $EXIT_CODE -eq 124 ]; then
        echo ""
        echo "ERROR: Timeout (>${timeout_sec}s)"
    else
        echo ""
        echo "ERROR: Execution failed"
    fi
fi
