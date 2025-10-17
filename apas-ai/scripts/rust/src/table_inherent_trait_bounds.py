#!/usr/bin/env python3
"""
Table of structs with both inherent impl and trait impl, showing their bounds.

Outputs a table with columns:
- File
- Struct
- Inherent Impl Bounds
- Trait Impl Bounds

This helps identify bound mismatches between inherent and trait implementations.
"""

# Git commit: 3ca9dcff19371dfe2d4dce7d93788971c098ac93
# Date: 2025-10-17 05:23:25 -0700

import re
import sys
from pathlib import Path
from collections import defaultdict

# Add lib directory to path for imports
sys.path.insert(0, str(Path(__file__).parent.parent.parent / 'lib'))
from review_utils import ReviewContext, create_review_parser


STANDARD_TRAITS = {
    'Debug', 'Clone', 'Copy', 'PartialEq', 'Eq', 'PartialOrd', 'Ord',
    'Hash', 'Display', 'Default', 'From', 'Into', 'AsRef', 'AsMut',
    'Deref', 'DerefMut', 'Drop', 'Iterator', 'IntoIterator',
    'Send', 'Sync', 'Sized', 'Unpin', 'Add', 'Sub', 'Mul', 'Div',
    'Error', 'Index', 'IndexMut'
}


def extract_bounds_from_generics(generics_str):
    """
    Extract bounds from generic parameter string.
    
    Examples:
        "T: Eq + Hash" -> "T: Eq + Hash"
        "T: StT + Hash, U: Clone" -> "T: StT + Hash, U: Clone"
        "T" -> "T"
    """
    if not generics_str:
        return ""
    
    # Clean up whitespace
    return ' '.join(generics_str.split())


def parse_impl_line(line):
    """
    Parse an impl line to extract type and bounds.
    
    Returns:
        - ('inherent', struct_name, bounds_str) for inherent impl
        - ('trait', struct_name, trait_name, bounds_str) for trait impl
        - None if not an impl line
    """
    line = line.strip()
    
    if not line.startswith('impl'):
        return None
    
    # Remove comments
    line = re.sub(r'//.*$', '', line)
    
    # Check for trait impl: impl<...> TraitName<...> for StructName<...>
    trait_match = re.match(
        r'impl\s*(?:<([^>]+)>)?\s+(?:[\w:]+::)?(\w+)(?:<[^>]*>)?\s+for\s+(\w+)',
        line
    )
    if trait_match:
        bounds = extract_bounds_from_generics(trait_match.group(1))
        trait_name = trait_match.group(2)
        struct_name = trait_match.group(3)
        return ('trait', struct_name, trait_name, bounds)
    
    # Check for inherent impl: impl<...> StructName<...> {
    inherent_match = re.match(
        r'impl\s*(?:<([^>]+)>)?\s+(\w+)(?:<[^>]*>)?\s*(?:where|{)',
        line
    )
    if inherent_match:
        bounds = extract_bounds_from_generics(inherent_match.group(1))
        struct_name = inherent_match.group(2)
        return ('inherent', struct_name, bounds)
    
    return None


def find_impl_block_range(lines, start_line):
    """Find the end line of an impl block starting at start_line (0-indexed)."""
    if start_line >= len(lines):
        return start_line
    
    brace_depth = 0
    started = False
    
    for i in range(start_line, len(lines)):
        line = lines[i]
        if '{' in line:
            started = True
        brace_depth += line.count('{') - line.count('}')
        if started and brace_depth <= 0:
            return i
    
    return len(lines) - 1


def extract_method_names(lines, start_idx, end_idx):
    """Extract method names from a block."""
    methods = []
    for i in range(start_idx, end_idx + 1):
        line = lines[i].strip()
        # Match method definitions
        match = re.match(r'(?:pub\s+)?fn\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*[(<]', line)
        if match and not line.startswith('//'):
            methods.append(match.group(1))
    return methods


