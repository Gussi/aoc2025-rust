#!/usr/bin/bash

if [ -z "$AOC_SESSION" ]; then
    echo "Error: AOC_SESSION environment variable is not set."
    exit 1
fi

for i in {1..12}; do
    DAY=$(printf "%02d" $i)
    TODAY=$(date +%d)
    if [ $i -gt $TODAY ]; then
        break
    fi

    if [ ! -f "inputs/day${DAY}_input.txt" ]; then
        echo "Downloading input for Day $i"
        curl -s -H "Cookie: session=$AOC_SESSION" "https://adventofcode.com/2025/day/$i/input" -o "inputs/day${DAY}_input.txt"
    else
        echo "Input for Day $i already exists, skipping download."
    fi
done
