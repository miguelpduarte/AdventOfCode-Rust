#![feature(test)]

extern crate test;

// TODO: Optimize, it's taking ~8ms atm.
// We can probably have a better heuristic of finding min and
// max that is not just iterating over the range

fn solve_day(input: String) -> (usize, usize) {
    let mut lines = input.lines();

    let (_time_label, times) = lines.next().unwrap().split_once(':').unwrap();
    let times = times
        .split_ascii_whitespace()
        .map(|time| time.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let (_distance_label, distances) = lines.next().unwrap().split_once(':').unwrap();
    let distances = distances
        .split_ascii_whitespace()
        .map(|time| time.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut record_product = 1;
    let n_races = times.len();

    for i in 0..n_races {
        let time_limit = times[i];
        let min_distance = distances[i];

        // Calculating the minimum time we need to hold to beat the min distance
        let mut possible_time_held = 1..time_limit;
        let min_time = possible_time_held
            .find(|time_held| calc_distance(*time_held, time_limit) > min_distance)
            .unwrap();

        // Calculating the max time we can hold and still beat the min distance
        let possible_time_held = 1..time_limit;
        let max_time = possible_time_held
            .rev()
            .find(|time_held| calc_distance(*time_held, time_limit) > min_distance)
            .unwrap();

        // Anything in between will be a possible way to win
        let n_ways_to_win = max_time - min_time + 1;
        // println!("{}: {}-{}", n_ways_to_win, min_time, max_time);
        record_product *= n_ways_to_win;
    }

    let p1 = record_product;

    // TODO: Find a better way to convert lists of numbers into a single number, this is very hacky
    let big_time = times
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("")
        .parse::<usize>()
        .unwrap();
    let big_dist = distances
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("")
        .parse::<usize>()
        .unwrap();

    // Calculating the minimum time we need to hold to beat the min distance
    let mut possible_time_held = 1..big_time;
    let min_time = possible_time_held
        .find(|time_held| calc_distance(*time_held, big_time) > big_dist)
        .unwrap();

    // Calculating the max time we can hold and still beat the min distance
    let possible_time_held = 1..big_time;
    let max_time = possible_time_held
        .rev()
        .find(|time_held| calc_distance(*time_held, big_time) > big_dist)
        .unwrap();

    // Anything in between will be a possible way to win
    let n_ways_to_win = max_time - min_time + 1;

    let p2 = n_ways_to_win;

    (p1, p2)
}

fn calc_distance(time_held: usize, time_limit: usize) -> usize {
    time_held * (time_limit - time_held)
}

#[test]
fn example_input() {
    let input = "Time:      7  15   30
Distance:  9  40  200"
        .to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 288);
    assert_eq!(res.1, 71503);
}

#[test]
fn prod_solution() {
    use std::fs::read_to_string;

    let input = read_to_string(format!("inputs/{}", "6.in")).unwrap();
    let res = solve_day(input);
    assert_eq!(res.0, 2612736);
    assert_eq!(res.1, 29891250);
}

aoc2023::day_main!("6.in");
