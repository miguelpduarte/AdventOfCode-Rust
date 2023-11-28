#![feature(test)]
extern crate test;

fn solve_day(input: String) -> (usize, usize) {
    let p1 = input
        .lines()
        .map(|line| {
            // 1-2,3-4
            line.split(',')
                // 1-2
                .map(|item|
                    // 1 2
                    item.split('-').map(|part| part.parse::<u32>().unwrap()).collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .map(|pair| {
            let first = &pair[0];
            let second = &pair[1];
            return fully_contains(first, second);
        })
        .filter(|&x| x)
        .count();

    let p2 = input
        .lines()
        .map(|line| {
            // 1-2,3-4
            line.split(',')
                // 1-2
                .map(|item|
                    // 1 2
                    item.split('-').map(|part| part.parse::<u32>().unwrap()).collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .map(|pair| overlaps(&pair[0], &pair[1]))
        .filter(|&x| x)
        .count();

    (p1, p2)
}

fn fully_contains(a: &[u32], b: &[u32]) -> bool {
    // A contains B
    (a[0] <= b[0] && a[1] >= b[1])
    // or B contains A
    || (b[0] <= a[0] && b[1] >= a[1])
}

fn overlaps(a: &[u32], b: &[u32]) -> bool {
    // To NOT overlap: endX must be strictly smaller than startY (and vice-versa for unsorted pairs)
    !(a[1] < b[0] || b[1] < a[0])
}

#[test]
fn example_input() {
    let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
        .to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 2);
    assert_eq!(res.1, 4);
}

#[test]
fn prod_solution() {
    use std::fs::read_to_string;

    let input = read_to_string(format!("inputs/{}", "4.in")).unwrap();
    let res = solve_day(input);
    assert_eq!(res.0, 542);
    assert_eq!(res.1, 900);
}

aoc2022::day_main!("4.in");
