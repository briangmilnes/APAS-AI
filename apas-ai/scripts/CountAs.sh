#!/bin/bash


cd ~/APASVERUS/APAS-AI/apas-ai/

echo "src counts"
grep ' as ' src/*.rs | wc -l

echo "tests counts"
grep ' as ' tests/*.rs | wc -l

echo "benches counts"
grep ' as ' benches/*.rs | wc -l 
