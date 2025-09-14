#!/usr/bin/env bash
set -euo pipefail

# Run benches in preferred order (Math, LinkedList, Array, AVL)
cd "$(dirname "$0")/.."

cargo bench \
  --bench BenchMathSeq \
  --bench BenchLinkedListPerChap18 \
  --bench BenchLinkedListPerChap19 \
  --bench BenchLinkedListEph \
  --bench BenchLinkedListEphChap18 \
  --bench BenchLinkedListEphChap19 \
  --bench BenchArraySeqPer \
  --bench BenchArraySeqPerChap18 \
  --bench BenchArraySeqPerChap19 \
  --bench BenchArraySeqEph \
  --bench BenchArraySeqEphChap18 \
  --bench BenchArraySeqEphChap19 
  --bench BenchAVLTreeSeqPer \
  --bench BenchAVLTreeSeqPerChap19 \
  --bench BenchAVLTreeSeqPerChap18 \
  --bench BenchAVLTreeSeqEph \
  --bench BenchAVLTreeSeqEphChap18 \
  --bench BenchAVLTreeSeqEphChap19 \


REPORT_DIR="target/criterion"
INDEX_PATH="$REPORT_DIR/report/index.html"
mkdir -p "$REPORT_DIR/report"

# helper to emit links, falling back to first sub-benchmark report if group-level is missing
emit_section() {
  local header="$1"
  local pattern="$2"
  echo "<h2>$header</h2><ul>"
  while IFS= read -r g; do
    # prefer group-level summary if it exists
    if [ -f "$REPORT_DIR/$g/report/index.html" ]; then
      echo "<li><a href=\"../$g/report/index.html\">$g</a></li>"
    else
      # pick first sub-report under the group
      first_report=$(find "$REPORT_DIR/$g" -type f -path '*/report/index.html' | sort | head -n 1 || true)
      if [ -n "$first_report" ]; then
        rel_path="${first_report#$REPORT_DIR/}"
        echo "<li><a href=\"../$rel_path\">$g</a></li>"
      fi
    fi
  done < <(find "$REPORT_DIR" -maxdepth 1 -mindepth 1 -type d -name "$pattern" -printf '%f\n' | sort)
  echo '</ul>'
}

{
  echo '<!DOCTYPE html>'
  echo '<html><head><meta charset="utf-8"><title>Algorithms Parallel and Sequential implementation in Rust — Benchmarks</title></head><body>'
  echo '<h1>Algorithms Parallel and Sequential implementation in Rust</h1>'
  echo '<h2>Benchmarks (Ordered)</h2>'
  emit_section "Math" 'Math*'
  emit_section "LinkedList" 'LinkedList*'
  emit_section "Array" 'Array*'
  emit_section "AVL" 'AVL*'
  echo '</body></html>'
} > "$INDEX_PATH"

echo "HTML report: file://$(pwd)/target/criterion/report/index.html"


