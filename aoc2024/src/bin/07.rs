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
    // Going over the numbers in reverse because then we can know if multiplication was used,
    // by checking if the current number is a divisor of the "accumulator".
    let mut reversed_nums = eq.numbers.clone();
    reversed_nums.reverse();

    // TODO: I guess there could be an optimization where we would be swapping around the lists and
    // clearing them, just to keep the allocated memory.
    // Looking at the input, the equations are not that long so this should be ok for now.
    let mut targets = vec![eq.value];

    // Dynamic Programming (I think) approach:
    // If we cannot divide, subtracting is the only option - already done before in greedy
    // If we can divide, we can also subtract. To not overshoot, we need to consider both options.
    // As such, when we have that case, we add 2 options: one if we add/subtract, another one for
    // div/mult.
    // Whenever we hit an underflow (subtracting would put us over target), we know that this
    // option is not possible, so we can remove this item from the targets list.
    //
    // Then, at the end, we just need to have at least one item in targets to have reached 0.

    for num in reversed_nums {
        // The vector is likely to have a similar size, might be smaller due to trimming impossible
        // paths, or bigger if there's a lot of branching, but this should be a decent baseline
        // case for now.
        let mut new_targets = Vec::with_capacity(targets.len());

        for target in targets {
            if target == 0 {
                // We already reached 0 but we still have `num`s to go.
                // This means we overshot, so this path is impossible.
                continue;
            }

            if num > target {
                // We overshot, so this path is impossible, don't add to new_targets.
                // Not counting == as that would just mean a full match which might be ok if this
                // is the last element.
                continue;
            }

            // We can always subtract
            // Since we checked for over/underflow cases above, this is safe.
            new_targets.push(target - num);

            // We can only divide if there is no remainder
            // (as otherwise we immediately know that this path was not taken)
            if target.rem_euclid(num) == 0 {
                new_targets.push(target / num);
            }
        }

        targets = new_targets;
    }

    // As said above, if any path reached 0, the eq is possible:
    targets.iter().any(|&x| x == 0)
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
    // 4701839220390 is too low - greedy option
    // This means that our greedy approach is failing some eqs that should otherwise be possible...
    // Success on the second impl!
    assert_eq!(res.0, 4998764814652);
    assert_eq!(res.1, 42);
}

aoc2024::day_main!("7.in");
