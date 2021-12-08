// use std::str::FromStr;
// impl FromStr for SignalBitmask {}

type SignalBitmask = u8;

/// Returns the respective bit set, to use in parse_to_signalbitmask
fn parse_bytechar_to_byte(input: u8) -> u8 {
    // This has the same result and performs the same
    // As such, keeping the solution below, as it is more explicit and has "error handling"
    // 1 << (input - b'a')

    match input {
        b'a' => 0b0000_0001,
        b'b' => 0b0000_0010,
        b'c' => 0b0000_0100,
        b'd' => 0b0000_1000,
        b'e' => 0b0001_0000,
        b'f' => 0b0010_0000,
        b'g' => 0b0100_0000,
        _ => panic!("Unexpected bytechar: {:?}", input),
    }
}

fn parse_to_signalbitmask(input: &str) -> SignalBitmask {
    input
        .as_bytes()
        .iter()
        .fold(0, |acc, &item| acc | parse_bytechar_to_byte(item))
}

// TODO: Make this cleaner (if possible?)
macro_rules! found_digit {
    ($signal_to_value:ident, $value_to_signal:ident, $signal_pattern:ident, $n_found_digits:ident, $digit:expr) => {
        $signal_to_value[$signal_pattern as usize] = $digit;
        $value_to_signal[$digit] = $signal_pattern;
        $n_found_digits += 1;
        // println!("  Found: {}", $digit);
    };
}

