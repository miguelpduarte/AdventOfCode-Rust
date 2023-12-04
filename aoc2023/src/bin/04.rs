#![feature(test)]

use std::collections::{HashMap, HashSet};

extern crate test;

// By the way, note to self: clippy's lint for single_char_pattern improved time from ~93us to ~48us
// See https://rust-lang.github.io/rust-clippy/master/index.html#/single_char_pattern

fn solve_day(input: String) -> (usize, usize) {
    let p1 = input
        .lines()
        .map(|line| {
            let matches = line_to_card_matches(line);

            if matches == 0 {
                0
            } else {
                2_usize.pow((matches - 1).try_into().unwrap())
            }
        })
        .sum();

    let mut n_cards = 0;
    let mut card_copies: HashMap<usize, usize> = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        let card_id = i + 1;

        let matches = line_to_card_matches(line);
        // Multiply matches by how many copies of this card we have
        // (Performance note: I tested both using .get and .remove and it seems that .remove was either the same or slower, consistently)
        // (I'm guessing that the cost of adding the remove operation is higher than just growing the map (it's roughly 200 entries of usizes, so not too much I guess))
        let copies = *card_copies.get(&card_id).unwrap_or(&0);
        // Count how many cards we have, which is 1 for the original of this card+however many copies we accumulated of it
        n_cards += 1 + copies;

        for won_relative_idx in 1..=matches {
            let won_id = card_id + won_relative_idx;
            // We always win 1 copy at minimum, and additionally however many copies this card has
            let won_copies = 1 + copies;
            // println!("Won {} copies of card {}!", won_copies, won_id);
            card_copies
                .entry(won_id)
                .and_modify(|curr_copies| *curr_copies += won_copies)
                .or_insert(won_copies);
        }
    }

    let p2 = n_cards;

    (p1, p2)
}

fn line_to_card_matches(line: &str) -> usize {
    let (_card_str, card_numbers) = line.split_once(':').unwrap();
    let (winning_numbers, our_numbers) = card_numbers.split_once('|').unwrap();

    let winning_numbers = winning_numbers
        .trim()
        .split_ascii_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect::<HashSet<_>>();

    let our_numbers = our_numbers
        .trim()
        .split_ascii_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect::<HashSet<_>>();

    let matches = our_numbers.intersection(&winning_numbers).count();
    matches
}

#[test]
fn example_input() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
        .to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 13);
    assert_eq!(res.1, 30);
}

#[test]
fn prod_solution() {
    use std::fs::read_to_string;

    let input = read_to_string(format!("inputs/{}", "4.in")).unwrap();
    let res = solve_day(input);
    assert_eq!(res.0, 19135);
    assert_eq!(res.1, 5704953);
}

aoc2023::day_main!("4.in");
