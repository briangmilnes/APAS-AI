#!/bin/bash

cd ~/APASVERUS/APAS-AI/apas-ai/

echo "src wheres"
grep where src/*.rs | wc -l

echo "tests wheres"
grep where tests/*.rs | wc -l

echo "benches wheres"
grep where benches/*.rs | wc -l

echo "Total wheres"
grep where src/*.rs tests/*.rs benches/*.rs | wc -l
