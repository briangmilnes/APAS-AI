#!/bin/bash
# Run benchmarks in batches of 10 with proper crash detection

BENCHMARKS=(
    BenchTSPApproxMtEph
    BenchTSPApproxStEph
    BenchKruskalStEph
    BenchPrimStEph
    BenchUnionFindStEph
)

cd /home/milnes/APASVERUS/APAS-AI/apas-ai

for bench in "${BENCHMARKS[@]}"; do
    echo "=== STARTING: $bench ===" 
    sync  # Flush filesystem
    timeout 10s cargo bench -j 10 --bench "$bench" 2>&1 | tail -5
    EXIT_CODE=$?
    sync  # Flush again
    if [ $EXIT_CODE -eq 124 ]; then
        echo "=== TIMEOUT: $bench ===" 
    elif [ $EXIT_CODE -ne 0 ]; then
        echo "=== CRASH: $bench (exit code $EXIT_CODE) ===" 
    else
        echo "=== COMPLETE: $bench ===" 
    fi
    echo ""
done
