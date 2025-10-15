#!/bin/bash

cd ~/APASVERUS/APAS-AI/apas-ai/

echo "SRC LOC"
wc -l src/*.rs src/*/*.rs  | grep total 

echo "Tests LOC"
wc -l tests/*.rs tests/*/*.rs | grep total

echo "Benches LOC"
wc -l benches/*/*.rs | grep total

echo "Scripts LOC"
find scripts -name "*.py" -or -name "*.sh" | xargs wc -l | grep total 

echo "Total LOC"
find . -name "*.py" -or -name "*.sh" -or -name "*.rs" | xargs wc -l | grep total 
