#![feature(test)]
extern crate test;

const MAX_GRID_SIZE: usize = 50;

fn solve_day(input: String) -> (usize, usize) {
    let mut trailhead_locs = vec![];
    let mut peak_locs = vec![];

    // Just assuming that it's a square, aligns with input and example
    let grid_size = input.lines().next().unwrap().len();
    // Checking that we don't break our static and constant allocation
    assert!(grid_size <= MAX_GRID_SIZE);

    // We allocate space to the maximum of our input (50), but note in grid_size our real bounds.
    // TODO: Test doing a contiguous array of 50*50 length to see if it's faster.
    // Using u8 since we only need 0-9. Marker value of 255 means we can never go there, and don't
    // even need to bounds check nicely other than for 0 or length.
    let mut grid = [[u8::MAX; MAX_GRID_SIZE]; MAX_GRID_SIZE];

    for (y, line) in input.lines().enumerate() {
        for (x, height) in line.bytes().enumerate() {
            // Assuming only ASCII 0-9.
            let height = height - b'0';
            grid[y][x] = height;

            if height == 0 {
                // Trailhead detected, cache for easier access
                trailhead_locs.push((x, y));
            } else if height == 9 {
                // Cache peaks for easier access for p2 as well.
                peak_locs.push((x, y));
            }
        }
    }

    let trailhead_scores = trailhead_locs
        .iter()
        .map(|trailhead| calc_trailhead_score(*trailhead, &grid))
        .sum();

    let p1 = trailhead_scores;

    let trailhead_ratings = trailhead_locs
        .iter()
        .map(|trailhead| calc_trailhead_rating(*trailhead, &grid))
        .sum();

    let p2 = trailhead_ratings;

    (p1, p2)
}

/// Lmao, part 2 is actually solvable by just removing the repeated path checking I had for part 1.
/// (Literally the only difference between the two is not having the set to track the duplicate
/// visits to a node, which I had accidentally forgot for part 1 initially).
fn calc_trailhead_rating(
    trailhead_loc: (usize, usize),
    grid: &[[u8; MAX_GRID_SIZE]; MAX_GRID_SIZE],
) -> usize {
    let mut rating = 0;

    // Start at trailhead, and then fan out (BFS, FIFO)
    let mut to_visit = vec![trailhead_loc];

    while let Some((curr_x, curr_y)) = to_visit.pop() {
        let curr_val = grid[curr_y][curr_x];

        if curr_val == 9 {
            // Trailhead got us to another peak
            rating += 1;
            // Since we cannot go upwards from 9, we don't need to explore anymore
            continue;
        }

        let target_value = curr_val + 1;

        let candidate_neighbour_coords = [
            // TODO: Maybe extract this to const, maybe it gets compiled in and is faster? probably
            // overengineering the optimization though.
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
        ]
        .into_iter()
        // Calculate neighbour coordinates with underflow check
        .filter_map(|(x_shift, y_shift)| {
            Some((
                curr_x.checked_add_signed(x_shift)?,
                curr_y.checked_add_signed(y_shift)?,
            ))
        })
        // Check outer bounds
        .filter(|(x, y)| {
            // The bound of being > 0 is already checked via usize::checked_add_signed, as it would
            // underflow
            // TODO: Technically we could use the parsed grid_size here to cut some exploration,but
            // it's unlikely to be relevant, as the real input would be the slower case and there
            // we use the full grid so this would be correct.
            *x < MAX_GRID_SIZE && *y < MAX_GRID_SIZE
        })
        // Check if the candidate neighbour has our target value (1 higher than the current value)
        .filter(|(x, y)| grid[*y][*x] == target_value);

        // Add all candidate neighbours to the exploration queue
        for candidate_neighbour_coords in candidate_neighbour_coords {
            to_visit.push(candidate_neighbour_coords);
        }
    }

    // When the loop is done, we have checked all paths. Since we need to strictly grow, we cannot
    // have cycles - so we don't need to worry about tracking what we already visited.
    rating
}

fn calc_trailhead_score(
    trailhead_loc: (usize, usize),
    grid: &[[u8; MAX_GRID_SIZE]; MAX_GRID_SIZE],
) -> usize {
    let mut score = 0;
    // Start at trailhead, and then fan out (BFS, FIFO)
    let mut to_visit = vec![trailhead_loc];

    // Sadly, we need to track visited nodes because we might have two paths leading up to the same
    // endpoint, which would lead to us counting the same trail multiple times.
    let mut seen = std::collections::HashSet::from([trailhead_loc]);

    while let Some(curr_coords @ (curr_x, curr_y)) = to_visit.pop() {
        seen.insert(curr_coords);

        let curr_val = grid[curr_y][curr_x];

        if curr_val == 9 {
            // Trailhead got us to another peak
            score += 1;
            // Since we cannot go upwards from 9, we don't need to explore anymore
            continue;
        }

        let target_value = curr_val + 1;

        let candidate_neighbour_coords = [
            // TODO: Maybe extract this to const, maybe it gets compiled in and is faster? probably
            // overengineering the optimization though.
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
        ]
        .into_iter()
        // Calculate neighbour coordinates with underflow check
        .filter_map(|(x_shift, y_shift)| {
            Some((
                curr_x.checked_add_signed(x_shift)?,
                curr_y.checked_add_signed(y_shift)?,
            ))
        })
        // Check outer bounds
        .filter(|(x, y)| {
            // The bound of being > 0 is already checked via usize::checked_add_signed, as it would
            // underflow
            // TODO: Technically we could use the parsed grid_size here to cut some exploration,but
            // it's unlikely to be relevant, as the real input would be the slower case and there
            // we use the full grid so this would be correct.
            *x < MAX_GRID_SIZE && *y < MAX_GRID_SIZE
        })
        // Check if the candidate neighbour has our target value (1 higher than the current value)
        .filter(|(x, y)| grid[*y][*x] == target_value)
        // We have not seen this node yet
        .filter(|coord| !seen.contains(coord));

        // Add all candidate neighbours to the exploration queue
        for candidate_neighbour_coords in candidate_neighbour_coords {
            to_visit.push(candidate_neighbour_coords);
        }
    }

    // When the loop is done, we have checked all paths. Since we need to strictly grow, we cannot
    // have cycles - so we don't need to worry about tracking what we already visited.

    score
}

#[test]
fn example_input() {
    let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
        .to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 36);
    assert_eq!(res.1, 81);
}

#[test]
fn prod_solution() {
    use std::fs::read_to_string;

    let input = read_to_string(format!("inputs/{}", "10.in")).unwrap();
    let res = solve_day(input);
    assert_eq!(res.0, 629);
    assert_eq!(res.1, 1242);
}

aoc2024::day_main!("10.in");
