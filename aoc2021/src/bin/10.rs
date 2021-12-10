use std::collections::BinaryHeap;

/// Returns (corrupted_score, completion_score)
fn calc_scores(line: &str) -> (usize, usize) {
    let mut opener_stack = Vec::<char>::new();

    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => {
                opener_stack.push(c);
            }
            ')' => {
                if let Some(opener) = opener_stack.last() {
                    if *opener == '(' {
                        opener_stack.pop();
                        continue;
                    }
                }
                // Either stack is empty, or unmatched delimiter
                return (3, 0);
            }
            ']' => {
                if let Some(opener) = opener_stack.last() {
                    if *opener == '[' {
                        opener_stack.pop();
                        continue;
                    }
                }
                // Either stack is empty, or unmatched delimiter
                return (57, 0);
            }
            '}' => {
                if let Some(opener) = opener_stack.last() {
                    if *opener == '{' {
                        opener_stack.pop();
                        continue;
                    }
                }
                // Either stack is empty, or unmatched delimiter
                return (1197, 0);
            }
            '>' => {
                if let Some(opener) = opener_stack.last() {
                    if *opener == '<' {
                        opener_stack.pop();
                        continue;
                    }
                }
                // Either stack is empty, or unmatched delimiter
                return (25137, 0);
            }
            _ => {
                panic!("Unexpected char: {}", c);
            }
        }
    }

    let completion_score = opener_stack.iter().rfold(0, |acc, elem| {
        acc * 5
            + match elem {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => panic!("Unexpected char in stack: {}", elem),
            }
    });

    (0, completion_score)
}

fn solve_day(input: String) -> (usize, usize) {
    let (p1, p2_bheap): (usize, BinaryHeap<usize>) = input
        .lines()
        .map(|line| calc_scores(line))
        .fold((0, BinaryHeap::new()), |(acc1, mut acc2), (val1, val2)| {
            if val2 != 0 {
                acc2.push(val2);
            }
            (acc1 + val1, acc2)
        });

    // let n_incomplete = p2_bheap.len();
    // for _ in 0..(n_incomplete / 2) {
    //     p2_bheap.pop();
    // }
    // let p2 = p2_bheap.pop().unwrap();

    let sorted_incompletes = p2_bheap.into_sorted_vec();
    let p2 = sorted_incompletes[sorted_incompletes.len() / 2];

    (p1, p2)
}

#[test]
fn example_input() {
    let input = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"
        .to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 26397);
    assert_eq!(res.1, 288957);
}

#[test]
fn prod_solution() {
    use std::fs::read_to_string;

    let input = read_to_string(format!("inputs/{}", "10.in")).unwrap();
    let res = solve_day(input);
    assert_eq!(res.0, 318099);
    assert_eq!(res.1, 2389738699);
}

aoc2021::day_main!("10.in");
