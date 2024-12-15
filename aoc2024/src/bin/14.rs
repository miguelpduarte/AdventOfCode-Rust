#![feature(test)]

use std::collections::{BTreeSet, HashMap};

extern crate test;

// Using isizes everywhere since velocities can be negative, and mixing signed and unsigned is
// annoying so I'm just using mostly isize.

const GRID_WIDTH: isize = 101;
const GRID_HEIGHT: isize = 103;

const P2_ITERS: usize = 30000;

// Tried OnceLock but apparently static's are shared between tests so we couldn't reuse code for
// example and "prod"
fn solve_day(input: String) -> (usize, usize) {
    // So we pull it into another separate fn
    solve_day_with_gridsize(input, GRID_WIDTH, GRID_HEIGHT)
}

fn solve_day_with_gridsize(input: String, grid_width: isize, grid_height: isize) -> (usize, usize) {
    // Checking all first 100 seconds to get the minimum safety score, praying that it's the xmas
    // tree.
    // quadrants are organized: top left, top right, bottom left, bottom right
    let mut quadrants_by_secs: HashMap<usize, [usize; 4]> = HashMap::with_capacity(100);
    // Cache this division
    let grid_mid_width = grid_width / 2;
    let grid_mid_width = grid_mid_width.try_into().unwrap();
    let grid_mid_height = grid_height / 2;
    let grid_mid_height = grid_mid_height.try_into().unwrap();

    for line in input.trim_end().lines() {
        let (p_str, v_str) = line.split_once(' ').unwrap();
        let p0 = parse_prefixed_coord(p_str);
        let v0 = parse_prefixed_coord(v_str);

        for seconds in 1..=P2_ITERS {
            let final_coord = calc_final_robot_coord(p0, v0, grid_width, grid_height, seconds);
            if let Some(robot_quadrant) =
                calc_quadrant(final_coord, grid_mid_width, grid_mid_height)
            {
                quadrants_by_secs.entry(seconds as usize).or_default()[robot_quadrant] += 1;
            }
        }
    }

    // dbg!(quadrants);

    let p1 = quadrants_to_safety_score(quadrants_by_secs.get(&100).unwrap());

    let secs_by_safety_score = quadrants_by_secs
        .iter()
        .map(|(sec, quadrants)| (sec, quadrants_to_safety_score(quadrants)));
    // Problem: We overwrites the safety score if it's the same due to the key being the same.
    // As such, we don't see the cycles...
    // // E.g. 27558 and 48364 have the same safety score (96433341) but I only saw via experimentation.
    // So, let's use a custom struct so we can use our own Ord implementation and use a BTreeSet.

    #[derive(Eq, PartialEq, Debug)]
    struct Wrapper {
        second: usize,
        safety_score: usize,
    }

    /// This will sort by safety score, and if it's the same, sort by second instead, so we always
    /// get the lowest second.
    impl Ord for Wrapper {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            match self.safety_score.cmp(&other.safety_score) {
                std::cmp::Ordering::Equal => self.second.cmp(&other.second),
                x => x,
            }
        }
    }

    impl PartialOrd for Wrapper {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    let secs_by_safety_score = secs_by_safety_score.map(|(&second, safety_score)| Wrapper {
        second,
        safety_score,
    });

    let sorted_secs_and_safety_score = BTreeSet::from_iter(secs_by_safety_score);

    // let secs_by_safety_score = BTreeMap::from_iter(secs_by_safety_score);

    // This is actually not necessary, just did it for debugging
    // let lowest_safety_scores: Vec<_> = sorted_secs_and_safety_score
    //     .iter()
    //     .take(20)
    //     .inspect(|x| println!("{:?}", x))
    //     .collect();

    // Find second with minimum safety score
    let p2 = sorted_secs_and_safety_score.first().unwrap().second;

    (p1, p2)
}

fn quadrants_to_safety_score(quadrants: &[usize; 4]) -> usize {
    quadrants.iter().product()
}

/// Returns which quadrant the position is in, None if in between quadrants.
/// Used to index into `quadrants`, so return must be 0..=4
///
fn calc_quadrant(
    (x, y): (usize, usize),
    grid_mid_width: usize,
    grid_mid_height: usize,
) -> Option<usize> {
    // dbg!(x, y, grid_mid_width, grid_mid_height);
    if x == grid_mid_width || y == grid_mid_height {
        // In no quadrant
        return None;
    }

    // A bit cursed, but works
    // If on the left of middle, x component will be 0 ; 1 otherwise
    // If below middle, y component will be 1*2=2 ; 0 otherwise
    // With that we get all combinations in "z order", 0,1,2,3.
    let quadrant =
        if x > grid_mid_width { 1 } else { 0 } + 2 * if y > grid_mid_height { 1 } else { 0 };

    Some(quadrant)
}

fn calc_final_robot_coord(
    (p0x, p0y): (isize, isize),
    (v0x, v0y): (isize, isize),
    grid_width: isize,
    grid_height: isize,
    n_iters: isize,
) -> (usize, usize) {
    let x = (p0x + v0x * n_iters).rem_euclid(grid_width);
    let y = (p0y + v0y * n_iters).rem_euclid(grid_height);
    (x.try_into().unwrap(), y.try_into().unwrap())
}

fn parse_prefixed_coord<T>(prefixed_coord: &str) -> (T, T)
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    // Remove p= or v= prefix
    let (_, coords) = prefixed_coord.split_at(2);
    let (x, y) = coords.split_once(',').unwrap();
    (x.parse().unwrap(), y.parse().unwrap())
}

#[test]
fn example_input() {
    let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"
        .to_owned();
    // Using example grid size to get the correct result.
    let res = solve_day_with_gridsize(input, 11, 7);
    assert_eq!(res.0, 12);
    // assert_eq!(res.1, 81);
}

#[test]
fn prod_solution() {
    use std::fs::read_to_string;

    let input = read_to_string(format!("inputs/{}", "14.in")).unwrap();
    // Here we can use the "real" solve_day fn since it defaults to the big grid.
    let res = solve_day(input);
    // 78914880 is too low. I was using the example grid size rather than the real one
    // (11x7 vs 101x103).
    assert_eq!(res.0, 225810288);
    // 86 (min safety score) is not the right answer...
    // 490 is too low. So we probably haven't hit the cycle yet.
    // 27558 is too high apparently.
    // ; Safety score for 48364 is the same, so the cycle might happen earlier, lowering num iters
    // to check.
    // After only storing the lowest second when the score matches, we got it :)
    assert_eq!(res.1, 6752);
}

aoc2024::day_main!("14.in");
