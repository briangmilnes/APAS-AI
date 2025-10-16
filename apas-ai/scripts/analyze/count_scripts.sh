#!/bin/bash

cd ~/APASVERUS/APAS-AI/apas-ai/scripts

echo "Python Review Scripts"
find . -name "review*.py" | wc -l

echo "Python Fix Scripts"
find . -name "fix*.py" | wc -l

