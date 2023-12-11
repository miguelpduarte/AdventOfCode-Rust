#![feature(test)]

use std::collections::{HashSet, VecDeque};

extern crate test;

#[derive(Debug, Clone)]
struct Coords {
    x: usize,
    y: usize,
    p2_x: usize,
    p2_y: usize,
}

fn solve_day(input: String) -> (usize, usize) {
    let galaxy_size = input.lines().next().unwrap().as_bytes().len();
    // Go over the galaxies, collecting their coordinates into a vec.
    let mut galaxy_coords: VecDeque<Coords> = VecDeque::new();
    // Also collect the x and ys into 2 different hashsets so that the "inversion" and expansion can be done
    let mut galaxy_xs = HashSet::new();
    let mut galaxy_ys = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxy_coords.push_back(Coords {
                    x,
                    y,
                    p2_x: x,
                    p2_y: y,
                });
                galaxy_xs.insert(x);
                galaxy_ys.insert(y);
            }
        }
    }

    // println!("Before exp: {:?}", galaxy_coords);

    // For some reason, we _must_ subtract 1 from the expected factor.
    // Just accepting it for now to get the right result, but it's annoying that I cannot see why right now
    const P2_INCREASE_FACTOR: usize = 1_000_000 - 1;

    // All Xs and Ys not in the sets will be expanded, pushing the coordinates along

    // Expanding X
    for x in (0..galaxy_size).rev() {
        // Searching in reverse so we don't need to "repush"
        if galaxy_xs.contains(&x) {
            // Some galaxy is here, thus no expansion happens
            continue;
        }

        // Push all galaxies with an X larger than this by 1
        for galaxy_coord in galaxy_coords.iter_mut() {
            if galaxy_coord.x > x {
                galaxy_coord.p2_x += P2_INCREASE_FACTOR;
                galaxy_coord.x += 1;
            }
        }
    }

    // Expanding Y
    for y in (0..galaxy_size).rev() {
        // Searching in reverse so we don't need to "repush"
        if galaxy_ys.contains(&y) {
            // Some galaxy is here, thus no expansion happens
            continue;
        }

        // Push all galaxies with an Y larger than this by 1
        for galaxy_coord in galaxy_coords.iter_mut() {
            if galaxy_coord.y > y {
                galaxy_coord.p2_y += P2_INCREASE_FACTOR;
                galaxy_coord.y += 1;
            }
        }
    }

    // println!("{:?}", galaxy_coords);

    let mut path_sum = 0;
    let mut path_sum_p2 = 0;

    while let Some(galaxy_coord) = galaxy_coords.pop_front() {
        let my_dists = galaxy_coords
            .iter()
            .map(|other_galaxy_coords| coord_distance(&galaxy_coord, other_galaxy_coords))
            .sum::<usize>();

        path_sum += my_dists;

        let my_dists_p2 = galaxy_coords
            .iter()
            .map(|other_galaxy_coords| coord_distance_p2(&galaxy_coord, other_galaxy_coords))
            .sum::<usize>();

        path_sum_p2 += my_dists_p2;
    }

    let p1 = path_sum;

    let p2 = path_sum_p2;

    (p1, p2)
}

fn coord_distance(a: &Coords, b: &Coords) -> usize {
    a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
}

fn coord_distance_p2(a: &Coords, b: &Coords) -> usize {
    a.p2_x.abs_diff(b.p2_x) + a.p2_y.abs_diff(b.p2_y)
}

#[test]
fn example_test() {
    let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
        .to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 374);
    // // Ratio: 10
    // assert_eq!(res.1, 1030);
    // // Ratio: 100
    // assert_eq!(res.1, 8410);
    // Ratio: 1000000
    assert_eq!(res.1, 82000210);
}

#[test]
fn prod_solution() {
    use std::fs::read_to_string;

    let input = read_to_string(format!("inputs/{}", "11.in")).unwrap();
    let res = solve_day(input);
    assert_eq!(res.0, 10077850);
    assert_eq!(res.1, 504715068438);
}

aoc2023::day_main!("11.in");
