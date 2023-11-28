/// Automatically generate main and benchmarks for the current day, inspired by
/// https://github.com/hugopeixoto/aoc2020 and https://github.com/AxlLind/AdventOfCode2020
/// Initially hammered my own benchmark thingy here since #[bench] is only available in nightly, now moved to nightly.

#[macro_export]
macro_rules! day_main {
    () => {
        fn main() {
            let now = std::time::Instant::now();
            let (res1, res2) = solve_day();
            let time = now.elapsed();
            println!("Part 1: {}", res1);
            println!("Part 2: {}", res2);
            println!("Time: {:?}", time);
        }
    };

    // TODO: Take in e.g. a day_id to use for the benchmark name (and also "prod" test if it is also generated by the macro in the future)
    // The prod test could be generated by taking in 2 params, one for each part. Might be overkill, though.
    ($input_file_name:literal) => {
        use std::fs::read_to_string;

        fn main() {
            let input = read_to_string(format!("inputs/{}", $input_file_name)).unwrap();

            let now = std::time::Instant::now();
            let (res1, res2) = solve_day(input);
            let time = now.elapsed();
            println!("Part 1: {}", res1);
            println!("Part 2: {}", res2);
            println!("Time: {:?}", time);
        }

        #[cfg(test)]
        mod benches {
            use std::hint::black_box;

            use super::*;
            use test::Bencher;

            #[bench]
            fn prod_bench(b: &mut Bencher) {
                use std::fs::read_to_string;
                let input = read_to_string(format!("inputs/{}", $input_file_name)).unwrap();

                b.iter(|| solve_day(black_box(input.clone())));
            }
        }
    };
}
