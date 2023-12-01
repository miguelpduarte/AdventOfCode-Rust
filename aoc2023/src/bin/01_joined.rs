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

    // Apparently we don't handle zero here but we also don't need to.
    const NUMBER_STRS: [&str; 18] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", //
        "1", "2", "3", "4", "5", "6", "7", "8", "9",
    ];

    // The logic in the 01_joined version is cleaner by joining the digit with the "string number" checking
    // However, it is a bit slower, as benchmark shows that it is consistently above 1ms, and the original solution is consistently under that.
    let p2 = input
        .lines()
        .map(|line| {
            let first_num_txt = NUMBER_STRS
                .iter()
                .enumerate()
                .filter_map(|(i, &num_str)| Some((line.find(num_str)?, (i % 9) + 1)))
                .min_by_key(|(index, _num)| *index);
            let first = first_num_txt.unwrap().1;

            let last_num_txt = NUMBER_STRS
                .iter()
                .enumerate()
                .filter_map(|(i, &num_str)| Some((line.rfind(num_str)?, (i % 9) + 1)))
                .max_by_key(|(index, _num_str)| *index);
            let second = last_num_txt.unwrap().1;

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
