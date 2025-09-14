#!/usr/bin/env bash
set -euo pipefail

# Generate Emacs TAGS covering both src/ and tests/ using universal-ctags

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")"/.. && pwd)"
SRC_DIR="${ROOT_DIR}/src"
TESTS_DIR="${ROOT_DIR}/tests"
BENCHES_DIR="${ROOT_DIR}/benches"
TAGS_FILE="${ROOT_DIR}/rusty-tags.emacs"

if ! command -v ctags >/dev/null 2>&1; then
  echo "Error: ctags not found. Install universal-ctags (e.g., sudo apt install universal-ctags)." >&2
  exit 1
fi

ctags -e -R -f "${TAGS_FILE}" "${SRC_DIR}" "${TESTS_DIR}" "${BENCHES_DIR}"
echo "Wrote tags: ${TAGS_FILE}"


