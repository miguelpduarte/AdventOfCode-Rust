#![feature(test)]
extern crate test;

fn solve_day(input: String) -> (usize, usize) {
    let p1 = input
        .lines()
        .map(|line| {
            match (line.find(char::is_numeric), line.rfind(char::is_numeric)) {
                (Some(first), Some(second)) => {
                    // No need to consider first and last digit being the same one, as that counts for the problem statement as 2 different ones.
                    let first: usize = (line.as_bytes()[first] - b'0').into();
                    let second: usize = (line.as_bytes()[second] - b'0').into();
                    first * 10 + second
                }
                (None, None) => 0,
                _ => 0,
            }
        })
        .sum();

    // For this version we try replacing the strings with the values to search for the digits all in one go
    // It seems to be very similar to the first solution in 01.rs.
    let p2 = input
        .lines()
        .map(|line| {
            println!("line pre: {}", line);
            // hacky but the example has an eightwo which is meant to be 82, so we double start and end to avoid consuming characters
            // It's probably only a problem based on precedence, but this is an easier approach that can be tuned later.
            let line = line.replace("one", "o1e");
            let line = line.replace("two", "t2");
            let line = line.replace("three", "t3e");
            let line = line.replace("four", "4");
            let line = line.replace("five", "5e");
            let line = line.replace("six", "6");
            let line = line.replace("seven", "7n");
            let line = line.replace("eight", "e8");
            let line = line.replace("nine", "9");
            println!("line post: {}", line);

            match (line.find(char::is_numeric), line.rfind(char::is_numeric)) {
                (Some(first), Some(second)) => {
                    // No need to consider first and last digit being the same one, as that counts for the problem statement as 2 different ones.
                    let first: usize = (line.as_bytes()[first] - b'0').into();
                    let second: usize = (line.as_bytes()[second] - b'0').into();
                    println!("1:{},2:{},val:{}", first, second, first * 10 + second);
                    first * 10 + second
                }
                (None, None) => 0,
                _ => 0,
            }
        })
        .sum();

    (p1, p2)
}

#[test]
fn example_input() {
    let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
        .to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 142);
    assert_eq!(res.1, 142);
}

#[test]
fn example2_input() {
    let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
        .to_owned();
    let res = solve_day(input);
    // assert_eq!(res.0, 142);
    assert_eq!(res.1, 281);
}

#[test]
fn prod_solution() {
    use std::fs::read_to_string;

    let input = read_to_string(format!("inputs/{}", "1.in")).unwrap();
    let res = solve_day(input);
    assert_eq!(res.0, 53194);
    assert_eq!(res.1, 54249);
}

aoc2023::day_main!("1.in");
