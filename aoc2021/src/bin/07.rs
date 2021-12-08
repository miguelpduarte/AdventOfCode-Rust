fn solve_day(input: String) -> (usize, usize) {
    let mut vals: Vec<usize> = input
        .trim()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    vals.sort_unstable();

    let median = if vals.len() % 2 != 0 {
        vals[vals.len() / 2]
    } else {
        let mid = vals.len() / 2;
        (vals[mid] + vals[mid - 1]) / 2
    };

    // // Using a loop seems to be slower
    // let mut p1 = 0;
    // for &val in &vals {
    //     p1 += (val as i32).wrapping_sub(median as i32).abs() as usize;
    // }

    let p1: usize = vals
        .iter()
        // This is "abs_diff" (https://doc.rust-lang.org/std/primitive.usize.html#method.abs_diff)
        // but without nightly (taken from implementation)
        .map(|&x| (x as i32).wrapping_sub(median as i32).abs() as usize)
        .sum();

    // For part 2 we can use the average instead of the median

    let sum: usize = vals.iter().sum();
    let len = vals.len();
    // Ceiling div https://github.com/rust-lang/rfcs/issues/2844
    // let average = (sum + len - 1) / len;
    // Actually I want rounding div, not ceiling
    // let average = (sum + len / 2) / len;
    // Actually I want both, floor and ceiling div, since either can be the best (not possible to
    // know beforehand)
    let floor_average = sum / len;
    let ceil_average = floor_average + 1;

    // let mut floor_fuel: usize = 0;
    // let mut ceil_fuel: usize = 0;
    // for &val in &vals {
    //     let floor_abs_dif = (val as i32).wrapping_sub(floor_average as i32).abs() as usize;
    //     let ceil_abs_dif = (val as i32).wrapping_sub(ceil_average as i32).abs() as usize;
    //     floor_fuel += floor_abs_dif * (floor_abs_dif + 1) / 2;
    //     ceil_fuel += ceil_abs_dif * (ceil_abs_dif + 1) / 2;
    // }

    // Using two maps seems to be faster
    let floor_fuel: usize = vals
        .iter()
        // This is "abs_diff" (https://doc.rust-lang.org/std/primitive.usize.html#method.abs_diff)
        // but without nightly (taken from implementation)
        .map(|&x| {
            let abs_dif = (x as i32).wrapping_sub(floor_average as i32).abs() as usize;
            abs_dif * (abs_dif + 1) / 2
        })
        .sum();

    let ceil_fuel: usize = vals
        .iter()
        // This is "abs_diff" (https://doc.rust-lang.org/std/primitive.usize.html#method.abs_diff)
        // but without nightly (taken from implementation)
        .map(|&x| {
            let abs_dif = (x as i32).wrapping_sub(ceil_average as i32).abs() as usize;
            abs_dif * (abs_dif + 1) / 2
        })
        .sum();

    let p2 = std::cmp::min(floor_fuel, ceil_fuel);

    (p1, p2)
}

#[test]
fn example_input() {
    let input = "16,1,2,0,4,2,7,1,2,14".to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 37);
    assert_eq!(res.1, 168);
}

#[test]
fn prod_solution() {
    use std::fs::read_to_string;

    let input = read_to_string(format!("inputs/{}", "7.in")).unwrap();
    let res = solve_day(input);
    assert_eq!(res.0, 356179);
    assert_eq!(res.1, 99788435);
}

aoc2021::day_main!("7.in");
