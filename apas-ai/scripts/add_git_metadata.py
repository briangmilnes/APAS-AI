#!/usr/bin/env python3
"""
Add git commit metadata to all Python scripts in the scripts/ directory.
This enables rollback and reapplication of scripts during research.

Per RustRules.md: Script Metadata and Version Control (MANDATORY)
"""

# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700

import subprocess
from pathlib import Path
from datetime import datetime
import re


def get_current_commit_info():
    """Get the current git commit hash and date."""
    try:
        commit_hash = subprocess.check_output(
            ['git', 'rev-parse', 'HEAD'],
            text=True
        ).strip()
        
        commit_date = subprocess.check_output(
            ['git', 'show', '-s', '--format=%ci', 'HEAD'],
            text=True
        ).strip()
        
        return commit_hash, commit_date
    except subprocess.CalledProcessError as e:
        print(f"Error getting git info: {e}")
        return None, None


def get_file_commit_info(filepath):
    """Get the last commit that modified this file."""
    try:
        commit_hash = subprocess.check_output(
            ['git', 'log', '-1', '--format=%H', '--', str(filepath)],
            text=True
        ).strip()
        
        if not commit_hash:
            # File not yet committed
            return get_current_commit_info()
        
        commit_date = subprocess.check_output(
            ['git', 'log', '-1', '--format=%ci', '--', str(filepath)],
            text=True
        ).strip()
        
        return commit_hash, commit_date
    except subprocess.CalledProcessError:
        return get_current_commit_info()


def add_or_update_git_metadata(filepath, commit_hash, commit_date):
    """Add or update git metadata in a Python script."""
    content = filepath.read_text()
    lines = content.split('\n')
    
    # Find where to insert metadata
    insert_idx = 0
    has_shebang = False
    has_docstring = False
    docstring_end = 0
    
    # Check for shebang
    if lines and lines[0].startswith('#!'):
        has_shebang = True
        insert_idx = 1
    
    # Check for module docstring
    if len(lines) > insert_idx:
        # Look for docstring starting after shebang
        start_idx = insert_idx
        if start_idx < len(lines) and lines[start_idx].strip().startswith('"""'):
            has_docstring = True
            # Find end of docstring
            if lines[start_idx].strip().count('"""') == 2:
                # Single-line docstring
                docstring_end = start_idx + 1
            else:
                # Multi-line docstring
                for i in range(start_idx + 1, len(lines)):
                    if '"""' in lines[i]:
                        docstring_end = i + 1
                        break
            insert_idx = docstring_end
    
    # Check if metadata already exists
    metadata_pattern = r'# Git commit: [0-9a-f]{40}'
    existing_metadata_idx = None
    
    for i in range(insert_idx, min(insert_idx + 5, len(lines))):
        if i < len(lines) and re.match(metadata_pattern, lines[i]):
            existing_metadata_idx = i
            break
    
    # Prepare metadata lines
    metadata = [
        f'# Git commit: {commit_hash}',
        f'# Date: {commit_date}',
    ]
    
    if existing_metadata_idx is not None:
        # Update existing metadata
        lines[existing_metadata_idx] = metadata[0]
        if existing_metadata_idx + 1 < len(lines) and lines[existing_metadata_idx + 1].startswith('# Date:'):
            lines[existing_metadata_idx + 1] = metadata[1]
        else:
            lines.insert(existing_metadata_idx + 1, metadata[1])
        modified = True
    else:
        # Insert new metadata
        # Add blank line if needed
        if insert_idx < len(lines) and lines[insert_idx].strip():
            metadata.insert(0, '')
        metadata.append('')
        
        for i, line in enumerate(metadata):
            lines.insert(insert_idx + i, line)
        modified = True
    
    if modified:
        filepath.write_text('\n'.join(lines))
        return True
    return False


def main():
    """Add git metadata to all Python scripts."""
    scripts_dir = Path('scripts')
    
    if not scripts_dir.exists():
        print("Error: scripts/ directory not found")
        return
    
    # Get all Python files
    python_files = list(scripts_dir.rglob('*.py'))
    
    print(f"Found {len(python_files)} Python scripts")
    print()
    
    updated_count = 0
    skipped_count = 0
    
    for py_file in sorted(python_files):
        # Get commit info for this specific file
        commit_hash, commit_date = get_file_commit_info(py_file)
        
        if not commit_hash:
            print(f"⚠ Skipped: {py_file} (no git info)")
            skipped_count += 1
            continue
        
        # Check if file already has current metadata
        content = py_file.read_text()
        if f'# Git commit: {commit_hash}' in content:
            print(f"- Already current: {py_file.relative_to(scripts_dir)}")
            continue
        
        # Add/update metadata
        if add_or_update_git_metadata(py_file, commit_hash, commit_date):
            # Get short hash for display
            short_hash = commit_hash[:7]
            print(f"✓ Updated: {py_file.relative_to(scripts_dir)} ({short_hash})")
            updated_count += 1
        else:
            print(f"- No change: {py_file.relative_to(scripts_dir)}")
    
    print()
    print(f"{'='*60}")
    print(f"Updated: {updated_count} files")
    print(f"Skipped: {skipped_count} files")
    print(f"Total: {len(python_files)} files")
    print(f"{'='*60}")


if __name__ == '__main__':
    main()


