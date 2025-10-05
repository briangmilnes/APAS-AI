#!/usr/bin/env python3
import json
import sys

# Read the JSON coverage file
with open('target/llvm-cov/coverage.json', 'r') as f:
    data = json.load(f)

# Extract function coverage data
functions = []
for file_data in data['data'][0]['files']:
    filename = file_data['filename']
    for func in file_data.get('functions', []):
        func_name = func['name']
        count = func['count']
        regions = func.get('regions', [])

        # Calculate coverage percentage for regions
        if regions:
            covered = sum(1 for r in regions if r[4] > 0)  # r[4] is execution count
            total = len(regions)
            coverage_pct = (covered / total * 100) if total > 0 else 0
        else:
            coverage_pct = 100 if count > 0 else 0

        functions.append({
            'file': filename,
            'function': func_name,
            'count': count,
            'coverage': coverage_pct
        })

# Sort by coverage percentage (ascending - least covered first)
functions.sort(key=lambda x: x['coverage'])

# Print results
print(f"{'Coverage %':<12} {'Exec Count':<12} {'Function':<60} File")
print("=" * 150)

for func in functions:
    # Truncate long function names
    func_name = func['function'][:58]
    file_name = func['file'].replace('/home/milnes/APASVERUS/APAS-AI/apas-ai/', '')

    print(f"{func['coverage']:>10.1f}% {func['count']:>11} {func_name:<60} {file_name}")

# Print summary
print("\n" + "=" * 150)
print(f"Total functions: {len(functions)}")
uncovered = sum(1 for f in functions if f['coverage'] == 0)
partial = sum(1 for f in functions if 0 < f['coverage'] < 100)
fully_covered = sum(1 for f in functions if f['coverage'] == 100)
print(f"Uncovered (0%): {uncovered}")
print(f"Partially covered (0-100%): {partial}")
print(f"Fully covered (100%): {fully_covered}")
