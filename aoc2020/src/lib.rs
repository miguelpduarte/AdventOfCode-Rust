// Shamelessly stolen from https://github.com/danvk/aoc2020/blob/master/src/util.rs

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Automatically generate main and benchmarks for the current day, inspired by
/// https://github.com/hugopeixoto/aoc2020 and https://github.com/AxlLind/AdventOfCode2020
/// Since #[bench] is only available in nightly, hammered my own benchmark thingy here
#[macro_export]
macro_rules! day_main {
    () => {
        fn main() {
            let now = std::time::Instant::now();
            let (res1, res2) = solve_day();
            let time = now.elapsed().as_millis();
            println!("Part 1: {}", res1);
            println!("Part 2: {}", res2);
            println!("Time: {}ms", time);
        }
    };

    ($input_file_name:literal) => {
        use std::fs::read_to_string;

        fn main() {
            let input = read_to_string(format!("inputs/{}", $input_file_name)).unwrap();

            let now = std::time::Instant::now();
            let (res1, res2) = solve_day(input);
            let time = now.elapsed().as_millis();
            println!("Part 1: {}", res1);
            println!("Part 2: {}", res2);
            println!("Time: {}ms", time);
        }
    };
}

/// Benchmarks the function with the given identifier, setting time to ms elapsed
#[macro_export]
macro_rules! benchmark_fn {
    ($fn_name:ident) => {
        let now = std::time::Instant::now();
        // TODO: Find out how to handle returning values back to the caller elegantly, what is the
        // best way to return the time (or just print it?)
        $fn_name();
        let time = now.elapsed().as_millis();
    };
}
