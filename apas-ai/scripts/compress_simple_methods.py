#!/usr/bin/env python3
"""Compress simple method bodies onto single lines if under 120 chars.

Git commit: [current]
Date: 2025-10-19

Finds methods with simple Self { ... } bodies and compresses them to single lines
if the total length is under 120 characters. This prevents rustfmt from expanding them.

Usage:
  compress_simple_methods.py [file_or_directory]
  compress_simple_methods.py                    # processes entire src/ directory
"""

import sys
import re
from pathlib import Path


def compress_simple_methods(content):
    """Compress simple method bodies to single lines if under 120 chars."""
    lines = content.split('\n')
    result = []
    i = 0
    
    while i < len(lines):
        line = lines[i]
        
        # Look for method definitions: "fn name(...) -> ReturnType {"
        if 'fn ' in line and '->' in line and line.rstrip().endswith('{'):
            # Check if next line(s) contain a simple Self { ... } body
            # Pattern: "Self { field: value, ... }"
            # Look ahead up to 5 lines for the body
            method_start = i
            indent = len(line) - len(line.lstrip())
            
            # Collect the full method (could be 2-4 lines)
            method_lines = [line]
            j = i + 1
            brace_count = line.count('{') - line.count('}')
            
            while j < len(lines) and brace_count > 0:
                next_line = lines[j]
                method_lines.append(next_line)
                brace_count += next_line.count('{') - next_line.count('}')
                j += 1
                if j - method_start > 5:  # Don't look too far
                    break
            
            # Check if this is a simple Self { ... } body
            full_method = '\n'.join(method_lines)
            
            # Simple pattern: method ends with "Self { ... }" or "Self { ... } }"
            # Remove all whitespace to check structure
            compressed = re.sub(r'\s+', ' ', full_method.strip())
            
            # Check if it's a simple constructor pattern
            if 'Self {' in compressed and brace_count == 0:
                # Try to compress to single line
                compressed_line = compressed
                
                # If under 120 chars, use it
                if len(compressed_line) <= 120:
                    # Preserve original indentation
                    indented_line = ' ' * indent + compressed_line
                    result.append(indented_line)
                    i = j  # Skip the lines we just compressed
                    continue
            
            # Otherwise, keep original lines
            result.extend(method_lines)
            i = j
        else:
            result.append(line)
            i += 1
    
    return '\n'.join(result)


def process_file(filepath):
    """Process a single Rust file."""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        modified = compress_simple_methods(content)
        
        if modified != content:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(modified)
            print(f"✓ Compressed: {filepath}")
            return 1
        else:
            return 0
    except Exception as e:
        print(f"✗ Error processing {filepath}: {e}", file=sys.stderr)
        return 0


def main():
    if len(sys.argv) > 1:
        target = Path(sys.argv[1])
    else:
        target = Path('src')
    
    if not target.exists():
        print(f"Error: {target} does not exist", file=sys.stderr)
        return 1
    
    files_modified = 0
    
    if target.is_file():
        files_modified = process_file(target)
    else:
        # Process all .rs files recursively
        for filepath in sorted(target.rglob('*.rs')):
            files_modified += process_file(filepath)
    
    print(f"\n{files_modified} file(s) modified")
    return 0


if __name__ == "__main__":
    sys.exit(main())


