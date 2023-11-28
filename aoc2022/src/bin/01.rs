#![feature(test)]
extern crate test;

use std::collections::BinaryHeap;

fn solve_day(input: String) -> (usize, usize) {
    let mut heap = BinaryHeap::new();
    let mut curr = 0;

    for line in input.lines() {
        if line.trim().is_empty() {
            heap.push(curr);
            curr = 0;
        } else {
            let n = line.parse::<usize>().unwrap();
            curr += n;
        }
    }

    // No ending newline, add the last one
    heap.push(curr);

    let max = heap.pop().unwrap();

    let p1 = max;

    let p2 = max + heap.pop().unwrap() + heap.pop().unwrap();
    println!("{:?}", heap.peek());

    (p1, p2)
}

fn solve_day_old_simple(input: String) -> (usize, usize) {
    let mut max = 0;
    let mut curr = 0;

    for line in input.lines() {
        if line.trim().is_empty() {
            if curr > max {
                max = curr;
            }
            curr = 0;
        } else {
            let n = line.parse::<usize>().unwrap();
            curr += n;
        }
    }

    // Final check because there is no ending newline lol
    if curr > max {
        max = curr;
    }

    let p1 = max;

    // let (p1, _) = input.lines().map(|x| x.parse::<usize>().unwrap()).fold(
    //     (0, None),
    //     |(n_incrs, last), elem| match last {
    //         None => (0, Some(elem)),
    //         Some(last) => (n_incrs + ((elem > last) as usize), Some(elem)),
    //     },
    // );

    // let (p2, _) = input
    //     .lines()
    //     .map(|x| x.parse::<usize>().unwrap())
    //     // Iterate over groups of 3
    //     .tuple_windows()
    //     // And sum them
    //     .map(|(x1, x2, x3)| x1 + x2 + x3)
    //     .fold((0, None), |(n_incrs, last), elem| match last {
    //         None => (0, Some(elem)),
    //         Some(last) => (n_incrs + ((elem > last) as usize), Some(elem)),
    //     });

    let p2 = 2;

    (p1, p2)
}

#[test]
fn example_input() {
    let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"
        .to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 24000);
    assert_eq!(res.1, 45000);
}

#[test]
fn prod_solution() {
    use std::fs::read_to_string;

    let input = read_to_string(format!("inputs/{}", "1.in")).unwrap();
    let res = solve_day(input);
    assert_eq!(res.0, 65912);
    assert_eq!(res.1, 195625);
}

aoc2022::day_main!("1.in");
