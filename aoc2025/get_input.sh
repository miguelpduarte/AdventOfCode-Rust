#!/usr/bin/env sh

SESSION="$(cat .session)"

# To play nice with our macro, the file name should be zero-padded (i.e. 01.in rather than 1.in)
printf -v out_file_name "%02d" "$1"

curl "https://adventofcode.com/2025/day/$1/input" --cookie "session=$SESSION" -o "inputs/$out_file_name.in"
