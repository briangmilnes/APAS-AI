#!/bin/bash
# Script to run benchmarks with 10s timeout and track total wall-clock time

TIMEOUT_SECONDS=10
RESULTS_FILE="/tmp/benchmark_timeout_results.txt"
LOG_DIR="/tmp/bench_logs"

# Create log directory
mkdir -p "$LOG_DIR"

# Clear previous results
> "$RESULTS_FILE"

# Get list of benchmarks from Cargo.toml (positions 41-181)
BENCHMARKS=(
BenchInsertionSortSt
BenchMappingStEphChap5_5
BenchDirGraphStEph
BenchUnDirGraphStEph
BenchWeightedDirGraphMtEphFloat
BenchWeightedDirGraphMtEphInt
BenchWeightedDirGraphStEphFloat
BenchWeightedDirGraphStEphInt
BenchWeightedUnDirGraphMtEphFloat
BenchWeightedUnDirGraphMtEphInt
BenchWeightedUnDirGraphStEphFloat
BenchWeightedUnDirGraphStEphInt
BenchExercise12_1
BenchExercise12_2
BenchExercise12_5
BenchBSTParaStEph
BenchBalBinTreeStEph
BenchPrimTreeSeqSt
BenchLabDirGraphStEph
BenchLabUnDirGraphStEph
BenchSetStEph
BenchRelationStEph
BenchBSTSetTreapMtEph
BenchArraySeq
BenchArraySeqMtEph
BenchLinkedListStEph
BenchLinkedListStPer
BenchLinkedListStEph19
BenchLinkedListStPer19
BenchBSTKeyValueStEph
BenchBSTSizeStEph
BenchBSTReducedStEph
BenchArraySetStEph
BenchArraySetEnumMtEph
BenchAVLTreeSetStEph
BenchAVLTreeSetStPer
BenchTableStPer
BenchTableStEph
BenchTableMtEph
BenchOrderedSetStEph
BenchOrderedSetMtEph
BenchOrderedTableStEph
BenchOrderedTableMtEph
BenchAugOrderedTableStPer
BenchAugOrderedTableStEph
BenchAugOrderedTableMtEph
BenchSeparateChaining
BenchLinearProbing
BenchFlatHashTable
BenchAdvancedLinearProbing
BenchAdvancedQuadraticProbing
BenchAdvancedDoubleHashing
BenchGraphSearchStPer
BenchPQMinStPer
BenchPQMinMtEph
BenchEdgeSetGraphStPer
BenchAdjTableGraphStPer
BenchSubsetSumStPer
BenchSubsetSumStEph
BenchSubsetSumMtPer
BenchSubsetSumMtEph
BenchOptBinSearchTreeStPer
BenchOptBinSearchTreeStEph
BenchMatrixChainStPer
BenchMatrixChainStEph
BenchBottomUpDPStPer
BenchBottomUpDPStEph
BenchBottomUpDPMtPer
BenchBottomUpDPMtEph
BenchTopDownDPStPer
BenchTopDownDPStEph
BenchTopDownDPMtPer
BenchTopDownDPMtEph
BenchMinEditDistStPer
BenchMinEditDistStEph
BenchMinEditDistMtPer
BenchMinEditDistMtEph
BenchBFSStEph
BenchBFSMtEph
BenchBFSStPer
BenchBFSMtPer
BenchOrderStatSelectStPer
BenchOrderStatSelectMtPer
BenchOrderStatSelectStEph
BenchOrderStatSelectMtEph
BenchMaxContigSubSumBruteStEph
BenchMaxContigSubSumReducedStEph
BenchMaxContigSubSumOptStEph
BenchMaxContigSubSumOptMtEph
BenchMaxContigSubSumDivConStEph
BenchMaxContigSubSumDivConMtEph
BenchMaxContigSubSumDivConOptStEph
BenchMaxContigSubSumDivConOptMtEph
BenchMergeSortSt
BenchMergeSortMt
BenchDivConReduceSt
BenchDivConReduceMt
BenchReduceContractStEph
BenchScanContractStEph
BenchReduceContractMtEph
BenchScanContractMtEph
BenchDFSStEph
BenchDFSStPer
BenchCycleDetectStEph
BenchCycleDetectStPer
BenchTopoSortStEph
BenchTopoSortStPer
BenchSCCStEph
BenchSCCStPer
BenchPathWeightUtilsStEph
BenchPathWeightUtilsStPer
BenchSSSPResultStEphInt
BenchSSSPResultStEphFloat
BenchStackStEph
BenchDijkstraStEphInt
BenchDijkstraStEphFloat
BenchBellmanFordStEphInt
BenchBellmanFordStEphFloat
BenchJohnsonStEphInt
BenchJohnsonStEphFloat
BenchJohnsonMtEphInt
BenchJohnsonMtEphFloat
BenchVertexMatchingStEph
BenchVertexMatchingMtEph
BenchEdgeContractionStEph
BenchEdgeContractionMtEph
BenchStarPartitionStEph
BenchStarPartitionMtEph
BenchStarContractionStEph
BenchStarContractionMtEph
BenchConnectivityStEph
BenchConnectivityMtEph
BenchSpanTreeStEph
BenchSpanTreeMtEph
BenchTSPApproxStEph
BenchTSPApproxMtEph
BenchUnionFindStEph
BenchPrimStEph
BenchKruskalStEph
BenchBoruvkaStEph
BenchBoruvkaMtEph
)

TOTAL=${#BENCHMARKS[@]}
COUNT=0

echo "Running $TOTAL benchmarks with ${TIMEOUT_SECONDS}s timeout per file..."
echo "Results will be saved to: $RESULTS_FILE"
echo ""

for BENCH in "${BENCHMARKS[@]}"; do
    COUNT=$((COUNT + 1))
    LOG_FILE="$LOG_DIR/${BENCH}.log"
    
    echo -n "[$COUNT/$TOTAL] Running $BENCH... "
    
    # Run with timeout and measure wall-clock time
    START=$(date +%s.%N)
    timeout ${TIMEOUT_SECONDS}s cargo bench -j 10 --bench "$BENCH" > "$LOG_FILE" 2>&1
    EXIT_CODE=$?
    END=$(date +%s.%N)
    
    # Calculate runtime
    RUNTIME=$(echo "$END - $START" | bc)
    
    # Record result
    if [ $EXIT_CODE -eq 124 ]; then
        echo "TIMEOUT (>${TIMEOUT_SECONDS}s)"
        echo "$BENCH|TIMEOUT|$RUNTIME|$LOG_FILE" >> "$RESULTS_FILE"
    elif [ $EXIT_CODE -ne 0 ]; then
        echo "ERROR (${RUNTIME}s)"
        echo "$BENCH|ERROR|$RUNTIME|$LOG_FILE" >> "$RESULTS_FILE"
    else
        echo "${RUNTIME}s"
        echo "$BENCH|OK|$RUNTIME|$LOG_FILE" >> "$RESULTS_FILE"
    fi
done

echo ""
echo "All benchmarks completed!"
echo "Results saved to: $RESULTS_FILE"
