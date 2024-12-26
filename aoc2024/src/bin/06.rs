#![feature(test)]

extern crate test;

#[derive(Debug)]
enum Heading {
    Up,
    Right,
    Down,
    Left,
}

impl Heading {
    fn turn_right(self) -> Self {
        use Heading as H;
        match self {
            H::Up => H::Right,
            H::Right => H::Down,
            H::Down => H::Left,
            H::Left => H::Up,
        }
    }

    fn get_coord_shift(&self) -> (isize, isize) {
        match self {
            Heading::Up => (0, -1),
            Heading::Right => (1, 0),
            Heading::Down => (0, 1),
            Heading::Left => (-1, 0),
        }
    }
}

// Could've used an enum but trying to be more space-efficient with u8s
const GRID_EMPTY: u8 = 0;
const GRID_OBSTACLE: u8 = 1;
const GRID_SEEN: u8 = 2;

// TODO: Test making this a power of 2 to see if it's more efficient lol
const MAX_GRID_SIZE: usize = 140;

fn solve_day(input: String) -> (usize, usize) {
    let mut guard_start_pos: Option<(isize, isize)> = None;
    // So we can know when the guard goes off the grid
    let grid_size = input.lines().next().unwrap().len();

    // Ensures we can keep this as a [] rather than Vec
    assert!(grid_size <= MAX_GRID_SIZE);

    // TODO: Think that there might be an issue with "wrapping"
    // - might need to manually check bounds on right and left.
    let mut grid = [GRID_EMPTY; MAX_GRID_SIZE * MAX_GRID_SIZE];

    for (y, line) in input.trim_end().lines().enumerate() {
        for (x, item) in line.bytes().enumerate() {
            match item {
                // Obstacle
                b'#' => {
                    grid[x + y * MAX_GRID_SIZE] = GRID_OBSTACLE;
                }
                // Guard start pos
                b'^' => {
                    let _ = guard_start_pos.insert((x.try_into().unwrap(), y.try_into().unwrap()));
                }
                _ => {}
            }
        }
    }

    let (mut guard_x, mut guard_y) = guard_start_pos.expect("start position should be set");
    // Mark starting position as seen
    grid[guard_x as usize + guard_y as usize * MAX_GRID_SIZE] = GRID_SEEN;
    // Guard always starts facing up
    let mut guard_heading = Heading::Up;

    // Move the guard around until they go off map
    loop {
        let coord_shift = guard_heading.get_coord_shift();

        let next_x = guard_x + coord_shift.0;
        let next_y = guard_y + coord_shift.1;
        if next_x == -1
            || next_y == -1
            || next_x >= grid_size as isize
            || next_y >= grid_size as isize
        {
            // Went off grid in this step, we're done!
            break;
        }
        // We checked bounds above and only go 1 at a time, so this is safe
        let next_elem = &mut grid[next_x as usize + next_y as usize * MAX_GRID_SIZE];

        match next_elem {
            &mut GRID_OBSTACLE => {
                guard_heading = guard_heading.turn_right();
            }
            _ => {
                *next_elem = GRID_SEEN;
                guard_x = next_x;
                guard_y = next_y;
            }
        }
    }

    // TODO: Move this counting into above, can just check if the element was not previously seen.
    // TODO: We can also bench if it's faster to "cut-off" by iterating only within grid_size x
    // grid_size, rather than the whole array/matrix.
    let p1 = grid.into_iter().filter(|&elem| elem == GRID_SEEN).count();

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
    // 4452 is too low... yes I tested it and 4453 is also too low so it's not an off-by-one.
    // Ok it was actually 4454 since I forgot to mark the starting position as seen, and initially
    // I had an off-by-one error due to > rather than >= grid_size in the loop end condition.
    assert_eq!(res.0, 4454);
    assert_eq!(res.1, 42);
}

aoc2024::day_main!("6.in");
