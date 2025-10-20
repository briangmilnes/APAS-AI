#!/usr/bin/env python3
"""
Find inherent impl blocks that contain ONLY private helper methods/functions.
These can be eliminated by moving their contents to module-level functions.
"""
import re
from pathlib import Path

project_root = Path("/home/milnes/APASVERUS/APAS-AI/apas-ai")
src_dir = project_root / "src"

def find_inherent_impls(content):
    """Find all inherent impl blocks in the content."""
    impls = []
    lines = content.split('\n')
    
    i = 0
    while i < len(lines):
        line = lines[i]
        
        # Look for impl blocks (must have 4 space indent)
        if re.match(r'^    impl', line):
            # Check if it's a trait impl (has 'for' keyword)
            is_trait_impl = False
            
            # Check current line
            if ' for ' in line:
                is_trait_impl = True
            elif not line.rstrip().endswith('{'):
                # Check next few lines for 'for'
                for j in range(i + 1, min(i + 5, len(lines))):
                    next_line = lines[j].strip()
                    if next_line.startswith('for '):
                        is_trait_impl = True
                        break
                    if next_line.endswith('{'):
                        break
            
            if not is_trait_impl:
                # Found an inherent impl
                start_line = i + 1  # 1-indexed (line number for reporting)
                
                # Extract struct name
                match = re.search(r'impl(?:<[^>]+>)?\s+(\w+)', line)
                struct_name = match.group(1) if match else "Unknown"
                
                # Find the closing brace
                brace_count = 0
                impl_lines = []
                end_line = i + 1  # Default if we can't find closing brace
                for j in range(i, len(lines)):
                    impl_lines.append(lines[j])
                    brace_count += lines[j].count('{') - lines[j].count('}')
                    if brace_count == 0 and '{' in lines[j]:
                        end_line = j + 1  # 1-indexed
                        break
                
                impls.append({
                    'start': start_line,
                    'end': end_line,
                    'struct': struct_name,
                    'content': '\n'.join(impl_lines)
                })
        
        i += 1
    
    return impls

def analyze_impl_block(impl_content):
    """Analyze an impl block to see if it's all private helpers."""
    lines = impl_content.split('\n')
    
    has_pub_fn = False
    has_private_fn = False
    methods = []
    
    i = 0
    while i < len(lines):
        line = lines[i]
        
        # Look for function definitions (8 space indent for methods)
        if re.match(r'\s{8}(pub\s+)?fn\s+', line):
            is_pub = 'pub fn' in line
            
            # Extract function name
            match = re.search(r'fn\s+(\w+)', line)
            fn_name = match.group(1) if match else "unknown"
            
            # Check if it's a method (has &self or &mut self)
            is_method = '&self' in line or '&mut self' in line
            
            if is_pub:
                has_pub_fn = True
            else:
                has_private_fn = True
            
            methods.append({
                'name': fn_name,
                'is_pub': is_pub,
                'is_method': is_method,
                'line': line.strip()
            })
        
        i += 1
    
    return {
        'has_pub': has_pub_fn,
        'has_private': has_private_fn,
        'only_private_helpers': has_private_fn and not has_pub_fn,
        'methods': methods
    }

def main():
    print("INHERENT IMPL BLOCKS WITH ONLY PRIVATE HELPERS")
    print("=" * 80)
    print("These can be eliminated by moving their contents to module-level functions.")
    print()
    
    only_private_count = 0
    mixed_count = 0
    only_pub_count = 0
    
    only_private_files = []
    mixed_files = []
    
    for rs_file in sorted(src_dir.rglob("*.rs")):
        if "Types.rs" in str(rs_file):
            continue
        
        content = rs_file.read_text()
        impls = find_inherent_impls(content)
        
        if not impls:
            continue
        
        for impl_info in impls:
            analysis = analyze_impl_block(impl_info['content'])
            
            rel_path = rs_file.relative_to(project_root)
            
            if analysis['only_private_helpers']:
                only_private_count += 1
                only_private_files.append({
                    'file': str(rel_path),
                    'line': impl_info['start'],
                    'struct': impl_info['struct'],
                    'methods': analysis['methods']
                })
            elif analysis['has_pub'] and analysis['has_private']:
                mixed_count += 1
                mixed_files.append({
                    'file': str(rel_path),
                    'line': impl_info['start'],
                    'struct': impl_info['struct'],
                    'methods': analysis['methods']
                })
            elif analysis['has_pub'] and not analysis['has_private']:
                only_pub_count += 1
    
    # Print only private helpers
    print(f"ONLY PRIVATE HELPERS (can eliminate) - {only_private_count} blocks")
    print("-" * 80)
    for item in only_private_files:
        print(f"\n{item['file']}:{item['line']}")
        print(f"  impl {item['struct']} {{")
        for method in item['methods']:
            method_or_fn = "method" if method['is_method'] else "function"
            print(f"    - {method['name']}() [{method_or_fn}]")
    
    print()
    print()
    print(f"MIXED (has both pub and private) - {mixed_count} blocks")
    print("-" * 80)
    print("These need the private helpers extracted, public methods stay in trait.")
    for item in mixed_files[:15]:  # Limit output
        print(f"\n{item['file']}:{item['line']}")
        print(f"  impl {item['struct']} {{")
        pub_methods = [m for m in item['methods'] if m['is_pub']]
        priv_methods = [m for m in item['methods'] if not m['is_pub']]
        print(f"    PUBLIC: {', '.join(m['name'] for m in pub_methods)}")
        print(f"    PRIVATE: {', '.join(m['name'] for m in priv_methods)}")
    if mixed_count > 15:
        print(f"\n... and {mixed_count - 15} more mixed blocks")
    
    print()
    print()
    print("=" * 80)
    print("SUMMARY:")
    print(f"  Only private helpers (ELIMINATE these blocks): {only_private_count}")
    print(f"  Mixed pub/private (extract private helpers): {mixed_count}")
    print(f"  Only public (move to trait): {only_pub_count}")
    print()
    print("EASY FIXES: The {0} 'only private' blocks can be fully eliminated.".format(only_private_count))
    print("HARDER: The {0} 'mixed' blocks need private methods extracted.".format(mixed_count))

if __name__ == '__main__':
    main()

