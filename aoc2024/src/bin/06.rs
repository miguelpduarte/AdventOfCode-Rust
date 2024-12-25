#![feature(test)]

use std::collections::{BTreeSet, HashMap, HashSet};
extern crate test;

#[derive(Debug)]
enum Heading {
    Up,
    Right,
    Down,
    Left,
}

/// Should probably just be 130, but this way we get a safety margin
#[allow(unused)]
fn coord_to_pair(x: usize, y: usize) -> (usize, usize) {
    (x, y)
}

// TODO: Test making this a power of 2 to see if it's more efficient lol
const WRAP_VALUE: usize = 140;
// TODO: Use this instead for efficiency
fn coord_to_collapsed_int(x: usize, y: usize) -> usize {
    x + y * WRAP_VALUE
}

fn solve_day(input: String) -> (usize, usize) {
    let mut guard_start_pos = None;
    // So we can know when the guard goes off the grid
    let grid_size = input.lines().next().unwrap().len();
    // col means same X, so we get the Ys
    let mut obstacle_by_col: HashMap<usize, BTreeSet<usize>> = HashMap::new();
    // row means same Y, so we get the Xs
    let mut obstacle_by_row: HashMap<usize, BTreeSet<usize>> = HashMap::new();

    for (y, line) in input.trim_end().lines().enumerate() {
        for (x, item) in line.bytes().enumerate() {
            match item {
                // Obstacle
                b'#' => {
                    obstacle_by_col.entry(x).or_default().insert(y);
                    obstacle_by_row.entry(y).or_default().insert(x);
                }
                // Guard start pos
                b'^' => {
                    let _ = guard_start_pos.insert((x, y));
                }
                _ => {}
            }
        }
    }

    // println!("by row: {:?}", obstacle_by_row);
    // println!("by col: {:?}", obstacle_by_col);

    let (mut guard_x, mut guard_y) = guard_start_pos.expect("start position should be set");
    // Guard always starts facing up
    let mut guard_heading = Heading::Up;

    // We can store the visited coords into an HashSet. Since we know that the coords cap at
    // 130x130 for puzzle input (even smaller for example) we can try to represent them as a single
    // int. Trying that out since it's more efficient lol.
    let mut seen_coords = HashSet::new();
    seen_coords.insert(coord_to_collapsed_int(guard_x, guard_y));

    // Move the guard around until they go off map
    loop {
        // TODO: Refactor the loop a bit to be nicer
        // We can probably also simplify the ranges logic to have a "final_x"/"final_y" var that we
        // use instead.
        // println!("{},{} - going {:?}", guard_x, guard_y, guard_heading);
        match guard_heading {
            Heading::Up => {
                // Determine the next obstacle that is found (same column, < y)
                let candidate_obstacle = obstacle_by_col.get(&guard_x).and_then(|obstacles| {
                    obstacles
                        .iter()
                        .filter(|&obstacle_y| obstacle_y < &guard_y)
                        // Take in reverse since the BTreeSet goes in ascending order, and we want the
                        // largest one (closest to the guard that is going upwards)
                        .next_back()
                });

                if let Some(&obstacle_y) = candidate_obstacle {
                    // Travel until the obstacle, then turn right.
                    let traveled_coords =
                        (obstacle_y + 1..=guard_y).map(|y| coord_to_collapsed_int(guard_x, y));
                    seen_coords.extend(traveled_coords);
                    // Guard doesn't step over the obstacle but instead stays in the space before
                    guard_y = obstacle_y + 1;
                    guard_heading = Heading::Right;
                } else {
                    // No obstacle found, let's go off grid and exit
                    // We go from the current y to 0
                    let traveled_coords = (0..=guard_y).map(|y| coord_to_collapsed_int(guard_x, y));
                    seen_coords.extend(traveled_coords);
                    println!("started at {},{}, going off grid", guard_x, guard_y);
                    break;
                }
            }
            Heading::Right => {
                // Determine the next obstacle - same row, > x
                let candidate_obstacle = obstacle_by_row.get(&guard_y).and_then(|obstacles| {
                    obstacles
                        .iter()
                        .filter(|&obstacle_x| obstacle_x > &guard_x)
                        // Take in reverse since the BTreeSet goes in ascending order, and we want the
                        // largest one (closest to the guard that is going to the right)
                        .next_back()
                });

                if let Some(&obstacle_x) = candidate_obstacle {
                    // Travel until the obstacle, then turn down.
                    let traveled_coords =
                        (guard_x..obstacle_x).map(|x| coord_to_collapsed_int(x, guard_y));
                    seen_coords.extend(traveled_coords);
                    // Guard doesn't step over the obstacle but instead stays in the space before
                    guard_x = obstacle_x - 1;
                    guard_heading = Heading::Down;
                } else {
                    // No obstacle found, let's go off grid and exit
                    // We go from the current x to grid size
                    let traveled_coords =
                        (guard_x..=grid_size).map(|x| coord_to_collapsed_int(x, guard_y));
                    seen_coords.extend(traveled_coords);
                    println!("started at {},{}, going off grid", guard_x, guard_y);
                    break;
                }
            }
            Heading::Down => {
                // Determine the next obstacle that is found (same column, > y)
                let candidate_obstacle = obstacle_by_col.get(&guard_x).and_then(|obstacles| {
                    obstacles.iter().find(|&obstacle_y| obstacle_y > &guard_y)
                });

                if let Some(&obstacle_y) = candidate_obstacle {
                    // Travel until the obstacle, then turn left.
                    let traveled_coords =
                        (guard_y..obstacle_y).map(|y| coord_to_collapsed_int(guard_x, y));
                    seen_coords.extend(traveled_coords);
                    // Guard doesn't step over the obstacle but instead stays in the space before
                    guard_y = obstacle_y - 1;
                    guard_heading = Heading::Left;
                } else {
                    // No obstacle found, let's go off grid and exit
                    // We go from the current y to max grid size
                    let traveled_coords =
                        (guard_y..=grid_size).map(|y| coord_to_collapsed_int(guard_x, y));
                    seen_coords.extend(traveled_coords);
                    println!("started at {},{}, going off grid", guard_x, guard_y);
                    break;
                }
            }
            Heading::Left => {
                // Determine the next obstacle - same row, < x
                let candidate_obstacle = obstacle_by_row.get(&guard_y).and_then(|obstacles| {
                    obstacles.iter().find(|&obstacle_x| obstacle_x < &guard_x)
                });

                if let Some(&obstacle_x) = candidate_obstacle {
                    // Travel until the obstacle, then turn upwards.
                    let traveled_coords =
                        (obstacle_x + 1..=guard_x).map(|x| coord_to_collapsed_int(x, guard_y));
                    seen_coords.extend(traveled_coords);
                    // Guard doesn't step over the obstacle but instead stays in the space before
                    guard_x = obstacle_x + 1;
                    guard_heading = Heading::Up;
                } else {
                    // No obstacle found, let's go off grid and exit
                    // We go from the current x to grid size
                    let traveled_coords = (0..=guard_x).map(|x| coord_to_collapsed_int(x, guard_y));
                    seen_coords.extend(traveled_coords);
                    println!("started at {},{}, going off grid", guard_x, guard_y);
                    break;
                }
            }
        }
    }

    // println!("{:?}", seen_coords);

    // let mut m: HashMap<usize, BTreeSet<usize>> = HashMap::new();
    // for item in seen_coords.iter() {
    //     m.entry(item.0).or_default().insert(item.1);
    // }

    // for (x, line) in m {
    //     println!("{} -> {:?}", x, line);
    // }

    // -1 since we are also inserting the off-grid stuff :sweat_smile:
    let p1 = seen_coords.len() - 1;

    let p2 = 0;

    (p1, p2)
}

#[test]
fn example_input() {
    let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
        .to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 41);
    // assert_eq!(res.1, 81);
}

#[test]
fn prod_solution() {
    use std::fs::read_to_string;

    let input = read_to_string(format!("inputs/{}", "6.in")).unwrap();
    let res = solve_day(input);
    assert_eq!(res.0, 42);
    assert_eq!(res.1, 42);
}

aoc2024::day_main!("6.in");
