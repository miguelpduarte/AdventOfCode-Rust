#![feature(test)]

use std::collections::HashSet;
extern crate test;

fn solve_day(input: String) -> (usize, usize) {
    let mut p1 = 0;
    let window_size = 4;
    for (i, window) in input.trim_end().as_bytes().windows(window_size).enumerate() {
        let a = window[0];
        let b = window[1];
        let c = window[2];
        let d = window[3];
        // println!("{:?}", window);

        if a == b || a == c || a == d || b == c || b == d || c == d {
            continue;
        }

        // println!("found: {:?} {}", String::from_utf8_lossy(window), i);

        // The expected value is "when the character arrives" rather than the index from which the 4-char sequence starts.
        // Also +1 because we start indexes at 0
        p1 = i + window_size;
        break;
    }

    let mut p2 = 0;
    let window_size = 14;
    for (i, window) in input.trim_end().as_bytes().windows(window_size).enumerate() {
        let set: HashSet<&u8> = HashSet::from_iter(window);
        if set.len() == window_size {
            p2 = i + window_size;
            break;
        }
    }

    (p1, p2)
}

#[test]
fn example0() {
    let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 7);
    assert_eq!(res.1, 19);
}

#[test]
fn example1() {
    let input = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 5);
    assert_eq!(res.1, 23);
}

#[test]
fn example2() {
    let input = "nppdvjthqldpwncqszvftbrmjlhg".to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 6);
    assert_eq!(res.1, 23);
}

#[test]
fn example3() {
    let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 10);
    assert_eq!(res.1, 29);
}

#[test]
fn example4() {
    let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 11);
    assert_eq!(res.1, 26);
}

#[test]
fn prod_solution() {
    use std::fs::read_to_string;

    let input = read_to_string(format!("inputs/{}", "6.in")).unwrap();
    let res = solve_day(input);
    assert_eq!(res.0, 1140);
    assert_eq!(res.1, 3495);
}

aoc2022::day_main!("6.in");
