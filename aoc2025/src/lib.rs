/// Automatically generate main, tests and benchmarks for the current day. Improved from 2024 setup.
///
/// It assumes that the crate name is the zero-padded day number (e.g. `01`, `05`, `13`).
///
/// Arguments:
/// - function identifier (previously was just hardcoded `solve_day`);
///   Must be in shape:
///   `fn foo(input: String) -> (typeof prod_sol_part1, typeof prod_sol_part2)`
/// - prod_sol_part1 - "prod" solution value for part 1;
/// - prod_sol_part2 - "prod" solution value for part 2;
///
/// TODO: Separate macro to handle examples.
///
/// - ??? Something about examples later? Maybe for now just make that a separate macro so it's
///   easier :sweat_smile:
///
#[macro_export]
macro_rules! do_day {
    ($solver_fn:ident) => {
        // TODO: Handle case without input file.
        // fn main() {
        //     let now = std::time::Instant::now();
        //     let (res1, res2) = $solver_fn();
        //     let time = now.elapsed();
        //     println!("Part 1: {}", res1);
        //     println!("Part 2: {}", res2);
        //     println!("Time: {:?}", time);
        // }

        const DAY_NAME: &str = env!("CARGO_BIN_NAME");

        fn main() {
            println!("Starting {DAY_NAME}!");
            let input = ::std::fs::read_to_string(format!("inputs/{}.in", DAY_NAME)).unwrap();

            let now = std::time::Instant::now();
            let (res1, res2) = $solver_fn(input);
            let time = now.elapsed();
            println!("Ran day {DAY_NAME}!");
            println!("Part 1: {res1}");
            println!("Part 2: {res2}");
            println!("Time: {:?}", time);
        }

        #[cfg(test)]
        mod benches {
            use std::hint::black_box;

            use super::*;
            use test::Bencher;

            #[bench]
            fn prod_bench(b: &mut Bencher) {
                let input = ::std::fs::read_to_string(format!("inputs/{}.in", DAY_NAME)).unwrap();

                b.iter(|| $solver_fn(black_box(input.clone())));
            }
        }
    };

    ($solver_fn:ident, p1:$prod_sol_part1:expr$(, p2:$prod_sol_part2:expr)?) => {
        $crate::do_day!($solver_fn);

        #[cfg(test)]
        mod tests {
            use super::*;
            #[test]
            fn prod_solution() {
                let input = ::std::fs::read_to_string(format!("inputs/{}.in", DAY_NAME)).unwrap();
                let res = $solver_fn(input);
                assert_eq!(res.0, $prod_sol_part1);
                $(assert_eq!(res.1, $prod_sol_part2);)?
            }
        }

    };
}