def check_delegation(lines, trait_start, trait_end, struct_name):
    """
    Check if trait impl methods delegate to inherent impl methods.
    
    Returns a delegation pattern string:
    - "Full delegation" if all methods delegate
    - "Partial (X/Y)" if some delegate
    - "Direct impl" if no delegation
    - "No methods" if empty impl
    """
    method_count = 0
    delegation_count = 0
    
    i = trait_start
    while i <= trait_end:
        line = lines[i].strip()
        
        # Check for method definition
        method_match = re.match(r'(?:pub\s+)?fn\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*[(<]', line)
        if method_match and not line.startswith('//'):
            method_name = method_match.group(1)
            method_count += 1
            
            # Look for delegation in the next few lines
            # Patterns: Self::method_name, StructName::method_name, self.method_name
            for j in range(i, min(i + 10, trait_end + 1)):
                body_line = lines[j]
                
                # Check for Self::method_name or StructName::method_name
                if re.search(rf'\b(?:Self|{struct_name})::{method_name}\s*\(', body_line):
                    delegation_count += 1
                    break
                
                # Check for self.method_name() where it's the main call
                # (not just calling in a complex expression)
                if re.search(rf'\bself\.{method_name}\s*\(', body_line):
                    # Make sure it's not just a recursive call or complex expression
                    if 'return' in body_line or body_line.strip().startswith('self.'):
                        delegation_count += 1
                        break
        
        i += 1
    
    if method_count == 0:
        return "No methods"
    elif delegation_count == 0:
        return "Direct impl"
    elif delegation_count == method_count:
        return "Full delegation"
    else:
        return f"Partial ({delegation_count}/{method_count})"


def analyze_file(filepath, context):
    """Analyze a file for inherent and trait impl bounds."""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            lines = f.readlines()
    except Exception:
        return {}
    
    # Track struct_name -> {'inherent': [bounds], 'traits': {trait_name: bounds}, 'delegation': pattern}
    struct_impls = defaultdict(lambda: {'inherent': [], 'traits': {}, 'delegation': {}})
    
    for i, line in enumerate(lines):
        result = parse_impl_line(line)
        
        if not result:
            continue
        
        if result[0] == 'inherent':
            _, struct_name, bounds = result
            struct_impls[struct_name]['inherent'].append((i + 1, bounds))
        
        elif result[0] == 'trait':
            _, struct_name, trait_name, bounds = result
            # Skip standard traits
            if trait_name not in STANDARD_TRAITS:
                trait_start = i
                trait_end = find_impl_block_range(lines, i)
                delegation = check_delegation(lines, trait_start, trait_end, struct_name)
                
                struct_impls[struct_name]['traits'][trait_name] = (i + 1, bounds)
                struct_impls[struct_name]['delegation'][trait_name] = delegation
    
    # Filter to only structs with BOTH inherent and custom trait impls
    filtered = {}
    for struct_name, impls in struct_impls.items():
        if impls['inherent'] and impls['traits']:
            filtered[struct_name] = impls
    
    return filtered


