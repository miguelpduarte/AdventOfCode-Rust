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

    const NUMBER_STRS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let p2 = input
        .lines()
        .map(|line| {
            let first_num_txt = NUMBER_STRS
                .iter()
                .enumerate()
                .filter_map(|(i, &num_str)| Some((line.find(num_str)?, i + 1)))
                .min_by_key(|(index, _num)| *index);
            let first_digit = line.find(char::is_numeric);

            // This logic can probably be thrown into the loop above, and search also for the strings of '0'-'9',
            // using *_by_key to do the comparisons. Probably would be able to use the index as (i%10) or similar as well to avoid the parsing.
            let first = match (first_num_txt, first_digit) {
                (Some((_first_num_idx, first_num_val)), None) => first_num_val,
                (None, Some(digit_idx)) => (line.as_bytes()[digit_idx] - b'0').into(),
                (Some((first_num_idx, first_num_val)), Some(digit_idx)) => {
                    if first_num_idx < digit_idx {
                        first_num_val
                    } else {
                        (line.as_bytes()[digit_idx] - b'0').into()
                    }
                }
                (None, None) => panic!(),
            };

            let last_num_txt = NUMBER_STRS
                .iter()
                .enumerate()
                .filter_map(|(i, &num_str)| Some((line.rfind(num_str)?, i + 1)))
                .max_by_key(|(index, _num_str)| *index);
            let last_digit = line.rfind(char::is_numeric);

            let second = match (last_num_txt, last_digit) {
                (Some((_last_num_idx, last_num_val)), None) => last_num_val,
                (None, Some(digit_idx)) => (line.as_bytes()[digit_idx] - b'0').into(),
                (Some((last_num_idx, last_num_val)), Some(digit_idx)) => {
                    if last_num_idx > digit_idx {
                        last_num_val
                    } else {
                        (line.as_bytes()[digit_idx] - b'0').into()
                    }
                }
                (None, None) => panic!(),
            };

            first * 10 + second
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
