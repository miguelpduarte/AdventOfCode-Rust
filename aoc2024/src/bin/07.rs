#![feature(test)]
extern crate test;

fn solve_day(input: String) -> (usize, usize) {
    let p1 = input
        .lines()
        .map(|line| {
            let (value, nums) = line.split_once(": ").unwrap();
            let value = value.parse().unwrap();
            let nums = nums
                .split(' ')
                .map(|item| item.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            Equation {
                value,
                numbers: nums,
            }
        })
        .filter(is_equation_possible)
        .map(|eq| eq.value)
        .sum();

    let p2 = 0;

    (p1, p2)
}

struct Equation {
    value: usize,
    numbers: Vec<usize>,
}

fn is_equation_possible(eq: &Equation) -> bool {
    let mut reversed_nums = eq.numbers.clone();
    reversed_nums.reverse();

    let mut target = eq.value;

    // Greedy approach, trying to reach as close as possible, as fast as possible:
    // - If number is not divisible we can only use addition
    // - Target is to use addition (subtraction) and get target to 0
    for num in reversed_nums {
        if target == 0 {
            // We have overshot. This might be an issue with our algo being greedy though.
            // So we are printing for debugging...
            println!("overshot");
            return false;
        }
        if target == num {
            // This means that by subtracting we have reached our goal!
            // However, we do not know if this is the last item. So we just set target to 0
            target = 0;
            continue;
        }

        // TODO: Handle overflow (?)

        if target.rem_euclid(num) == 0 {
            // Target is currently divisible by our number.
            // Since we are being greedy, we take this as the "fastest approach" operation.
            target /= num;
        } else {
            // If we can't divide cleanly, this means that this number should just be added (so
            // subtracted since we are going in reverse)
            // Check for overflow, because this also means we have overshot and thus this current
            // solution will not work.
            if let Some(new_target) = target.checked_sub(num) {
                target = new_target;
            } else {
                // Overflow, we overshot
                println!("overshot overflow");
                return false;
            }
        }
    }

    // If we have reached the number, the eq is possible, so check if target is 0 (means exact
    // value was reached)
    target == 0
}

#[test]
fn example_input() {
    let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
        .to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 3749);
    // assert_eq!(res.1, 4);
}

#[test]
fn prod_solution() {
    use std::fs::read_to_string;

    let input = read_to_string(format!("inputs/{}", "7.in")).unwrap();
    let res = solve_day(input);
    // 4701839220390 is too low
    // This means that our greedy approach is failing some eqs that should otherwise be possible...
    assert_eq!(res.0, 42);
    assert_eq!(res.1, 42);
}

aoc2024::day_main!("7.in");