fn solve_day(input: String) -> (usize, usize) {
    let mut n_digits_w_unique_segments = 0;

    // For part 2 I am making the assumption that the 10 initial signal patterns represent unique
    // values (0-9). Otherwise I'm not sure that this is possible.
    // The example seems to suggest that as well so let's go with that :D

    let mut output_total = 0;

    for line in input.trim().lines() {
        // Maps a signal bitmask to the respective digit value
        // So we can do signal_to_value[signal_bitmask] to get the value [0-9]
        // Size is 255 since that's the possible values for the bitmask.
        // Some are impossible, so maybe a HashMap would be better, but w/e, we'll test that later
        // (Using usize to avoid casting in the sum part)
        let mut signal_to_value = [10_usize; 255];
        // The same as above, but in reverse. pass in a value as an index,
        // and get the respective bitmask
        let mut value_to_signal = [255_u8; 10];
        // This will tell us when to stop (when this is =10 we have found all possible digits)
        let mut n_found_digits: usize = 0;

        let mut line_iter = line.split('|');
        let signal_patterns: Vec<SignalBitmask> = line_iter
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .map(parse_to_signalbitmask)
            .collect();

        const BITMASK_8: SignalBitmask = 0b0111_1111;

        while n_found_digits != 10 {
            for &signal_pattern in signal_patterns.iter() {
                if signal_to_value[signal_pattern as usize] != 10 {
                    // Already found the correspondence for this bitmask, so just zoom on by
                    continue;
                }

                match signal_pattern.count_ones() {
                    // First the "easy" digits
                    2 => {
                        // signal_to_value[signal_pattern as usize] = 1;
                        // value_to_signal[1] = signal_pattern;
                        // n_found_digits += 1;
                        found_digit!(
                            signal_to_value,
                            value_to_signal,
                            signal_pattern,
                            n_found_digits,
                            1
                        );
                    }
                    4 => {
                        found_digit!(
                            signal_to_value,
                            value_to_signal,
                            signal_pattern,
                            n_found_digits,
                            4
                        );
                    }
                    3 => {
                        found_digit!(
                            signal_to_value,
                            value_to_signal,
                            signal_pattern,
                            n_found_digits,
                            7
                        );
                    }
                    7 => {
                        found_digit!(
                            signal_to_value,
                            value_to_signal,
                            signal_pattern,
                            n_found_digits,
                            8
                        );
                    }

                    // Now the "hard" ones, that can have several possibilites.
                    // For each check that a mask is a certain number, we also check if that number
                    // has already been found (assumption: all 10 are unique)
                    5 => {
                        // Can be either 2, 3 or 5
                        // TODO: "If both have been found, then it's the other". (Kind of done
                        // implicitly in the comparisons, but still
                        // TODO: Consider several possibilities for checking (i.e. have an outer
                        // check for the digit not being found yet and internally add several
                        // possibilites for bitmath with already found digits

                        // Checking if it is 2
                        if value_to_signal[2] == 255 {
                            // (this | 4) == 8
                            if value_to_signal[4] != 255
                                && (signal_pattern | value_to_signal[4]) == BITMASK_8
                            {
                                found_digit!(
                                    signal_to_value,
                                    value_to_signal,
                                    signal_pattern,
                                    n_found_digits,
                                    2
                                );
                                continue;
                            }
                        }

                        // Checking if it is 3
                        if value_to_signal[3] == 255 {
                            // (this & 1) has 2 segments set
                            // TODO: Compare efficiency with (this | 1) == this
                            if value_to_signal[1] != 255
                                && (value_to_signal[1] & signal_pattern).count_ones() == 2
                            {
                                found_digit!(
                                    signal_to_value,
                                    value_to_signal,
                                    signal_pattern,
                                    n_found_digits,
                                    3
                                );
                                continue;
                            }
                        }

                        // Checking if it is 5
                        if value_to_signal[5] == 255 {
                            let bm_4 = value_to_signal[4];
                            let bm_1 = value_to_signal[1];

                            if bm_4 != 255 && bm_1 != 255 {
                                // Bitmask with 'bd' set
                                let bm_bd = bm_4 ^ bm_1;

                                // (4 ^ 1) & this == (4 ^ 1)
                                if bm_bd & signal_pattern == bm_bd {
                                    found_digit!(
                                        signal_to_value,
                                        value_to_signal,
                                        signal_pattern,
                                        n_found_digits,
                                        5
                                    );
                                    continue;
                                }
                            }
                        }
                    }
                    6 => {
                        // Can be either 0, 6 or 9

                        // Checking if it is 0
                        if value_to_signal[0] == 255 {
                            // ((1 ^ 4) & this) has 1 segment set
                            // TODO: Can also check this with (7 ^ 4)

                            let bm_1 = value_to_signal[1];
                            let bm_4 = value_to_signal[4];

                            if bm_1 != 255
                                && bm_4 != 255
                                && ((bm_1 ^ bm_4) & signal_pattern).count_ones() == 1
                            {
                                found_digit!(
                                    signal_to_value,
                                    value_to_signal,
                                    signal_pattern,
                                    n_found_digits,
                                    0
                                );
                                continue;
                            }
                        }

                        // Checking if it is 6
                        if value_to_signal[6] == 255 {
                            // (this | 1) == 8
                            // TODO: Use any digit that has 'c' set instead of just 1, ex: (this | 7) == 8
                            if value_to_signal[1] != 255
                                && (signal_pattern | value_to_signal[1]) == BITMASK_8
                            {
                                found_digit!(
                                    signal_to_value,
                                    value_to_signal,
                                    signal_pattern,
                                    n_found_digits,
                                    6
                                );
                                continue;
                            }
                        }

                        // Checking if it is 9
                        if value_to_signal[9] == 255 {
                            // (this | 4) results in the same bitmask
                            if value_to_signal[4] != 255
                                && (signal_pattern | value_to_signal[4]) == signal_pattern
                            {
                                found_digit!(
                                    signal_to_value,
                                    value_to_signal,
                                    signal_pattern,
                                    n_found_digits,
                                    9
                                );
                                continue;
                            }
                        }
                    }

                    _ => {
                        panic!(
                            "Found unexpected number of 'ones' for a signal_pattern as binary: {}",
                            signal_pattern
                        );
                    }
                }
            }
        }

        let outputs = line_iter.next().unwrap();

        let mut line_total = 0;

        for output in outputs.trim().split(' ') {
            match output.len() {
                // 1 | 4 | 7 | 8
                2 | 4 | 3 | 7 => {
                    n_digits_w_unique_segments += 1;
                }
                _ => {}
            };

            let output_bitmask = parse_to_signalbitmask(output);
            let output_value = signal_to_value[output_bitmask as usize];
            // Shifting the other digits to the left 1 "space"
            line_total *= 10;
            line_total += output_value;
        }

        output_total += line_total;
    }

    let p1 = n_digits_w_unique_segments;
    let p2 = output_total;

    (p1, p2)
}

//TODO: Add the small test here, just because :)

#[test]
fn example_input() {
    // let input = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf".to_owned();
    let input = "\
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
        .to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 26);
    assert_eq!(res.1, 61229);
}

#[test]
fn prod_solution() {
    use std::fs::read_to_string;

    let input = read_to_string(format!("inputs/{}", "8.in")).unwrap();
    let res = solve_day(input);
    assert_eq!(res.0, 440);
    assert_eq!(res.1, 1046281);
}

aoc2021::day_main!("8.in");
