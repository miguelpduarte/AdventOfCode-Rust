#![feature(test)]

extern crate test;

// We don't even need to have a different heuristic, it's just math,
// and finding the function's zeros.
// The distance that the toy boat goes is defined by a quadratic function,
// and we can calculate it in function of the time we hold the button.

// According to `cargo bench`, improved from ~8ms to ~450ns. Math is good.

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

        let (min_time, max_time) = calc_zeros(time_limit, min_distance);
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
    let (min_time, max_time) = calc_zeros(big_time, big_dist);
    // Anything in between will be a possible way to win
    let n_ways_to_win = max_time - min_time + 1;

    let p2 = n_ways_to_win;

    (p1, p2)
}

fn calc_zeros(time_limit: usize, min_distance: usize) -> (usize, usize) {
    // Small hack to handle cases in which the solution would match the distance exactly, as we want to beat it and not be equal to it
    let min_distance = min_distance as f64 + 0.001;

    // The distance is given by a quadratic function in the shape of
    // dist = time_held * (time_limit - time_held)
    // Which is equivalent to dist = -time_held^2 + time_held * time_limit
    // We need to find where this function intersects with dist = min_distance,
    // which is the same as finding the zeros for the function shifted by min_distance:
    // y = -x^2 + x*time_limit - min_distance
    // Where y=dist, x=time_held

    // The zeros of a quadratic function (in the statndard form) can be calculated by:
    // x = (-b +- sqrt(b^2 - 4ac))/2a
    // As per the standard form (y=ax^2 + bx + c): a=-1, b=time_limit, c=-min_distance

    // Therefore:
    let inner_part = f64::sqrt((time_limit.pow(2) as f64 - 4.0 * min_distance) as f64);
    // This must be true for there to be 2 real roots.
    debug_assert!(inner_part > 0.0);
    let min_held_time = (time_limit as f64 - inner_part) / 2.0;
    let max_held_time = (time_limit as f64 + inner_part) / 2.0;

    // Because we can only hold the button for units of time
    // The minimum must be rounded up, as it's a lower bound
    // The maximum must be rounded down, as it's an upper bound.
    (
        min_held_time.ceil() as usize,
        max_held_time.floor() as usize,
    )
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
