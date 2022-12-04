#!/usr/bin/env sh

SESSION="$(cat .session)"

curl "https://adventofcode.com/2022/day/$1/input" --cookie "session=$SESSION" -o "inputs/$1.in"
