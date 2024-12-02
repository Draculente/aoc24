#!/bin/bash

if [ "$#" -ne 1 ]; then
    echo "Illegal number of parameters"
    exit 1
fi

session="${AOC_SESSION}"

if [[ -z "$session" ]]; then
    echo "No AOC_SESSION specified"
    exit 2
fi

curl -o "./inputs/day$1.txt" --cookie "session=$session" "https://adventofcode.com/2024/day/$1/input"
