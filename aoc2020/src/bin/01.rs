#[allow(dead_code)]
fn p1_naive(data: &[u32]) -> u32 {
    let mut res = 0;
    'a: for (idx1, n1) in data.iter().enumerate() {
        for (idx2, n2) in data.iter().enumerate() {
            if idx1 != idx2 && n1 + n2 == 2020 {
                res = n1 * n2;
                break 'a;
            }
        }
    }
    res
}

#[allow(dead_code)]
fn p2_naive(data: &[u32]) -> u32 {
    let mut res = 0;
    'a: for (idx1, n1) in data.iter().enumerate() {
        for (idx2, n2) in data.iter().enumerate() {
            for (idx3, n3) in data.iter().enumerate() {
                if idx1 != idx2 && idx2 != idx3 && n1 + n2 + n3 == 2020 {
                    res = n1 * n2 * n3;
                    break 'a;
                }
            }
        }
    }
    res
}

#[allow(dead_code)]
fn solve_day_naive(input: String) -> (u32, u32) {
    let data: Vec<u32> = input.lines().map(|x| x.parse::<u32>().unwrap()).collect();

    let p1 = p1_naive(&data);
    let p2 = p2_naive(&data);
    (p1, p2)
}

use itertools::Itertools;

#[allow(dead_code)]
fn solve_day_itertools(input: String) -> (u32, u32) {
    let data: Vec<u32> = input.lines().map(|x| x.parse::<u32>().unwrap()).collect();

    let p1 = data
        .iter()
        .tuple_combinations()
        .find(|&(a, b)| a + b == 2020)
        .map(|(a, b)| a * b);
    let p2 = data
        .iter()
        .tuple_combinations()
        .find(|&(a, b, c)| a + b + c == 2020)
        .map(|(a, b, c)| a * b * c);
    (p1.unwrap(), p2.unwrap())
}

fn solve_day(input: String) -> (u32, u32) {
    // solve_day_naive(input)
    solve_day_itertools(input)
}

#[test]
fn example_input() {
    let input = "1721
979
366
299
675
1456"
        .to_owned();

    let res = solve_day(input);
    assert_eq!(res.0, 514579);
    assert_eq!(res.1, 241861950);
}

aoc2020::day_main!("1.in");
