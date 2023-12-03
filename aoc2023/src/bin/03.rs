#![feature(test)]

extern crate test;

// By the way, note to self: clippy's lint for single_char_pattern improved time from ~93us to ~48us
// See https://rust-lang.github.io/rust-clippy/master/index.html#/single_char_pattern

fn solve_day(input: String) -> (usize, usize) {
    let mut schematic = input.lines().map(str::to_owned).collect::<Vec<_>>();
    const SYMBOL_CHARS: [char; 10] = ['#', '$', '%', '&', '*', '+', '-', '/', '=', '@'];

    let mut sum_part_numbers = 0;
    let mut gear_ratio_sum = 0;

    for y in 0..schematic.len() {
        let symbols = schematic[y]
            .match_indices(SYMBOL_CHARS)
            .map(|(symbol_idx, symbol_str)|
            // We need to satisfy the borrow checker, and dropping here (even explicitly) was not accepted as enough
            (symbol_idx, symbol_str.to_owned()))
            .collect::<Vec<_>>();
        for (symbol_x, symbol_str) in symbols {
            // println!("Symbol {}", symbol_str);
            let mut found_numbers = vec![];
            // Upwards
            if y >= 1 {
                take_numbers_adjacent_to_symbol(
                    &mut found_numbers,
                    &mut schematic,
                    y - 1,
                    symbol_x,
                    false,
                );
            }

            // Same line
            take_numbers_adjacent_to_symbol(&mut found_numbers, &mut schematic, y, symbol_x, true);

            // Downwards
            if y + 1 < schematic.len() {
                take_numbers_adjacent_to_symbol(
                    &mut found_numbers,
                    &mut schematic,
                    y + 1,
                    symbol_x,
                    false,
                );
            }

            // println!("Found: {:?}", found_numbers);
            if symbol_str == "*" && found_numbers.len() == 2 {
                gear_ratio_sum += found_numbers[0] * found_numbers[1];
            }
            let local_sum: usize = found_numbers.iter().sum();
            sum_part_numbers += local_sum;
        }
    }

    let p1 = sum_part_numbers;

    let p2 = gear_ratio_sum;

    (p1, p2)
}

fn take_numbers_adjacent_to_symbol(
    numbers: &mut Vec<usize>,
    schematic: &mut Vec<String>,
    curr_y: usize,
    symbol_x: usize,
    ignore_center: bool,
) {
    // println!("For y = {}", curr_y);
    // Check left, center, right, just 3 range in a loop
    for x_shift in 0..=2 {
        if ignore_center && x_shift == 1 {
            // (When checking the middle line, the symbol in the center is just the special char)
            continue;
        }

        let line = schematic[curr_y].as_bytes();
        // Because -1..1 would require x_shift to be isize, we always subtract 1 and range 0..2
        let curr_x = symbol_x + x_shift - 1;
        if let Some(c) = line.get(curr_x) {
            // println!(
            //     "Tested {} and got {:?}",
            //     curr_x,
            //     String::from_utf8_lossy(&[*c])
            // );
            if c.is_ascii_digit() {
                // Now, find the rest of the number
                let mut right_bound = curr_x;
                for x in curr_x..line.len() {
                    if line[x].is_ascii_digit() {
                        right_bound = x;
                    } else {
                        // Number ends as it's no longer a consecutive digit string.
                        break;
                    }
                }
                let mut left_bound = curr_x;
                for x in (0..=curr_x).rev() {
                    if line[x].is_ascii_digit() {
                        left_bound = x;
                    } else {
                        // Number ends as it's no longer a consecutive digit string.
                        break;
                    }
                }

                let line = &mut schematic[curr_y];
                let number_str = &line[left_bound..=right_bound];
                // println!("x,y={},{} / {}", curr_x, curr_y, number_str);
                let number = number_str.parse::<usize>().unwrap();
                numbers.push(number);
                // Replace numbers we already found with '.'s so we don't match them twice.
                line.replace_range(
                    left_bound..=right_bound,
                    &".".repeat(right_bound - left_bound + 1),
                )
            }
        }
    }
}

#[test]
fn example_input() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
        .to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 4361);
    assert_eq!(res.1, 467835);
}

#[test]
fn prod_solution() {
    use std::fs::read_to_string;

    let input = read_to_string(format!("inputs/{}", "3.in")).unwrap();
    let res = solve_day(input);
    assert_eq!(res.0, 537732);
    assert_eq!(res.1, 84883664);
}

aoc2023::day_main!("3.in");
