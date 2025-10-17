#!/usr/bin/env python3
"""Batch convert remaining 6 weighted graph files to use Pair/Triple wrappers."""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import re
from pathlib import Path

def convert_graph_file(filepath):
    """Convert a single graph file."""
    with open(filepath) as f:
        content = f.read()
    
    original = content
    
    # Convert type signatures
    content = re.sub(r'SetStEph<\(V, V, (i32|OrderedFloat<f64>)\)>', r'SetStEph<Triple<V, V, \1>>', content)
    content = re.sub(r'SetStEph<\(V, (i32|OrderedFloat<f64>)\)>', r'SetStEph<Pair<V, \1>>', content)
    
    # Convert tuple construction to Triple
    content = re.sub(
        r'edges\.insert\(\(labeled_edge\.0\.clone\(\), labeled_edge\.1\.clone\(\), labeled_edge\.2\)\)',
        r'edges.insert(Triple(labeled_edge.0.clone(), labeled_edge.1.clone(), labeled_edge.2))',
        content
    )
    
    # Convert tuple construction to Pair for neighbors
    content = re.sub(
        r'neighbors\.insert\(\(labeled_edge\.(0|1)\.clone\(\), labeled_edge\.2\)\)',
        r'neighbors.insert(Pair(labeled_edge.\1.clone(), labeled_edge.2))',
        content
    )
    
    # Convert tuple destructuring in map for from_weighted_edges
    content = re.sub(
        r'\.map\(\|\(from, to, weight\)\| LabEdge\(from\.clone\(\), to\.clone\(\), \*weight\)\)',
        r'.map(|Triple(from, to, weight)| LabEdge(from.clone(), to.clone(), *weight))',
        content
    )
    
    # Convert macro tuple patterns to use Triple wrapper
    content = re.sub(
        r'(\$crate::SetLit!\[\s*\$\(\s*)\(\$from, \$to, \$weight\)(\s*\),\*\s*\]\)',
        r'\1Triple($from, $to, $weight)\2',
        content
    )
    
    if content != original:
        with open(filepath, 'w') as f:
            f.write(content)
        return True
    return False

if __name__ == '__main__':
    repo_root = Path(__file__).parent.parent.parent
    src_files = [
        'src/Chap06/WeightedUnDirGraphStEphFloat.rs',
        'src/Chap06/WeightedUnDirGraphStEphInt.rs',
        'src/Chap06/WeightedDirGraphMtEphFloat.rs',
        'src/Chap06/WeightedDirGraphMtEphInt.rs',
        'src/Chap06/WeightedUnDirGraphMtEphFloat.rs',
        'src/Chap06/WeightedUnDirGraphMtEphInt.rs',
    ]
    
    for src_file in src_files:
        filepath = repo_root / src_file
        if filepath.exists():
            if convert_graph_file(filepath):
                print(f'Converted: {src_file}')
            else:
                print(f'No changes: {src_file}')
        else:
            print(f'Not found: {src_file}')


