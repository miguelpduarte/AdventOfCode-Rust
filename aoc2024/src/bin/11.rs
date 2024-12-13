#![feature(test)]

use std::collections::HashMap;
extern crate test;

// TODO: For some reason, this is still 4ms. Might have been due to the other janky solution being
// running at the time, but it might need some shaving off.
// Likely would be doable by reusing the same map rather than basically cloning it every time. But
// then we need to iterate over the map while changing it at the same time, which I'm not sure is
// really possible... And if we clone it to then modify it based on the "view-only" copy, it's
// basically the same as creating a new map every time!
//
// I guess maybe we can get away with just cloning a list of keys and then accessing those and
// changing them? But that should not be so different...

fn solve_day(input: String) -> (usize, usize) {
    // Maps <stone_value, stone_count>
    let mut stones_frequency: HashMap<usize, usize> = HashMap::new();

    for stone_value in input
        .trim_end()
        .split(' ')
        .map(|item| item.parse::<usize>().unwrap())
    {
        *stones_frequency.entry(stone_value).or_default() += 1;
    }

    // println!("start: {:?}", stones_frequency);

    for _i in 0..25 {
        blink(&mut stones_frequency);
        // println!("{_i}: {:?}", stones_frequency);
    }

    let p1 = stones_frequency.values().sum();

    for _i in 0..50 {
        blink(&mut stones_frequency);
    }

    let p2 = stones_frequency.values().sum();

    (p1, p2)
}

fn blink(stones: &mut HashMap<usize, usize>) {
    // BUG: Since we are not preserving the stones order and making the change in-place, we are
    // working on intermediate updated values, so we might transform more stones than we actually
    // should, because they should not have had that value yet.
    //
    // Ideas for solutions:
    // 1. Use two maps and just switch back and forth. This reduces allocations I guess but ends up
    //    being similar to the original solution. Here we would just be .clear()ing the output map
    //    each time and swapping them around I guess.
    // 2. Output all the changes into a list, and only "commit" them later, all in one go.
    //
    // Trying option 2 as the more straightforward and honestly maybe more likely to be effective.

    let existing_stone_values: Vec<usize> = stones.keys().cloned().collect();
    let mut changed_stones: Vec<(usize, usize)> = Vec::with_capacity(stones.len());

    for value in existing_stone_values {
        // Almost like a .pop for a queue, so that we don't need to reset it later and minimize
        // used space.
        let count = stones.remove(&value).unwrap();

        // Rule 1: 0->1
        if value == 0 {
            // *stones.entry(1).or_default() += count;
            changed_stones.push((1, count));
            continue;
        }

        // Rule 2: even digits = split off
        // This means that we will have count stones with each of the values
        if let Some((stone1_value, stone2_value)) = split_if_even_digits(value) {
            // *stones.entry(stone1_value).or_default() += count;
            changed_stones.push((stone1_value, count));
            // *stones.entry(stone2_value).or_default() += count;
            changed_stones.push((stone2_value, count));
            continue;
        }

        // Rule 3: value*2024
        // *stones.entry(value * 2024).or_default() += count;
        changed_stones.push((value * 2024, count));
    }

    // Update all values at once, after we compute the changes
    // This avoids working on dirty state and accidentally moving around wrong counts since more
    // stones changed to a value mid-iteration
    for (value, count) in changed_stones {
        *stones.entry(value).or_default() += count;
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
    assert_eq!(res.1, 236804088748754);
}

aoc2024::day_main!("11.in");
