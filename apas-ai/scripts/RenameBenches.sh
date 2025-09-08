#!/usr/bin/env bash
set -euo pipefail

# Repo root and benches directory
REPO="/home/milnes/APASVERUS/APAS-AI/apas-ai"
ROOT="$REPO/benches"

# Mapping: old filename -> new canonical filename
declare -A MAP=(
  ["BenchArraySeq.rs"]="BenchArraySeqEph.rs"
  ["BenchArraySeqEphemeral.rs"]="BenchArraySeqEphChap18.rs"
  ["BenchArraySeqPersistent.rs"]="BenchArraySeqPer.rs"
  ["BenchAVLTreeSeq.rs"]="BenchAVLTreeSeqPer.rs"
  ["BenchAVLTreeSeqPersistent.rs"]="BenchAVLTreeSeqPerChap19.rs"
  ["BenchMathSeq.rs"]="BenchMathSeq.rs"
  ["BenchSinglyLinkedListPersistent.rs"]="BenchLinkedListPer.rs"
)

echo "Renaming benches in $ROOT ..."
pushd "$REPO" >/dev/null

for src in "${!MAP[@]}"; do
  dst="${MAP[$src]}"

  if [[ ! -e "$ROOT/$src" ]]; then
    echo "WARN: missing $src, skipping"
    continue
  fi

  if [[ "$src" == "$dst" ]]; then
    echo "OK: $src already canonical"
    continue
  fi

  if [[ -e "$ROOT/$dst" ]]; then
    echo "ERROR: destination $dst already exists. Resolve manually before proceeding."
    exit 1
  fi

  git mv "$ROOT/$src" "$ROOT/$dst"
  echo "RENAMED: $src -> $dst"
done

popd >/dev/null
echo "Done."
