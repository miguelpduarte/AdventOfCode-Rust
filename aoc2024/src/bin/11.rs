#![feature(test)]

use std::collections::HashMap;
extern crate test;

fn solve_day(input: String) -> (usize, usize) {
    // Maps <stone_value, stone_count>
    let mut stones_frequency: HashMap<usize, usize> = HashMap::new();

    for stone_value in input
        .trim_end()
        .split(' ')
        .map(|item| item.parse::<usize>().unwrap())
    {
        *stones_frequency.entry(stone_value).or_default() += 1;
    }

    // println!("start: {:?}", stones_frequency);

    for _i in 0..25 {
        stones_frequency = blink(stones_frequency);
        // println!("{_i}: {:?}", stones_frequency);
    }

    let p1 = stones_frequency.values().sum();

    for _i in 0..50 {
        stones_frequency = blink(stones_frequency);
    }

    let p2 = stones_frequency.values().sum();

    (p1, p2)
}

fn blink(stones: HashMap<usize, usize>) -> HashMap<usize, usize> {
    // Honestly it's probably just easier to always make a new HashMap, so that we don't have to
    // worry with mid-loop changing values and stuff.

    let mut new_stones: HashMap<usize, usize> = HashMap::with_capacity(stones.len());

    for (value, count) in stones {
        // Rule 1: 0->1
        if value == 0 {
            *new_stones.entry(1).or_default() += count;
            continue;
        }

        // Rule 2: even digits = split off
        // This means that we will have count stones with each of the values
        if let Some((stone1_value, stone2_value)) = split_if_even_digits(value) {
            *new_stones.entry(stone1_value).or_default() += count;
            *new_stones.entry(stone2_value).or_default() += count;

            continue;
        }

        // Rule 3: value*2024
        *new_stones.entry(value * 2024).or_default() += count;
    }

    new_stones
}

fn split_if_even_digits(n: usize) -> Option<(usize, usize)> {
    // Stolen from day 7
    let n_digits = n.checked_ilog10().unwrap_or(0) + 1;

    if n_digits % 2 != 0 {
        return None;
    }

    let half_n_digits = n_digits / 2;
    let digit_mask = 10usize.pow(half_n_digits);

    Some((n / digit_mask, n % digit_mask))
}

#[test]
fn example_input() {
    let input = "125 17".to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 55312);
    // assert_eq!(res.1, 81);
}

#[test]
fn prod_solution() {
    use std::fs::read_to_string;

    let input = read_to_string(format!("inputs/{}", "11.in")).unwrap();
    let res = solve_day(input);
    assert_eq!(res.0, 199986);
    assert_eq!(res.1, 42);
}

aoc2024::day_main!("11.in");
