#!/bin/bash


cd ~/APASVERUS/APAS-AI/apas-ai/

echo "src counts"
grep ' as ' src/*.rs | wc -l

echo "tests counts"
grep ' as ' tests/*.rs | wc -l

echo "benches counts"
grep ' as ' benches/*.rs | wc -l 

echo "total <Type as Traits>"
grep ' as ' src/*.rs tests/*.rs benches/*.rs | wc -l 

echo "LOC" 
wc -l src/*.rs tests/*.rs benches/*.rs

