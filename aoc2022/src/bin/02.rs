#![feature(test)]
extern crate test;

fn match_score(opponent: u8, me: u8) -> u8 {
    let opp_idx = opponent - b'A';
    let me_idx = me - b'X';

    assert!(opp_idx <= 2);
    assert!(me_idx <= 2);

    /*
    tie   win   loss
    0 0
          0 1
                 0 2
                 1 0
    1 1
          1 2
          2 0
                 2 1
    2 2
    */

    let outcome_score: u8 = match (opp_idx - me_idx) as i8 {
        0 => 3,              // Same choice, tie
        -1 | 2 => 6,         // I chose the next one in the circle, I win
        -2 | 1 => 0,         // I lose, opponent chose the best option
        _ => unreachable!(), // should be covered by the asssert above :thinking:
    };

    outcome_score + (me_idx + 1)
}

fn match_score_pt2(opponent: u8, me: u8) -> u8 {
    let opp_idx = opponent - b'A';
    let desired_outcome_idx = me - b'X'; // 0 = lose, 1 = tie, 2 = win

    assert!(opp_idx <= 2);
    assert!(desired_outcome_idx <= 2);

    /*
    tie   win   loss
    0 0
          0 1
                 0 2
                 1 0
    1 1
          1 2
          2 0
                 2 1
    2 2
    */

    let outcome_score = desired_outcome_idx * 3;

    let my_choice = match (opp_idx, desired_outcome_idx) {
        // I want to lose
        // Rock, pick scissors
        (0, 0) => 2,
        // Paper, pick rock
        (1, 0) => 0,
        // Scissors, pick paper
        (2, 0) => 1,
        // I want to tie
        (opp_choice, 1) => opp_choice,
        // I want to win
        // Rock, pick paper
        (0, 2) => 1,
        // Paper, pick scissors
        (1, 2) => 2,
        // Scissors, pick rock
        (2, 2) => 0,
        _ => unreachable!(),
    };

    outcome_score + (my_choice + 1)
}

fn solve_day(input: String) -> (usize, usize) {
    let (p1, p2) = input
        .lines()
        .fold((0usize, 0usize), |(sum_pt1, sum_pt2), l| {
            let line_bytes = l.as_bytes();
            (
                sum_pt1 + match_score(line_bytes[0], line_bytes[2]) as usize,
                sum_pt2 + match_score_pt2(line_bytes[0], line_bytes[2]) as usize,
            )
        });

    (p1, p2)
}

#[test]
fn example_input() {
    let input = "A Y
B X
C Z"
    .to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 15);
    assert_eq!(res.1, 12);
}

#[test]
fn prod_solution() {
    use std::fs::read_to_string;

    let input = read_to_string(format!("inputs/{}", "2.in")).unwrap();
    let res = solve_day(input);
    assert_eq!(res.0, 13484);
    assert_eq!(res.1, 13433);
}

aoc2022::day_main!("2.in");
