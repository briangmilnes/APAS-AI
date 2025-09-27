#!/bin/bash


cd ~/APASVERUS/APAS-AI/apas-ai/

echo "src counts"
grep 'vec' src/*.rs src/*/*.rs | wc -l

echo "tests counts"
grep 'vec' tests/*.rs tests/*/*.rs | wc -l

echo "tests counts vec!"
grep 'vec\!' tests/*.rs tests/*/*.rs | wc -l

echo "benches counts"
grep 'vec' benches/*/*.rs | wc -l 

echo "total vec counts"
grep 'vec' src/*.rs src/*/*.rs tests/*.rs tests/*/*.rs benches/*/*.rs | wc -l

