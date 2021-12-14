const ALREADY_FLASHED: u8 = 150;

/// Flashes a certain position and propagates and flashes neighbors recursively, returning the
/// number of octopus flashed
fn flash_and_propagate(i: usize, j: usize, octopus: &mut [[u8; 10]; 10]) -> usize {
    let curr = octopus[j][i];
    if curr >= ALREADY_FLASHED || curr <= 9 {
        // Already flashed or not going to flash right now
        return 0;
    }

    // This octopus is going to flash
    // Set "already flashed"
    octopus[j][i] = ALREADY_FLASHED;

    let mut n_flashes = 1;

    // Increment and propagate flashing if not at a boundry
    if i > 0 {
        if j > 0 {
            octopus[j - 1][i - 1] += 1;
            n_flashes += flash_and_propagate(i - 1, j - 1, octopus);
        }
        octopus[j][i - 1] += 1;
        n_flashes += flash_and_propagate(i - 1, j, octopus);
        if j < 9 {
            octopus[j + 1][i - 1] += 1;
            n_flashes += flash_and_propagate(i - 1, j + 1, octopus);
        }
    }
    if j > 0 {
        octopus[j - 1][i] += 1;
        n_flashes += flash_and_propagate(i, j - 1, octopus);
    }
    // Don't self flash lol
    // octopus[j][i] += 1;
    // n_flashes += flash_and_propagate(i, j, octopus);
    if j < 9 {
        octopus[j + 1][i] += 1;
        n_flashes += flash_and_propagate(i, j + 1, octopus);
    }
    if i < 9 {
        if j > 0 {
            octopus[j - 1][i + 1] += 1;
            n_flashes += flash_and_propagate(i + 1, j - 1, octopus);
        }
        octopus[j][i + 1] += 1;
        n_flashes += flash_and_propagate(i + 1, j, octopus);
        if j < 9 {
            octopus[j + 1][i + 1] += 1;
            n_flashes += flash_and_propagate(i + 1, j + 1, octopus);
        }
    }

    n_flashes
}

fn solve_day(input: String) -> (usize, usize) {
    let mut octopus = [[0_u8; 10]; 10];

    for (i, line) in input.lines().enumerate() {
        for (j, item) in line.as_bytes().iter().enumerate() {
            let val = item - b'0';
            octopus[j][i] = val;
        }
    }

    let mut n_flashes = 0_usize;
    let mut already_synced = 0;

    for iteration in 0..100 {
        let mut n_flashes_this_iter = 0;
        for line in &mut octopus[..] {
            for item in &mut line[..] {
                *item += 1;
            }
        }

        for i in 0..10 {
            for j in 0..10 {
                let curr = octopus[j][i];
                if curr > 9 && curr < ALREADY_FLASHED {
                    // needs to propagate though (recursively? ofc)
                    n_flashes_this_iter += flash_and_propagate(i, j, &mut octopus);
                }
            }
        }
        n_flashes += n_flashes_this_iter;

        if n_flashes_this_iter == 100 {
            already_synced = iteration;
        }

        for i in 0..10 {
            for j in 0..10 {
                if octopus[j][i] >= ALREADY_FLASHED {
                    octopus[j][i] = 0;
                }
            }
        }
    }

    'outer: for iteration in 100.. {
        let mut n_flashes_this_iter = 0;
        for line in &mut octopus[..] {
            for item in &mut line[..] {
                *item += 1;
            }
        }

        for i in 0..10 {
            for j in 0..10 {
                let curr = octopus[j][i];
                if curr > 9 && curr < ALREADY_FLASHED {
                    // needs to propagate though (recursively? ofc)
                    n_flashes_this_iter += flash_and_propagate(i, j, &mut octopus);
                }
            }
        }
        // n_flashes += n_flashes_this_iter;

        if n_flashes_this_iter == 100 {
            already_synced = iteration;
            break 'outer;
        }

        for i in 0..10 {
            for j in 0..10 {
                if octopus[j][i] >= ALREADY_FLASHED {
                    octopus[j][i] = 0;
                }
            }
        }
    }

    let p1 = n_flashes;
    let p2 = already_synced + 1;
    (p1, p2)
}

#[test]
fn example_input() {
    let input = "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"
        .to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 1656);
    assert_eq!(res.1, 195);
}

#[test]
fn prod_solution() {
    use std::fs::read_to_string;

    let input = read_to_string(format!("inputs/{}", "11.in")).unwrap();
    let res = solve_day(input);
    assert_eq!(res.0, 1647);
    // assert_eq!(res.1, 99788435);
}

aoc2021::day_main!("11.in");