def main():
    parser = create_review_parser(
        description="Table of inherent and trait impl bounds for comparison"
    )
    args = parser.parse_args()
    context = ReviewContext(args)

    # Only check src/ files
    src_dir = context.repo_root / 'src'
    if not src_dir.exists():
        print("✗ No src/ directory found")
        return 1
    
    files = list(src_dir.rglob('*.rs'))
    
    # Collect all data
    all_data = []
    
    for filepath in sorted(files):
        struct_impls = analyze_file(filepath, context)
        if struct_impls:
            rel_path = context.relative_path(filepath)
            for struct_name, impls in sorted(struct_impls.items()):
                # Combine all inherent impl bounds
                inherent_bounds_list = [bounds for (line, bounds) in impls['inherent']]
                inherent_bounds = ' | '.join(inherent_bounds_list) if inherent_bounds_list else "(none)"
                
                # Combine all trait impl bounds
                trait_bounds_list = []
                for trait_name, (line, bounds) in sorted(impls['traits'].items()):
                    trait_bounds_list.append(f"{trait_name}: {bounds}" if bounds else trait_name)
                trait_bounds = ' | '.join(trait_bounds_list) if trait_bounds_list else "(none)"
                
                # Get delegation info
                delegation_list = []
                for trait_name in sorted(impls['traits'].keys()):
                    delegation = impls['delegation'].get(trait_name, "Unknown")
                    delegation_list.append(delegation)
                delegation_str = ' | '.join(delegation_list) if delegation_list else "Unknown"
                
                all_data.append({
                    'file': str(rel_path),
                    'struct': struct_name,
                    'inherent': inherent_bounds if inherent_bounds_list[0] else "(no bounds)",
                    'trait': trait_bounds,
                    'delegation': delegation_str
                })
    
    if not all_data:
        print("\n✓ No structs with both inherent and trait impls found!")
        return 0
    
    # Print table
    print(f"\nFound {len(all_data)} struct(s) with both inherent impl and trait impl:\n")
    print("=" * 150)
    
    # Calculate column widths
    max_file = max(len(d['file']) for d in all_data)
    max_struct = max(len(d['struct']) for d in all_data)
    max_inherent = max(len(d['inherent']) for d in all_data)
    max_trait = max(len(d['trait']) for d in all_data)
    max_delegation = max(len(d['delegation']) for d in all_data)
    
    # Limit column widths for readability
    max_file = min(max_file, 40)
    max_struct = min(max_struct, 20)
    max_inherent = min(max_inherent, 30)
    max_trait = min(max_trait, 30)
    max_delegation = min(max_delegation, 20)
    
    # Header
    header = f"{'File':<{max_file}} | {'Struct':<{max_struct}} | {'Inherent Bounds':<{max_inherent}} | {'Trait Bounds':<{max_trait}} | {'Delegation':<{max_delegation}}"
    print(header)
    print("-" * len(header))
    
    # Data rows
    for d in all_data:
        file_str = d['file'][:max_file]
        struct_str = d['struct'][:max_struct]
        inherent_str = d['inherent'][:max_inherent]
        trait_str = d['trait'][:max_trait]
        delegation_str = d['delegation'][:max_delegation]
        
        print(f"{file_str:<{max_file}} | {struct_str:<{max_struct}} | {inherent_str:<{max_inherent}} | {trait_str:<{max_trait}} | {delegation_str:<{max_delegation}}")
    
    print("=" * 150)
    print(f"\nTotal: {len(all_data)} struct(s) with both inherent and trait impls")
    
    # Summary of bound mismatches
    mismatches = []
    for d in all_data:
        inherent = d['inherent']
        # Check if inherent has weaker bounds (missing StT or other complex bounds)
        if 'StT' in d['trait'] and 'StT' not in inherent:
            mismatches.append(f"{d['file']}: {d['struct']} (inherent missing StT)")
        elif 'MtT' in d['trait'] and 'MtT' not in inherent:
            mismatches.append(f"{d['file']}: {d['struct']} (inherent missing MtT)")
    
    if mismatches:
        print(f"\nPotential bound mismatches ({len(mismatches)}):")
        for m in mismatches:
            print(f"  - {m}")
    else:
        print("\nNo obvious bound mismatches detected (StT/MtT check)")
    
    # Summary of delegation patterns
    full_delegation = [d for d in all_data if 'Full delegation' in d['delegation']]
    partial_delegation = [d for d in all_data if 'Partial' in d['delegation']]
    direct_impl = [d for d in all_data if 'Direct impl' in d['delegation']]
    
    print(f"\nDelegation patterns:")
    print(f"  Full delegation: {len(full_delegation)} struct(s) - trait impl delegates to inherent impl")
    print(f"  Partial delegation: {len(partial_delegation)} struct(s) - some methods delegate")
    print(f"  Direct implementation: {len(direct_impl)} struct(s) - trait impl has own logic")
    
    if full_delegation:
        print(f"\n  Structs with FULL delegation (trait is redundant wrapper):")
        for d in full_delegation[:10]:
            print(f"    - {d['file']}: {d['struct']}")
        if len(full_delegation) > 10:
            print(f"    ... and {len(full_delegation) - 10} more")
    
    return 0


if __name__ == '__main__':
    sys.exit(main())

