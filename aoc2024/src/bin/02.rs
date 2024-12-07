#![feature(test)]
extern crate test;

fn solve_day(input: String) -> (usize, usize) {
    let p1 = input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|item| item.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(is_safe)
        .filter(|&x| x)
        .count();

    let p2 = input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|item| item.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(is_safe_with_dampener)
        .filter(|&x| x)
        .count();

    (p1, p2)
}

// Just bruteforcing it for now tbh...
fn is_safe_with_dampener(levels: Vec<usize>) -> bool {
    if is_safe(levels.clone()) {
        return true;
    }

    // Start the bruteforcing
    let len = levels.len();
    for i in 0..len {
        let mut dampened = levels.clone();
        dampened.remove(i);
        if is_safe(dampened) {
            return true;
        }
    }

    false
}

enum Report {
    Unstarted,
    Initial(usize),
    Increasing(usize),
    Decreasing(usize),
}

fn is_safe(levels: Vec<usize>) -> bool {
    let mut report_status = Report::Unstarted;
    for num in levels {
        match report_status {
            Report::Unstarted => report_status = Report::Initial(num),
            Report::Initial(prev) => {
                let abs_dif = num.abs_diff(prev);
                if abs_dif > 3 || abs_dif == 0 {
                    return false;
                }
                if num > prev {
                    report_status = Report::Increasing(num);
                } else {
                    report_status = Report::Decreasing(num);
                }
            }
            Report::Increasing(prev) => {
                if num <= prev || num > prev + 3 {
                    return false;
                }
                report_status = Report::Increasing(num);
            }
            Report::Decreasing(prev) => {
                // prev - num > 3 // Decrease is up to 3, this is the failure condition
                // prev > num + 3
                if num >= prev || num + 3 < prev {
                    return false;
                }
                report_status = Report::Decreasing(num);
            }
        };
    }

    true
}

#[test]
fn example_input() {
    let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
        .to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 2);
    assert_eq!(res.1, 4);
}

#[test]
fn prod_solution() {
    use std::fs::read_to_string;

    let input = read_to_string(format!("inputs/{}", "2.in")).unwrap();
    let res = solve_day(input);
    assert_eq!(res.0, 421);
    assert_eq!(res.1, 476);
}

aoc2024::day_main!("2.in");
