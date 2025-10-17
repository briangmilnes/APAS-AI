#!/usr/bin/env python3
"""
Identify key data structure files where the primary struct doesn't match the filename.

Filters out:
- Helper structs in multi-struct files
- Utility files (Types.rs, HashFunctionTraits.rs, etc.)
- Example/analysis files
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import subprocess
import re

# Run the review script
result = subprocess.run(
    ['python3', 'scripts/rust/src/review_struct_file_naming.py'],
    capture_output=True, text=True, cwd='/home/milnes/APASVERUS/APAS-AI/apas-ai'
)

violations = []
for line in result.stderr.split('\n'):
    if line.strip().startswith('src/') and ' - struct ' in line:
        violations.append(line.strip())

# Categorize violations
key_data_structures = []
utility_files = []

# Files to skip (multi-struct utility files)
skip_files = ['Types.rs', 'HashFunctionTraits.rs', 'ClusteringAnalysis.rs', 
              'ProbeSequenceExamples.rs', 'AdvancedDoubleHashing.rs',
              'AdvancedLinearProbing.rs', 'AdvancedQuadraticProbing.rs',
              'Example44_1.rs', 'DocumentIndex.rs', 'ParaHashTableStEph.rs',
              'StructChainedHashTable.rs', 'ChainedHashTable.rs']

# Helper struct names to skip
skip_structs = ['Node', 'ChainList', 'ChainEntry', 'LoadAndSize', 'QueryBuilder',
                'ClusteringPerformanceImpact', 'ClusteringComparison', 'PQMinResult',
                'ClosurePriority', 'HashTable', 'DoubleHashingMetrics', 'RelativePrimeValidator',
                'ProbeSequenceVisualization', 'TextbookExampleResults', 'ProbeSequenceAnalyzer',
                'PrimaryClusteringMetrics', 'SecondaryClusteringMetrics', 'PrimeValidator',
                'DefaultHashFunction', 'StringPositionHashFunction', 'PolynomialHashFunction',
                'UniversalIntegerHashFunction', 'DefaultKeyEquality', 'CaseInsensitiveStringEquality',
                'UniversalIntegerHashFamily', 'ProbeSequenceGenerator', 'LoadFactorManager',
                'HashTableStats', 'HashTableUtils', 'HashFunctionTester', 'TweetQueryExamples',
                'ComprehensiveClusteringAnalysis', 'ClusteringAnalyzer']

for violation in violations:
    # Extract filename and struct name
    match = re.search(r'src/([^:]+):\d+ - struct \'(\w+)\'', violation)
    if not match:
        continue
    
    filepath = match.group(1)
    filename = filepath.split('/')[-1]
    struct_name = match.group(2)
    
    # Skip utility files
    if filename in skip_files:
        continue
    
    # Skip helper structs
    if struct_name in skip_structs:
        continue
    
    # These are likely key data structures
    key_data_structures.append((filepath, struct_name, filename))

print("=" * 80)
print("KEY DATA STRUCTURES WITH NAME MISMATCHES")
print("=" * 80)
print()

# Group by pattern
suffix_mismatches = []  # Struct has StEph suffix, file doesn't
prefix_mismatches = []  # Different name entirely
strategy_suffix = []    # File is Foo.rs, struct is FooStrategy

for filepath, struct_name, filename in key_data_structures:
    file_stem = filename.replace('.rs', '')
    
    if struct_name.endswith('StEph') and not file_stem.endswith('StEph'):
        suffix_mismatches.append((filepath, struct_name, file_stem))
    elif struct_name.endswith('Strategy') and not file_stem.endswith('Strategy'):
        strategy_suffix.append((filepath, struct_name, file_stem))
    else:
        prefix_mismatches.append((filepath, struct_name, file_stem))

if suffix_mismatches:
    print("1. FILES MISSING StEph/MtEph SUFFIX:")
    print("-" * 80)
    for filepath, struct_name, file_stem in sorted(suffix_mismatches):
        print(f"  {filepath}")
        print(f"    struct: {struct_name}")
        print(f"    file:   {file_stem}")
        print()

if strategy_suffix:
    print("2. FILES MISSING 'Strategy' SUFFIX:")
    print("-" * 80)
    for filepath, struct_name, file_stem in sorted(strategy_suffix):
        print(f"  {filepath}")
        print(f"    struct: {struct_name}")
        print(f"    file:   {file_stem}")
        print()

if prefix_mismatches:
    print("3. COMPLETELY DIFFERENT NAMES:")
    print("-" * 80)
    for filepath, struct_name, file_stem in sorted(prefix_mismatches):
        print(f"  {filepath}")
        print(f"    struct: {struct_name}")
        print(f"    file:   {file_stem}")
        print()

print("=" * 80)
print(f"TOTAL KEY DATA STRUCTURES WITH MISMATCHES: {len(key_data_structures)}")
print("=" * 80)

