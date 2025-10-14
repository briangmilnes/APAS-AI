#!/usr/bin/env bash
set -euo pipefail

# Generate Emacs TAGS covering both src/ and tests/ using universal-ctags

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")"/.. && pwd)"
TAGS_FILE="${ROOT_DIR}/rusty-tags.emacs"

if ! command -v ctags >/dev/null 2>&1; then
  echo "Error: ctags not found. Install universal-ctags (e.g., sudo apt install universal-ctags)." >&2
  exit 1
fi

ctags -e -R -f "${TAGS_FILE}" ${ROOT_DIR}/src ${ROOT_DIR}/src/Chap* ${ROOT_DIR}/tests ${ROOT_DIR}/tests/Chap* ${ROOT_DIR}/benches ${ROOT_DIR}/benches/Chap*
echo "Wrote tags: ${TAGS_FILE}"


