#!/usr/bin/env python3
"""
Common utilities for review scripts.

Provides standardized argument parsing, file discovery, and reporting.
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import argparse
import sys
from pathlib import Path
from typing import List, Optional, Callable


def get_repo_root() -> Path:
    """Get the repository root from any script location."""
    # Scripts are in scripts/*, scripts/*/*, or scripts/*/*/*
    script_path = Path(__file__).resolve()
    # Go up to find the repo root (where Cargo.toml is)
    current = script_path.parent
    while current != current.parent:
        if (current / "Cargo.toml").exists():
            return current
        current = current.parent
    raise RuntimeError("Could not find repository root (Cargo.toml)")


def create_review_parser(description: str) -> argparse.ArgumentParser:
    """
    Create standardized argument parser for review scripts.
    
    Args:
        description: Description of what the review script checks
        
    Returns:
        ArgumentParser with --file and --dry-run options
    """
    parser = argparse.ArgumentParser(description=description)
    parser.add_argument(
        '--file',
        type=str,
        help='Specific file to check (instead of searching directories)'
    )
    parser.add_argument(
        '--dry-run',
        action='store_true',
        help='Show what would be checked without actually checking'
    )
    return parser


def find_rust_files(
    directories: List[Path],
    single_file: Optional[str] = None,
    repo_root: Optional[Path] = None
) -> List[Path]:
    """
    Find Rust files to check.
    
    Args:
        directories: List of directories to search recursively
        single_file: If provided, check only this file
        repo_root: Repository root (for resolving relative paths)
        
    Returns:
        List of Path objects for Rust files to check
    """
    if repo_root is None:
        repo_root = get_repo_root()
    
    if single_file:
        file_path = Path(single_file)
        if not file_path.is_absolute():
            file_path = repo_root / file_path
        if not file_path.exists():
            print(f"Error: File not found: {file_path}", file=sys.stderr)
            sys.exit(1)
        if not file_path.suffix == '.rs':
            print(f"Error: Not a Rust file: {file_path}", file=sys.stderr)
            sys.exit(1)
        return [file_path]
    
    rust_files = []
    for directory in directories:
        if not directory.exists():
            continue
        rust_files.extend(directory.rglob("*.rs"))
    
    return sorted(rust_files)


def report_violations(
    violations: List[tuple],
    rule_name: str,
    rule_reference: str,
    fix_suggestion: Optional[str] = None,
    repo_root: Optional[Path] = None
) -> int:
    """
    Print standardized violation report.
    
    Args:
        violations: List of violation tuples (format depends on check type)
        rule_name: Human-readable rule name
        rule_reference: Reference to rule document (e.g., "RustRules.md Line 86")
        fix_suggestion: Optional suggestion for how to fix
        repo_root: Repository root for computing relative paths
        
    Returns:
        Exit code (0 for pass, 1 for violations found)
    """
    if repo_root is None:
        repo_root = get_repo_root()
    
    if not violations:
        print(f"✓ {rule_name}: PASS")
        return 0
    
    print(f"✗ {rule_name}: {len(violations)} violation(s) ({rule_reference})\n")
    
    # Violations format varies by check type, so this is flexible
    # Caller should format their own output, this just handles header/footer
    
    if fix_suggestion:
        print(f"\n{fix_suggestion}")
    
    return 1


def report_simple(
    passed: bool,
    rule_name: str,
    violation_count: int = 0,
    rule_reference: str = ""
) -> int:
    """
    Simple pass/fail report.
    
    Args:
        passed: Whether the check passed
        rule_name: Human-readable rule name
        violation_count: Number of violations (if failed)
        rule_reference: Optional rule reference
        
    Returns:
        Exit code (0 for pass, 1 for fail)
    """
    if passed:
        print(f"✓ {rule_name}: PASS")
        return 0
    else:
        ref = f" ({rule_reference})" if rule_reference else ""
        print(f"✗ {rule_name}: {violation_count} violation(s){ref}")
        return 1


class ReviewContext:
    """Context object for review operations."""
    
    def __init__(self, args: argparse.Namespace):
        self.args = args
        self.repo_root = get_repo_root()
        self.dry_run = args.dry_run
        self.single_file = args.file
    
    def find_files(self, directories: List[Path]) -> List[Path]:
        """Find files to check based on context."""
        return find_rust_files(
            directories,
            single_file=self.single_file,
            repo_root=self.repo_root
        )
    
    def relative_path(self, file_path: Path) -> Path:
        """Get relative path from repo root."""
        try:
            return file_path.relative_to(self.repo_root)
        except ValueError:
            return file_path


def run_review(
    description: str,
    rule_name: str,
    rule_reference: str,
    directories: List[Path],
    check_function: Callable[[Path, 'ReviewContext'], List],
    fix_suggestion: Optional[str] = None
) -> int:
    """
    Standard review script runner.
    
    Args:
        description: Script description
        rule_name: Human-readable rule name
        rule_reference: Rule document reference
        directories: Directories to search
        check_function: Function that takes (file_path, context) and returns violations
        fix_suggestion: Optional fix suggestion
        
    Returns:
        Exit code
    """
    parser = create_review_parser(description)
    args = parser.parse_args()
    
    context = ReviewContext(args)
    
    if context.dry_run:
        files = context.find_files(directories)
        print(f"Would check {len(files)} file(s) for: {rule_name}")
        return 0
    
    all_violations = []
    files = context.find_files(directories)
    
    for file_path in files:
        violations = check_function(file_path, context)
        if violations:
            all_violations.extend(violations)
    
    if not all_violations:
        print(f"✓ {rule_name}: PASS")
        return 0
    
    print(f"✗ {rule_name}: {len(all_violations)} violation(s) ({rule_reference})\n")
    
    # Print violations (format depends on what check_function returns)
    for violation in all_violations:
        print(violation)
    
    if fix_suggestion:
        print(f"\n{fix_suggestion}")
    
    return 1

