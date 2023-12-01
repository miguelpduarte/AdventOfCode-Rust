#!/usr/bin/env sh

SESSION="$(cat .session)"

curl "https://adventofcode.com/2023/day/$1/input" --cookie "session=$SESSION" -o "inputs/$1.in"
