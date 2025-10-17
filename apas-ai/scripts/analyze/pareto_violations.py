#!/usr/bin/env python3
"""
Pareto analysis of code review violations.

Runs all review scripts and generates a ranked list showing which violations
are most common, helping prioritize fixes.

Output format:
  1. Import order (460 violations) ████████████████████ 80.0%
  2. Integration tests (51) ███ 8.9%
  3. Where clauses (50) ███ 8.7%
  ...
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import re
import subprocess
import sys
from pathlib import Path


def run_review_script(script_path):
    """Run a review script and parse its output for violation count."""
    try:
        result = subprocess.run(
            [str(script_path)],
            capture_output=True,
            text=True,
            timeout=60
        )
        
        output = result.stdout + result.stderr
        
        # Parse violation count from various formats:
        # "Total violations: 123"
        # "✗ Found X violations"
        # "123 violation(s)"
        
        match = re.search(r'Total violations?: (\d+)', output, re.IGNORECASE)
        if match:
            return int(match.group(1))
        
        match = re.search(r'Found (\d+) violation', output, re.IGNORECASE)
        if match:
            return int(match.group(1))
        
        match = re.search(r'(\d+) violation\(s\)', output)
        if match:
            return int(match.group(1))
        
        # Check for pass
        if '✓' in output or 'PASS' in output or result.returncode == 0:
            return 0
        
        return None  # Unknown format
        
    except subprocess.TimeoutExpired:
        return None
    except Exception as e:
        print(f"Error running {script_path.name}: {e}", file=sys.stderr)
        return None


def get_script_name(script_path):
    """Get human-readable name from script path."""
    name = script_path.stem.replace('review_', '').replace('_', ' ').title()
    
    # Fix common acronyms that should stay uppercase
    acronyms = ['Ufcs', 'Apas', 'Bst', 'Id', 'Ids']
    for acronym in acronyms:
        name = name.replace(acronym, acronym.upper())
    
    return name


def main():
    repo_root = Path(__file__).parent.parent.parent
    scripts_dir = repo_root / "scripts"
    
    # Find all review_*.py scripts (excluding orchestrators)
    review_scripts = []
    
    # RustRules scripts (most important)
    for pattern in ['rust/review_*.py', 'rust/src/review_*.py', 'rust/tests/review_*.py', 'rust/benches/review_*.py']:
        review_scripts.extend(scripts_dir.glob(pattern))
    
    # APAS scripts
    for pattern in ['APAS/src/review_*.py', 'APAS/tests/review_*.py', 'APAS/benches/review_*.py']:
        review_scripts.extend(scripts_dir.glob(pattern))
    
    # Exclude orchestrators (they just call other scripts)
    orchestrators = {
        'review_rust.py', 'review_rust_src.py', 'review_rust_tests.py', 'review_rust_benches.py',
        'review_APAS.py', 'review_APAS_src.py', 'review_APAS_tests.py', 'review_APAS_benches.py'
    }
    
    review_scripts = [s for s in review_scripts if s.name not in orchestrators]
    
    print(f"Running {len(review_scripts)} review scripts...\n")
    
    violations = []
    
    for script in sorted(review_scripts):
        print(f"  {script.name}...", end=' ', flush=True)
        count = run_review_script(script)
        if count is not None:
            violations.append((get_script_name(script), count, script.name))
            print(f"{count} violations" if count > 0 else "✓")
        else:
            print("✗ (parsing failed)")
    
    # Sort by count (descending)
    violations.sort(key=lambda x: x[1], reverse=True)
    
    # Calculate totals
    total_violations = sum(v[1] for v in violations)
    
    if total_violations == 0:
        print("\n✅ No violations found! Perfect codebase.")
        return 0
    
    print(f"\n{'='*70}")
    print(f"PARETO ANALYSIS: {total_violations} total violations")
    print(f"{'='*70}\n")
    
    cumulative = 0
    max_bar_width = 40
    
    for rank, (name, count, script_name) in enumerate(violations, 1):
        if count == 0:
            continue
        
        percent = (count / total_violations) * 100
        cumulative += percent
        bar_width = int((count / total_violations) * max_bar_width)
        bar = '█' * bar_width
        
        print(f"{rank:2}. {name:30} {count:4} │{bar:<{max_bar_width}}│ {percent:5.1f}% (cum: {cumulative:5.1f}%)")
    
    # 80/20 analysis
    print(f"\n{'─'*70}")
    critical_threshold = total_violations * 0.8
    cumulative_count = 0
    critical_issues = []
    
    for name, count, _ in violations:
        if count == 0:
            break
        cumulative_count += count
        critical_issues.append(name)
        if cumulative_count >= critical_threshold:
            break
    
    print(f"📊 Pareto Principle: Fix top {len(critical_issues)} issue(s) to resolve ~80% of violations")
    for i, name in enumerate(critical_issues, 1):
        print(f"   {i}. {name}")
    
    return 1 if total_violations > 0 else 0


if __name__ == "__main__":
    sys.exit(main())

