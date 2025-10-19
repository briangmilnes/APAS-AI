#!/usr/bin/env python3
"""Check test variant purity: Eph tests should not use Per types, etc.

Git commit: [current]
Date: 2025-10-19

Enforces architectural purity:
  - Eph tests should only use Eph types (no Per variants)
  - Per tests should only use Per types (no Eph variants)
  - St tests should only use St types (no Mt variants, with exceptions)
  - Mt tests can use St types internally (allowed for graph adjacency lists, etc.)

Usage:
  check_test_variant_purity.py [--log_file FILE]
  
Output: analyses/code_review/test_variant_purity.txt by default
"""

import re
import sys
from pathlib import Path
from typing import List, Tuple


class TeeOutput:
    """Write to both stdout and a file."""
    def __init__(self, filepath):
        self.terminal = sys.stdout
        self.log = open(filepath, 'w', encoding='utf-8')
    
    def write(self, message):
        self.terminal.write(message)
        self.log.write(message)
    
    def flush(self):
        self.terminal.flush()
        self.log.flush()
    
    def close(self):
        self.log.close()


def extract_variant(filename: str) -> str:
    """Extract variant from test filename: StEph, StPer, MtEph, MtPer."""
    name_upper = filename.upper()
    if 'STEPH' in name_upper:
        return 'StEph'
    elif 'STPER' in name_upper:
        return 'StPer'
    elif 'MTEPH' in name_upper:
        return 'MtEph'
    elif 'MTPER' in name_upper:
        return 'MtPer'
    elif 'LABDIRGRAPH' in name_upper or 'WEIGHTEDDIRGRAPH' in name_upper:
        # These have St/Mt but not Eph/Per - check for threading
        if 'MTST' in name_upper or 'MT' in name_upper:
            return 'Mt'
        else:
            return 'St'
    else:
        return None


def check_file_purity(filepath: Path) -> List[Tuple[int, str]]:
    """Check a single test file for variant purity violations.
    
    Returns list of (line_number, violation_message) tuples.
    """
    violations = []
    variant = extract_variant(filepath.stem)
    
    if not variant:
        # Not a variant-specific test file, skip
        return violations
    
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            lines = f.readlines()
    except Exception as e:
        return [(0, f"Error reading file: {e}")]
    
    # Define forbidden patterns for each variant
    if variant == 'StEph':
        forbidden = [
            (r'\bStPer[A-Z]', 'StPer type'),
            (r'\bMtPer[A-Z]', 'MtPer type'),
            (r'\bMtEph[A-Z]', 'MtEph type'),
            (r'Per[A-Z]\w*S\b', 'Per struct'),
            (r'PerTrait', 'Per trait'),
        ]
    elif variant == 'StPer':
        forbidden = [
            (r'\bStEph[A-Z]', 'StEph type'),
            (r'\bMtEph[A-Z]', 'MtEph type'),
            (r'\bMtPer[A-Z]', 'MtPer type'),
            (r'Eph[A-Z]\w*S\b', 'Eph struct'),
            (r'EphTrait', 'Eph trait'),
        ]
    elif variant == 'MtEph':
        forbidden = [
            (r'\bStPer[A-Z]', 'StPer type'),
            (r'\bMtPer[A-Z]', 'MtPer type'),
            (r'Per[A-Z]\w*S\b', 'Per struct'),
            (r'PerTrait', 'Per trait'),
        ]
    elif variant == 'MtPer':
        forbidden = [
            (r'\bStEph[A-Z]', 'StEph type'),
            (r'\bMtEph[A-Z]', 'MtEph type'),
            (r'Eph[A-Z]\w*S\b', 'Eph struct'),
            (r'EphTrait', 'Eph trait'),
        ]
    elif variant == 'St':
        # St tests shouldn't use Mt types (no exceptions for graph tests)
        forbidden = [
            (r'\bMt[A-Z]\w*S\b', 'Mt struct'),
            (r'\bMtTrait', 'Mt trait'),
            (r'::Mt[A-Z]', 'Mt module'),
        ]
    elif variant == 'Mt':
        # Mt tests CAN use St types (common for internal structures)
        # This is explicitly allowed, no checks
        forbidden = []
    else:
        forbidden = []
    
    # Scan file for violations
    for line_num, line in enumerate(lines, start=1):
        # Skip comments
        if line.strip().startswith('//'):
            continue
        
        for pattern, description in forbidden:
            if re.search(pattern, line):
                violations.append((line_num, f"{description} in {variant} test: {line.strip()[:80]}"))
    
    return violations


def main():
    # Parse arguments
    log_file = Path('analyses/code_review/test_variant_purity.txt')
    if '--log_file' in sys.argv:
        idx = sys.argv.index('--log_file')
        if idx + 1 < len(sys.argv):
            log_file = Path(sys.argv[idx + 1])
    
    # Ensure output directory exists
    log_file.parent.mkdir(parents=True, exist_ok=True)
    
    # Set up tee output
    tee = TeeOutput(log_file)
    sys.stdout = tee
    
    print("TEST VARIANT PURITY CHECK")
    print("=" * 80)
    print()
    print("Checking that:")
    print("  - Eph tests only use Eph types (no Per)")
    print("  - Per tests only use Per types (no Eph)")
    print("  - St tests only use St types (no Mt)")
    print("  - Mt tests can use St types (allowed)")
    print()
    
    # Find all test files
    project_root = Path(__file__).parent.parent.resolve()
    tests_dir = project_root / 'tests'
    
    if not tests_dir.exists():
        print(f"Error: {tests_dir} does not exist")
        tee.close()
        return 1
    
    all_violations = []
    files_checked = 0
    
    # Check all .rs files in tests/
    for test_file in sorted(tests_dir.rglob('*.rs')):
        variant = extract_variant(test_file.stem)
        if not variant:
            continue
        
        files_checked += 1
        violations = check_file_purity(test_file)
        
        if violations:
            all_violations.append((test_file, variant, violations))
    
    # Report results
    print(f"Checked {files_checked} variant-specific test files")
    print()
    
    if all_violations:
        print(f"FOUND {len(all_violations)} FILES WITH VIOLATIONS:")
        print("=" * 80)
        print()
        
        for filepath, variant, violations in all_violations:
            rel_path = filepath.relative_to(project_root)
            print(f"{rel_path} ({variant} test):")
            for line_num, message in violations:
                print(f"  Line {line_num}: {message}")
            print()
        
        print(f"TOTAL: {sum(len(v) for _, _, v in all_violations)} violations in {len(all_violations)} files")
        tee.close()
        sys.stdout = tee.terminal
        return 1
    else:
        print("✓ ALL TESTS PASS: No variant purity violations found")
        tee.close()
        sys.stdout = tee.terminal
        return 0


if __name__ == "__main__":
    sys.exit(main())


