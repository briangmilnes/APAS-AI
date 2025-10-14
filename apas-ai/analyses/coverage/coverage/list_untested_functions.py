#!/usr/bin/env python3
"""
Parse coverage JSON and list untested functions/methods by module.
Shows only base method names (no test contexts, closures, or type specializations).
Reads from coverage.json and writes to untested_functions_by_module.txt
"""

import json
import re
import subprocess
from pathlib import Path
from collections import defaultdict

def demangle_rust_name(mangled_name):
    """Demangle a Rust function name using rustfilt."""
    try:
        result = subprocess.run(
            ['rustfilt'],
            input=mangled_name,
            capture_output=True,
            text=True,
            timeout=1
        )
        if result.returncode == 0:
            return result.stdout.strip()
    except (subprocess.TimeoutExpired, FileNotFoundError):
        pass
    return mangled_name

def extract_method_name(demangled_name):
    """
    Extract ONLY the base method/function name.
    Examples:
      "<Foo as Trait>::insert::<TestContext::test_case::" -> "insert"
      "<Foo as Trait>::collect" -> "collect"
      "module::function" -> "function"
    """
    # Skip closures entirely
    if 'closure#' in demangled_name or '{closure' in demangled_name:
        return None
    
    # Pattern 1: Method in trait implementation: <Type as Trait>::method_name
    match = re.search(r'>::([a-z_][a-z0-9_]*)', demangled_name)
    if match:
        method = match.group(1)
        # Make sure it's not a module name (modules usually are capitalized or have :)
        if method and not method[0].isupper():
            return method
    
    # Pattern 2: Simple function: module::function_name
    parts = demangled_name.split('::')
    if parts:
        last_part = parts[-1]
        # Remove generic markers and anything after
        last_part = re.sub(r'<.*$', '', last_part)
        last_part = re.sub(r'\{.*$', '', last_part)
        last_part = last_part.strip()
        if last_part and not last_part[0].isupper():
            return last_part
    
    return None

def extract_module_from_filename(filename, workspace_path):
    """Extract module path from full filename, relative to workspace src/."""
    if workspace_path in filename:
        return filename.split(workspace_path)[-1]
    return None

def main():
    input_file = Path(__file__).parent / "coverage.json"
    output_file = Path(__file__).parent / "untested_functions_by_module.txt"
    
    # Get the actual workspace source path
    workspace_src = "/home/milnes/APASVERUS/APAS-AI/apas-ai/src/"
    
    if not input_file.exists():
        print(f"Error: {input_file} not found")
        return
    
    # Load JSON content as text for regex parsing
    with open(input_file, 'r') as f:
        content = f.read()
    
    # Parse coverage data
    # Map: module_path -> method_name -> set (for deduplication)
    untested_by_file = defaultdict(lambda: defaultdict(set))
    stats = {
        'total_files': set(),
        'total_instantiations': 0,
        'untested_instantiations': 0,
        'tested_instantiations': 0,
        'skipped': 0,
    }
    
    # Functions pattern
    function_pattern = r'"count":(\d+).*?"filenames":\["([^"]+)"\].*?"name":"([^"]+)"'
    
    print("Parsing coverage data and extracting base method names...")
    
    # Collect all mangled names first
    functions_to_process = []
    for match in re.finditer(function_pattern, content, re.DOTALL):
        count = int(match.group(1))
        filename = match.group(2)
        func_name = match.group(3)
        
        # ONLY include files from our workspace src directory
        if not filename.startswith(workspace_src):
            continue
        
        module_path = extract_module_from_filename(filename, workspace_src)
        if not module_path:
            continue
        
        functions_to_process.append((count, module_path, func_name))
    
    print(f"Found {len(functions_to_process)} function instantiations in workspace...")
    print("Demangling and extracting method names...")
    
    # Process in batch
    for count, module_path, func_name in functions_to_process:
        stats['total_files'].add(module_path)
        stats['total_instantiations'] += 1
        
        if count == 0:
            stats['untested_instantiations'] += 1
            # Demangle and extract base method name
            demangled = demangle_rust_name(func_name)
            method_name = extract_method_name(demangled)
            
            if method_name is None:
                # Skip closures and things we can't parse
                stats['skipped'] += 1
                continue
            
            # Use a set to deduplicate
            untested_by_file[module_path][method_name].add(demangled)
        else:
            stats['tested_instantiations'] += 1
    
    # Convert sets to counts
    untested_counts = {}
    for module, methods in untested_by_file.items():
        untested_counts[module] = {method: len(contexts) for method, contexts in methods.items()}
    
    # Sort modules
    sorted_modules = sorted(untested_counts.items())
    
    # Calculate unique functions
    total_unique_untested = sum(len(methods) for methods in untested_counts.values())
    
    # Write output
    with open(output_file, 'w') as f:
        f.write("=" * 100 + "\n")
        f.write("UNTESTED FUNCTIONS BY MODULE (Base Method Names Only)\n")
        f.write("=" * 100 + "\n\n")
        f.write(f"Summary:\n")
        f.write(f"  Total source files analyzed: {len(stats['total_files'])}\n")
        f.write(f"  Files with untested functions: {len(untested_counts)}\n")
        f.write(f"  Unique untested method names: {total_unique_untested}\n")
        f.write(f"  Total instantiations analyzed: {stats['total_instantiations']}\n")
        f.write(f"  Closures/unparseable skipped: {stats['skipped']}\n")
        if stats['total_instantiations'] > 0:
            f.write(f"  Tested instantiations: {stats['tested_instantiations']} ({100*stats['tested_instantiations']/stats['total_instantiations']:.1f}%)\n")
            f.write(f"  Untested instantiations: {stats['untested_instantiations']} ({100*stats['untested_instantiations']/stats['total_instantiations']:.1f}%)\n")
        f.write("\n" + "=" * 100 + "\n\n")
        
        for module_path, untested_methods in sorted_modules:
            f.write(f"\n{'=' * 100}\n")
            f.write(f"MODULE: {module_path}\n")
            f.write(f"{'=' * 100}\n")
            f.write(f"Untested methods: {len(untested_methods)}\n\n")
            
            # Sort by method name
            sorted_methods = sorted(untested_methods.items())
            
            for i, (method_name, count) in enumerate(sorted_methods, 1):
                if count > 1:
                    f.write(f"  {i:3d}. {method_name} [{count} contexts]\n")
                else:
                    f.write(f"  {i:3d}. {method_name}\n")
        
        f.write("\n" + "=" * 100 + "\n")
        f.write("END OF REPORT\n")
        f.write("=" * 100 + "\n")
    
    print(f"✓ Analyzed {len(stats['total_files'])} source files from workspace")
    print(f"✓ Found {total_unique_untested} unique untested method names")
    print(f"✓ Skipped {stats['skipped']} closures/unparseable items")
    print(f"✓ Full report written to: {output_file}")
    
    if untested_counts:
        print(f"\nAll modules with untested methods:")
        sorted_by_count = sorted(
            [(module, len(methods)) for module, methods in untested_counts.items()],
            key=lambda x: x[1],
            reverse=True
        )
        for i, (module, count) in enumerate(sorted_by_count, 1):
            print(f"  {i:3d}. {count:3d} untested methods - {module}")

if __name__ == "__main__":
    main()
