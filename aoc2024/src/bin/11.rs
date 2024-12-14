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

    // Try to cache stone transition calculation to see if this improves runtime.
    // We can start by caching the first rule, as it is sure to appear.
    let mut transition_cache: HashMap<usize, (usize, Option<usize>)> =
        HashMap::from([(0, (1, None))]);

    for _i in 0..25 {
        stones_frequency = blink(stones_frequency, &mut transition_cache);
        // println!("{_i}: {:?}", stones_frequency);
    }

    let p1 = stones_frequency.values().sum();

    for _i in 0..50 {
        stones_frequency = blink(stones_frequency, &mut transition_cache);
    }

    let p2 = stones_frequency.values().sum();

    (p1, p2)
}

fn blink(
    stones: HashMap<usize, usize>,
    transition_cache: &mut HashMap<usize, (usize, Option<usize>)>,
) -> HashMap<usize, usize> {
    // Creating a new map each time is actually roughly the same or very slightly more efficient
    // than swapping back and forth between two maps, clearing them between iterations.
    // So, just revert back to that simpler approach and ignore the big blocks of commnted text.
    //
    // Interesting observation: Using ::with_capacity here saves us some time, total runtime drops
    // from 6ms using ::new, to 4ms using ::with_capacity. My guess is that the consumed HashMap is
    // freed and then reused in the allocation for the next one, as it retains a similar size,
    // which might justify why it's as efficient as swapping two hashmaps around.
    let mut new_stones = HashMap::with_capacity(stones.len());

    //// BUG: Since we are not preserving the stones order and making the change in-place, we are
    //// working on intermediate updated values, so we might transform more stones than we actually
    //// should, because they should not have had that value yet.
    ////
    //// Ideas for solutions:
    //// 1. Use two maps and just switch back and forth. This reduces allocations I guess but ends up
    ////    being similar to the original solution. Here we would just be .clear()ing the output map
    ////    each time and swapping them around I guess.
    //// 2. Output all the changes into a list, and only "commit" them later, all in one go.
    ////
    //// Option 2 made things slower, so trying out option 1.
    //// Conclusion: roughly same runtime as original solution that always created a new hashmap from
    //// the previous one. Marginally slower even, but in a very low scale so it could just be a
    //// measurement error. Rust is very efficient at optimizing this, it seems.
    //// So, NOTE: This is not the fastest solution out of the ones I've implemented, ironically.
    //// The first non-naive solution is.

    //// We clear the map: keeps allocated memory, but not the intermediate values which might have
    //// been wrong.
    //// Update: Tried just setting the values and never clearing the map, and the result was wrong.
    //// So experimentation seems to show that that optimization is not possible :)
    //output.clear();

    for (value, count) in stones {
        // Check for cached value
        if let Some((stone1, stone2)) = transition_cache.get(&value) {
            // Cached value, just apply transformation and exit
            *new_stones.entry(*stone1).or_default() += count;
            if let Some(stone2) = stone2 {
                *new_stones.entry(*stone2).or_default() += count;
            }

            continue;
        }

        // Rule 1: 0->1
        // With the new transition_cache this should always be cached :)
        if value == 0 {
            *new_stones.entry(1).or_default() += count;
            continue;
        }

        // Rule 2: even digits = split off
        // This means that we will have count stones with each of the values
        if let Some((stone1_value, stone2_value)) = split_if_even_digits(value) {
            *new_stones.entry(stone1_value).or_default() += count;
            *new_stones.entry(stone2_value).or_default() += count;
            transition_cache.insert(value, (stone1_value, Some(stone2_value)));
            continue;
        }

        // Rule 3: value*2024
        let new_value = value * 2024;
        *new_stones.entry(new_value).or_default() += count;
        transition_cache.insert(value, (new_value, None));
    }

    new_stones
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
