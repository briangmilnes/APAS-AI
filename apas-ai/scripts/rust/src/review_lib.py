#!/usr/bin/env python3
"""
Review lib.rs to ensure all src modules are declared.
Handles nested module structure like:
  pub mod ChapXX {
      pub mod ModuleName;
  }
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import sys
from pathlib import Path
import re

def parse_nested_modules(content):
    """Parse nested module declarations from lib.rs"""
    declared = {}  # chapter -> set of modules
    
    # Match: pub mod ChapXX { ... }
    chapter_pattern = re.compile(
        r'pub\s+mod\s+(\w+)\s*\{([^}]*)\}',
        re.MULTILINE | re.DOTALL
    )
    
    for match in chapter_pattern.finditer(content):
        chapter = match.group(1)
        chapter_content = match.group(2)
        
        # Extract module names within this chapter
        mod_pattern = re.compile(r'pub\s+mod\s+(\w+)\s*;')
        modules = {m.group(1) for m in mod_pattern.finditer(chapter_content)}
        declared[chapter] = modules
    
    # Also handle top-level module declarations like "pub mod Types;"
    top_level_pattern = re.compile(r'^pub\s+mod\s+(\w+)\s*;', re.MULTILINE)
    top_level = set()
    for match in top_level_pattern.finditer(content):
        mod_name = match.group(1)
        # Only include if not a chapter module
        if mod_name not in declared:
            top_level.add(mod_name)
    
    return declared, top_level

def find_actual_modules(src_dir):
    """Find all actual module files in src directory"""
    actual = {}  # chapter -> set of modules
    top_level_files = set()
    
    for path in src_dir.iterdir():
        if path.is_dir() and not path.name.startswith('.'):
            # Chapter directory - find all .rs files within
            chapter = path.name
            modules = {f.stem for f in path.glob("*.rs")}
            if modules:
                actual[chapter] = modules
        elif path.is_file() and path.suffix == '.rs' and path.name not in ['lib.rs', 'main.rs']:
            # Top-level module file
            top_level_files.add(path.stem)
    
    return actual, top_level_files

def main():
    repo_root = Path(__file__).parent.parent.parent.parent
    lib_rs = repo_root / "src" / "lib.rs"
    src_dir = repo_root / "src"
    
    # Read lib.rs
    with open(lib_rs) as f:
        lib_content = f.read()
    
    declared_chapters, declared_top = parse_nested_modules(lib_content)
    actual_chapters, actual_top = find_actual_modules(src_dir)
    
    errors = []
    
    # Check top-level modules
    missing_top = actual_top - declared_top
    extra_top = declared_top - actual_top
    
    if missing_top:
        errors.append("❌ Top-level modules not declared in lib.rs:")
        for mod in sorted(missing_top):
            errors.append(f"   pub mod {mod};")
    
    if extra_top:
        errors.append("❌ Top-level declarations without corresponding files:")
        for mod in sorted(extra_top):
            errors.append(f"   {mod} (expected src/{mod}.rs)")
    
    # Check chapter modules
    all_chapters = sorted(set(declared_chapters.keys()) | set(actual_chapters.keys()))
    
    for chapter in all_chapters:
        declared_mods = declared_chapters.get(chapter, set())
        actual_mods = actual_chapters.get(chapter, set())
        
        missing = actual_mods - declared_mods
        extra = declared_mods - actual_mods
        
        if missing:
            errors.append(f"❌ Modules in src/{chapter}/ not declared in lib.rs:")
            for mod in sorted(missing):
                errors.append(f"   pub mod {mod}; // in pub mod {chapter} block")
        
        if extra:
            errors.append(f"❌ Modules declared in {chapter} without corresponding files:")
            for mod in sorted(extra):
                errors.append(f"   {mod} (expected src/{chapter}/{mod}.rs)")
    
    if errors:
        for line in errors:
            print(line)
        
        # Count violations for pareto analysis
        violation_count = 0
        for line in errors:
            if line.startswith("   pub mod "):
                violation_count += 1
            elif line.startswith("   ") and "(expected " in line:
                violation_count += 1
        
        print(f"\nTotal violations: {violation_count}")
        return 1
    
    total_declared = len(declared_top) + sum(len(mods) for mods in declared_chapters.values())
    print(f"✓ All {total_declared} source modules properly declared in lib.rs")
    return 0

if __name__ == "__main__":
    sys.exit(main())

