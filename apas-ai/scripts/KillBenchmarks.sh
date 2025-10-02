#!/usr/bin/env bash
# KillBenchmarks.sh - Kill all running benchmark and cargo processes
# Usage: ./scripts/KillBenchmarks.sh

set -e

echo "===================================================================="
echo "Killing all benchmark and cargo processes..."
echo "===================================================================="

# Kill the benchmark runner script first
if pgrep -f "RunBenchmarksWithTimeout.sh" > /dev/null; then
    echo "Killing benchmark runner script..."
    pkill -9 -f "RunBenchmarksWithTimeout.sh" || true
    echo "  ✓ Killed RunBenchmarksWithTimeout.sh"
else
    echo "  No benchmark runner script found"
fi

# Kill all cargo bench processes
if pgrep -f "cargo bench" > /dev/null; then
    echo "Killing 'cargo bench' processes..."
    pkill -9 -f "cargo bench" || true
    echo "  ✓ Killed cargo bench"
else
    echo "  No 'cargo bench' processes found"
fi

# Kill all benchmark executables (in target/release/deps/Bench*)
if pgrep -f "target/release/deps/Bench" > /dev/null; then
    echo "Killing benchmark executables..."
    pkill -9 -f "target/release/deps/Bench" || true
    echo "  ✓ Killed benchmark executables"
else
    echo "  No benchmark executables found"
fi

# Kill any rustc processes (in case compilation is stuck)
if pgrep rustc > /dev/null; then
    echo "Killing rustc processes..."
    pkill -9 rustc || true
    echo "  ✓ Killed rustc"
else
    echo "  No rustc processes found"
fi

# Kill any cargo processes
if pgrep cargo > /dev/null; then
    echo "Killing cargo processes..."
    pkill -9 cargo || true
    echo "  ✓ Killed cargo"
else
    echo "  No cargo processes found"
fi

# Wait a moment for processes to terminate
sleep 1

# Verify all killed
REMAINING=$(pgrep -f "cargo\|rustc\|Bench" | wc -l || echo "0")

echo "===================================================================="
if [ "$REMAINING" -eq 0 ]; then
    echo "✅ All benchmark processes killed successfully"
else
    echo "⚠️  Warning: $REMAINING processes may still be running"
    echo "Run 'ps aux | grep -E \"cargo|rustc|Bench\"' to check"
fi
echo "===================================================================="

