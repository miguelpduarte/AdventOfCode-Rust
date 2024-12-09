#![feature(test)]
extern crate test;

use std::collections::{HashMap, HashSet};

type TowerType = char;

#[derive(Debug, PartialEq)]
struct Tower {
    pub x: usize,
    pub y: usize,
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct AntiNode {
    pub x: usize,
    pub y: usize,
}

fn solve_day(input: String) -> (usize, usize) {
    let mut towers_by_type: HashMap<TowerType, Vec<Tower>> = HashMap::new();
    let mut height = 0;
    // TODO: Maybe those OnceCell can make this not repeat a calculation?
    // Should probably not be a bottleneck though.
    let mut width = 0;

    for (y, line) in input.lines().enumerate() {
        // TODO: We could also iterate and get this rather than using str::len().
        // Not sure if that would necessarily be better, as this might be cached,
        // but since we iterate anyway...
        width = line.len();
        height += 1;

        // Parse towers
        // Assume ASCII chars for speed.
        for (x, item) in line.bytes().enumerate() {
            let item = item as char;
            if item != '.' {
                towers_by_type.entry(item).or_default().push(Tower { x, y });
            }
        }
    }

    // println!("{:?}", towers_by_type);

    // let n_towers: usize = towers_by_type.values().map(|towers| towers.len()).sum();
    // println!("n towers: {}", n_towers);

    let antinodes: HashSet<AntiNode> = calculate_antinodes(&towers_by_type, width, height);

    // let _: Vec<_> = antinodes
    //     .iter()
    //     .inspect(|an| println!("{:?}", an))
    //     .collect();

    let p1 = antinodes.len();

    let harmonic_antinodes: HashSet<AntiNode> =
        calculate_harmonic_antinodes(&towers_by_type, width, height);

    let p2 = harmonic_antinodes.len();

    (p1, p2)
}

fn calculate_harmonic_antinodes(
    towers: &HashMap<char, Vec<Tower>>,
    width: usize,
    height: usize,
) -> HashSet<AntiNode> {
    let mut antinodes = HashSet::new();

    for towers in towers.values() {
        // For each tower, compute all possible antinodes by matching with all towers (other than
        // itself)

        for our_tower in towers {
            for their_tower in towers {
                if our_tower == their_tower {
                    continue;
                }

                let new_antinodes =
                    calculate_harmonic_antinodes_bounded(our_tower, their_tower, width, height);

                antinodes.extend(new_antinodes);
            }
        }
    }

    antinodes
}

fn calculate_antinodes(
    towers: &HashMap<char, Vec<Tower>>,
    width: usize,
    height: usize,
) -> HashSet<AntiNode> {
    let mut antinodes = HashSet::new();

    for towers in towers.values() {
        // For each tower, compute all possible antinodes by matching with all towers (other than
        // itself)

        for our_tower in towers {
            for their_tower in towers {
                if our_tower == their_tower {
                    continue;
                }

                if let Some(antinode) =
                    calculate_antinode_bounded(our_tower, their_tower, width, height, 2)
                {
                    antinodes.insert(antinode);
                }
            }
        }
    }

    antinodes
}

fn calculate_harmonic_antinodes_bounded(
    ours: &Tower,
    theirs: &Tower,
    width: usize,
    height: usize,
) -> Vec<AntiNode> {
    // Loop the "distance_scale", stopping when we find an item out of bounds
    // While something is in-bounds, we keep going.

    let mut antinodes = vec![];

    for distance_scale in 1.. {
        let Some(antinode) =
            calculate_antinode_bounded(ours, theirs, width, height, distance_scale)
        else {
            // OOB, we're done
            break;
        };

        antinodes.push(antinode);
    }

    antinodes
}

fn calculate_antinode_bounded(
    ours: &Tower,
    theirs: &Tower,
    width: usize,
    height: usize,
    distance_scale: usize,
) -> Option<AntiNode> {
    // Calculate the vector between our_tower and their_tower and double it.
    // Always use the same direction (our to their) because we are doing all possible combinations,
    // so we will cover the reverse case already.

    // x = ours.x + dif * 2 <>
    // x = ours.x + (theirs.x - ours.x) * 2
    // <> 2*theirs.x - ours.x (aka theirs.x + dif)
    // Maybe in the end this equation optimization was not the best,
    // because the distance_scale - 1 is not super obvious / intuitive.
    // "Safety": This is safe, because distance_scale is guaranteed to be at least 1.
    let x = (distance_scale * theirs.x).checked_sub((distance_scale - 1) * ours.x)?;
    let y = (distance_scale * theirs.y).checked_sub((distance_scale - 1) * ours.y)?;

    // checked_sub handles left and top bound, but the others have to be checked manually:
    // (This needs to be inclusive since our bounds are [0, width[, we only go up to width-1)
    if x >= width || y >= height {
        return None;
    }

    Some(AntiNode { x, y })
}

#[test]
fn example_input() {
    let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
        .to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 14);
    assert_eq!(res.1, 34);
}

#[test]
fn p2_specific_smallish_example_input() {
    let input = "T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
.........."
        .to_owned();
    let res = solve_day(input);
    // assert_eq!(res.0, 14);
    assert_eq!(res.1, 9);
}

#[test]
fn small_example_input() {
    let input = "..........
..........
..........
....a.....
........a.
.....a....
..........
..........
..........
.........."
        .to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 4);
    // assert_eq!(res.1, 11387);
}

#[test]
fn small_example_input_with_big_a() {
    let input = "..........
..........
..........
....a.....
........a.
.....a....
..........
......A...
..........
.........."
        .to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 4);
    // assert_eq!(res.1, 11387);
}

#[test]
fn small_manual_example_input_test_underflow() {
    let input = "aaZ.
..Z.
....
...."
        .to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 2);
    // assert_eq!(res.1, 11387);
}

#[test]
fn small_manual_example_input_test_oob_diagonal_bottom_left() {
    let input = ".....
.....
..N..
.....
N...."
        .to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 1);
    // assert_eq!(res.1, 11387);
}

#[test]
fn manual_oob_right_and_bottom() {
    let input = "....
..aa
...b
...b"
        .to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 2);
    // assert_eq!(res.1, 11387);
}

#[test]
fn prod_solution() {
    use std::fs::read_to_string;

    let input = read_to_string(format!("inputs/{}", "8.in")).unwrap();
    let res = solve_day(input);
    // 366 is too high
    // 96 is "not right" - after diff calc change. (the diff calc was just straight up wrong lol)
    // New method and old got the same outcome, but the bounds check was what was wrong...
    // Added "manual_oob_right_and_bottom" to test for this...
    assert_eq!(res.0, 361);
    assert_eq!(res.1, 1249);
}

aoc2024::day_main!("8.in");
