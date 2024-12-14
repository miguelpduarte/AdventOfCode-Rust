#![feature(test)]

extern crate test;

const XMAS_WORD_CHARS: [u8; 4] = [b'X', b'M', b'A', b'S'];
const MAX_GRID_SIZE: usize = 140;

fn solve_day(input: String) -> (usize, usize) {
    let mut word_matrix = [[u8::MAX; MAX_GRID_SIZE]; MAX_GRID_SIZE];
    let mut x_positions: Vec<(usize, usize)> = Vec::new();
    let mut a_positions: Vec<(usize, usize)> = Vec::new();

    // Build word matrix and cache X positions
    for (y, line) in input.lines().enumerate() {
        for (x, item) in line.bytes().enumerate() {
            word_matrix[y][x] = item;
            if item == XMAS_WORD_CHARS[0] {
                x_positions.push((x, y));
            } else if item == b'A' {
                a_positions.push((x, y));
            }
        }
    }

    let p1 = x_positions
        .iter()
        .map(|x_pos| count_xmases_dfs(x_pos, &word_matrix))
        .sum();

    let p2 = a_positions
        .into_iter()
        .filter(|&a_pos| count_mas_cross(a_pos, &word_matrix))
        .count();

    (p1, p2)
}

const COORD_SHIFT_CORNER_PAIRS: [((isize, isize), (isize, isize)); 2] =
    // top left and bottom right, top right and bottom left
    [((-1, -1), (1, 1)), ((1, -1), (-1, 1))];

fn count_mas_cross(a_pos: (usize, usize), word_matrix: &[[u8; 140]; 140]) -> bool {
    for (corner_a_shift, corner_b_shift) in COORD_SHIFT_CORNER_PAIRS {
        match (
            try_coord_shift_bounded_checked(a_pos, corner_a_shift),
            try_coord_shift_bounded_checked(a_pos, corner_b_shift),
        ) {
            (Some((xa, ya)), Some((xb, yb)))
                if (word_matrix[ya][xa] == b'M' && word_matrix[yb][xb] == b'S')
                    || (word_matrix[ya][xa] == b'S' && word_matrix[yb][xb] == b'M') =>
            {
                // So far so good. The continue is a no-op I guess.
                continue;
            }
            _ => {
                // At least one of the four corners if off-map
                // or does not have the opposing value (M to S, S to M).
                return false;
            }
        }
    }

    true
}

/// All coordinate shift options, going clockwise starting left
const COORD_SHIFTS: [(isize, isize); 8] = [
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
];

/// Using a DFS here instead because we have a clear max-depth of 4, and we need to keep going the
/// same direction always.
fn count_xmases_dfs(
    x_pos: &(usize, usize),
    word_matrix: &[[u8; MAX_GRID_SIZE]; MAX_GRID_SIZE],
) -> usize {
    let mut found_xmases = 0;

    for shift_direction in COORD_SHIFTS {
        let mut curr_coords = *x_pos;
        for i in 0..XMAS_WORD_CHARS.len() {
            // TODO: There's probably a more elegant way to write this :thinking_face:
            if i == XMAS_WORD_CHARS.len() - 1 {
                // We made it to the last index!
                // increment count and bail
                found_xmases += 1;
                break;
            }

            // Try always the same direction, pruning if not possible
            match try_coord_shift_bounded_checked(curr_coords, shift_direction) {
                // If the new coords are within bounds and the character is the next one we are
                // expecting, continue
                Some(new_coords @ (x, y)) if word_matrix[y][x] == XMAS_WORD_CHARS[i + 1] => {
                    curr_coords = new_coords;
                }
                _ => {
                    // Otherwise, bail
                    break;
                }
            }
        }
    }

    found_xmases
}

fn try_coord_shift_bounded_checked(
    (x, y): (usize, usize),
    (x_shift, y_shift): (isize, isize),
) -> Option<(usize, usize)> {
    let x = x.checked_add_signed(x_shift)?;
    let y = y.checked_add_signed(y_shift)?;
    // 0 bound is already checked in add_signed op.
    if x >= MAX_GRID_SIZE || y >= MAX_GRID_SIZE {
        None
    } else {
        Some((x, y))
    }
}

#[test]
fn example_input() {
    let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
        .to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 18);
    assert_eq!(res.1, 9);
}

#[test]
fn prod_solution() {
    use std::fs::read_to_string;

    let input = read_to_string(format!("inputs/{}", "4.in")).unwrap();
    let res = solve_day(input);
    assert_eq!(res.0, 2401);
    // 1875 is too high. And interesting:
    // "Curiously, it's the right answer for someone else; you might be logged in to the wrong
    // account or just unlucky. In any case, you need to be using your puzzle input."
    assert_eq!(res.1, 1822);
}

aoc2024::day_main!("4.in");
