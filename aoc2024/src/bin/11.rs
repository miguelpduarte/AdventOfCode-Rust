#![feature(test)]
extern crate test;

fn solve_day(input: String) -> (usize, usize) {
    let mut stones: Vec<usize> = input
        .trim_end()
        .split(' ')
        .map(|item| item.parse::<usize>().unwrap())
        .collect();

    // println!("start: {:?}", stones);

    for _i in 0..25 {
        blink(&mut stones);
        // println!("{_i}: {:?}", stones);
    }

    let p1 = stones.len();

    // Here is for sure where it gets crazy - no example answer as well, but I guess all rule
    // edge-cases are already checked before
    for _i in 0..50 {
        blink(&mut stones);
    }

    let p2 = stones.len();

    (p1, p2)
}

fn blink(stones: &mut Vec<usize>) {
    let mut i = 0;
    // We need a while with a manual i since we are doing some insertions...
    while i < stones.len() {
        let stone = stones[i];
        // Rule 1: 0->1
        if stone == 0 {
            stones[i] = 1;
            i += 1;
            continue;
        }

        // Rule 2: even digits = split off
        if let Some((stone1, stone2)) = split_if_even_digits(stone) {
            stones[i] = stone1;
            stones.insert(i + 1, stone2);
            // Since insertion shifted indexes, we move past the two new stones.
            i += 2;
            continue;
        }

        // Rule 3: value*2024
        stones[i] *= 2024;
        i += 1;
    }
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
