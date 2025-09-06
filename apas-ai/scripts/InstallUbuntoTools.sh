#!/usr/bin/env bash
set -euo pipefail

if ! command -v apt-get >/dev/null 2>&1; then
  echo "This script supports Ubuntu/Debian systems with apt-get." >&2
  exit 1
fi

sudo apt-get update -y
sudo apt-get install -y build-essential pkg-config libssl-dev curl git unzip jq ripgrep
sudo apt-get install -y universal-ctags || sudo apt-get install -y exuberant-ctags || sudo apt-get install -y ctags
cargo install rusty-tags

echo "Installed base Unix tools: build-essential, pkg-config, libssl-dev, curl, git, unzip, jq, ripgrep, (universal-)ctags, rusty-tags"

