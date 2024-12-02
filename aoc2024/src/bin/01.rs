#![feature(test)]
extern crate test;

use std::collections::{BinaryHeap, HashMap};

fn solve_day(input: String) -> (usize, usize) {
    let mut left_nums = BinaryHeap::new();
    let mut right_nums = BinaryHeap::new();

    let mut left_occurrences: HashMap<u64, u64> = HashMap::new();
    let mut right_occurrences: HashMap<u64, u64> = HashMap::new();

    for line in input.lines() {
        let (left, right) = line
            .split_once("   ")
            .expect("Exactly 3 spaces between numbers");
        let left = left.parse::<u64>().expect("is number");
        left_nums.push(left);
        let right = right.parse::<u64>().expect("is number");
        right_nums.push(right);

        *left_occurrences.entry(left).or_default() += 1;
        *right_occurrences.entry(right).or_default() += 1;
    }

    let mut sum = 0;

    while let (Some(min_left), Some(min_right)) = (left_nums.pop(), right_nums.pop()) {
        sum += min_right.abs_diff(min_left);
    }

    let p1 = sum;

    let p2: u64 = left_occurrences
        .into_iter()
        .map(|(left_num, left_num_count)| {
            // Calc "value" of line - similarity score increase
            // The value of each line would be the left number's value times the number of times it
            // appears on the right list.
            // Since we already counted how many times the left number shows up on the left as well, we
            // can roll up both by doing: left num val * left num occurrences * left num occurrences in
            // right list
            let count_in_right = right_occurrences
                .get(&left_num)
                .copied()
                .unwrap_or_default();
            left_num * left_num_count * count_in_right
        })
        .sum();

    (
        p1.try_into().expect("u64 > usize"),
        p2.try_into().expect("u64 > usize"),
    )
}

#[test]
fn example_input() {
    let input = "3   4
4   3
2   5
1   3
3   9
3   3"
        .to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 11);
    assert_eq!(res.1, 31);
}

#[test]
fn prod_solution() {
    use std::fs::read_to_string;

    let input = read_to_string(format!("inputs/{}", "1.in")).unwrap();
    let res = solve_day(input);
    assert_eq!(res.0, 2113135);
    assert_eq!(res.1, 19097157);
}

aoc2024::day_main!("1.in");
