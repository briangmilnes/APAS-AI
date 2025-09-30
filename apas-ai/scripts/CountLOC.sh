#!/bin/bash

cd ~/APASVERUS/APAS-AI/apas-ai/

echo "Total LOC"
wc -l src/*.rs src/*/*.rs tests/*.rs tests/*/*.rs  benches/*/*.rs 
