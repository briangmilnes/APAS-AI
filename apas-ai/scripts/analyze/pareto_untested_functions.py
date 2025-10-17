#!/usr/bin/env python3
"""
Generates a Pareto analysis of untested functions from llvm-cov coverage data.
Identifies functions with zero test coverage and ranks them by impact (number of lines).
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import json
import re
import subprocess
import sys
from pathlib import Path
from collections import defaultdict


def demangle_all_names(mangled_names):
    """Demangle all Rust symbols at once using rustfilt (batch processing)."""
    if not mangled_names:
        return {}
    
    try:
        # Batch process all names through rustfilt
        input_text = '\n'.join(mangled_names)
        result = subprocess.run(
            ['rustfilt'],
            input=input_text,
            capture_output=True,
            text=True,
            timeout=30
        )
        
        if result.returncode == 0:
            demangled_lines = result.stdout.strip().split('\n')
            mapping = {}
            
            for mangled, demangled in zip(mangled_names, demangled_lines):
                # Extract just the function name (last meaningful part after ::)
                if '::' in demangled:
                    parts = demangled.split('::')
                    # Find the actual function name (skip generic params)
                    for part in reversed(parts):
                        clean = part.split('<')[0].split('(')[0].strip()
                        if clean and not clean.startswith('{') and not clean.startswith('<'):
                            mapping[mangled] = clean[:50]
                            break
                    else:
                        mapping[mangled] = demangled.split('(')[0].split('<')[0][:50]
                else:
                    mapping[mangled] = demangled.split('(')[0].split('<')[0][:50]
            
            return mapping
    except (subprocess.TimeoutExpired, FileNotFoundError):
        pass
    
    # Fallback: return original names truncated
    return {name: name[:50] for name in mangled_names}


def get_file_module(filepath):
    """Extract module path from file path."""
    path = Path(filepath)
    if 'src/' in str(path):
        # Get relative path from src/
        parts = str(path).split('src/', 1)[1]
        return parts.replace('.rs', '').replace('/', '::')
    return path.stem


def is_external_dep(filepath):
    """Check if a file is from an external dependency."""
    path_str = str(filepath)
    return any(marker in path_str for marker in [
        'index.crates.io',
        'rust/library/std',
        'rust/library/core',
        'rust/library/alloc',
        '.cargo/registry',
    ])


def is_test_file(filepath):
    """Check if a file is a test file (not src/)."""
    path_str = str(filepath)
    return '/tests/' in path_str or '/benches/' in path_str or 'main.rs' in path_str


def main():
    repo_root = Path(__file__).parent.parent.parent
    analyses_dir = repo_root / "analyses"
    coverage_json = analyses_dir / "coverage.json"
    
    if not coverage_json.exists():
        print("Error: coverage.json not found. Run ./scripts/llvm-cov.py first.")
        return 1
    
    print("Analyzing untested functions from coverage data...\n")
    
    with open(coverage_json, 'r') as f:
        data = json.load(f)
    
    # First pass: collect all untested function names and demangle in batch
    print("Collecting untested functions...")
    untested_raw = []
    mangled_names = []
    
    for entry in data['data'][0]['functions']:
        if entry['count'] == 0:  # Untested function
            # Skip compiler-generated functions
            if any(skip in entry['name'] for skip in ['_Drop', '_Clone', '_Default', '_Debug', '_Display', '_PartialEq', '_Eq', '_PartialOrd', '_Ord']):
                continue
            
            if entry['filenames']:
                filepath = entry['filenames'][0]
                
                # Skip external dependencies
                if is_external_dep(filepath):
                    continue
                
                # Skip test files - only show untested src/ functions
                if is_test_file(filepath):
                    continue
                
                untested_raw.append({
                    'mangled': entry['name'],
                    'filepath': filepath,
                    'regions': len(entry['regions'])
                })
                mangled_names.append(entry['name'])
    
    print(f"Demangling {len(mangled_names)} function names...")
    name_map = demangle_all_names(mangled_names)
    
    # Second pass: build final list with demangled names and deduplicate
    # Group by (module, function) to handle generic instantiations
    functions_map = {}
    
    for raw in untested_raw:
        module = get_file_module(raw['filepath'])
        func_name = name_map.get(raw['mangled'], raw['mangled'][:50])
        
        # Remove generic parameters and return type to get base function name
        base_func = func_name.split('<')[0].split('(')[0].strip()
        
        # Skip test functions (test_ prefix or example_ prefix in src/ files)
        if base_func.startswith('test_') or base_func.startswith('example_') or base_func.startswith('performance_'):
            continue
        
        key = (module, base_func)
        
        if key not in functions_map:
            functions_map[key] = {
                'module': module,
                'function': base_func,
                'filepath': raw['filepath'],
                'regions': raw['regions'],
                'instantiations': 1,
                'mangled': raw['mangled']
            }
        else:
            # Accumulate regions from all instantiations
            functions_map[key]['regions'] += raw['regions']
            functions_map[key]['instantiations'] += 1
    
    # Convert to list
    untested = list(functions_map.values())
    
    if not untested:
        print("✓ All functions are tested!")
        return 0
    
    # Sort by impact (number of regions) - lowest to highest
    untested.sort(key=lambda x: x['regions'])
    
    # Calculate totals
    total_untested = len(untested)
    total_regions = sum(f['regions'] for f in untested)
    
    # Group by module for better organization
    by_module = defaultdict(list)
    for func in untested:
        by_module[func['module']].append(func)
    
    # Module-level summary (sorted by regions, highest to lowest)
    module_summary = []
    for module, funcs in by_module.items():
        total_module_regions = sum(f['regions'] for f in funcs)
        total_module_insts = sum(f['instantiations'] for f in funcs)
        module_summary.append({
            'module': module,
            'count': len(funcs),
            'instantiations': total_module_insts,
            'regions': total_module_regions
        })
    
    module_summary.sort(key=lambda x: x['regions'], reverse=True)  # Highest to lowest
    
    print("=" * 80)
    print(f"UNTESTED FUNCTIONS: {total_untested} functions, {total_regions} uncovered regions")
    print("=" * 80)
    print()
    print(f"{'Module':<45} {'Funcs':<8} {'Insts':<8} {'Regions':<10}")
    print("─" * 80)
    
    for m in module_summary:
        print(f"{m['module']:<45} {m['count']:<8} {m['instantiations']:<8} {m['regions']:<10}")
    
    # Save module summary report (simple, easy to read)
    summary_file = analyses_dir / "untested_modules.txt"
    with open(summary_file, 'w') as f:
        f.write("=" * 80 + "\n")
        f.write(f"UNTESTED FUNCTIONS: {total_untested} functions, {total_regions} uncovered regions\n")
        f.write("=" * 80 + "\n\n")
        f.write(f"{'Module':<45} {'Funcs':<8} {'Insts':<8} {'Regions':<10}\n")
        f.write("─" * 80 + "\n")
        for m in module_summary:
            f.write(f"{m['module']:<45} {m['count']:<8} {m['instantiations']:<8} {m['regions']:<10}\n")
    
    print(f"\n✓ Module summary saved to: {summary_file.relative_to(repo_root)}")
    
    # Save detailed report (function-level breakdown)
    output_file = analyses_dir / "untested_functions.txt"
    with open(output_file, 'w') as f:
        f.write(f"UNTESTED FUNCTIONS: {total_untested} functions, {total_regions} uncovered regions\n")
        f.write("=" * 70 + "\n\n")
        
        f.write("All Untested Functions (sorted by impact):\n\n")
        f.write(f"{'Rank':<6} {'Regions':<8} {'Insts':<6} {'Module':<50} {'Function':<30}\n")
        f.write("─" * 120 + "\n")
        
        for i, func in enumerate(untested, 1):
            f.write(f"{i:<6} {func['regions']:<8} {func['instantiations']:<6} {func['module']:<50} {func['function']:<30}\n")
        
        f.write("\n" + "=" * 70 + "\n")
        f.write("By Module:\n\n")
        
        for module in sorted(by_module.keys()):
            funcs = by_module[module]
            total = sum(f['regions'] for f in funcs)
            total_insts = sum(f['instantiations'] for f in funcs)
            f.write(f"\n{module} ({len(funcs)} functions, {total_insts} instantiations, {total} regions):\n")
            for func in sorted(funcs, key=lambda x: x['regions'], reverse=True):
                inst_note = f" ({func['instantiations']} insts)" if func['instantiations'] > 1 else ""
                f.write(f"  - {func['function']:<40} {func['regions']:>4} regions{inst_note}\n")
    
    print(f"✓ Detailed function report saved to: {output_file.relative_to(repo_root)}")
    print(f"\nNext: Review untested modules and add targeted tests.")
    
    return 1  # Return error code to indicate work needed


if __name__ == "__main__":
    sys.exit(main())

