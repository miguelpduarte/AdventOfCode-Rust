use itertools::Itertools;

fn solve_day(input: String) -> (usize, usize) {
    let (p1, _) = input.lines().map(|x| x.parse::<usize>().unwrap()).fold(
        (0, None),
        |(n_incrs, last), elem| match last {
            None => (0, Some(elem)),
            Some(last) => (n_incrs + ((elem > last) as usize), Some(elem)),
        },
    );

    let (p2, _) = input
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        // Iterate over groups of 3
        .tuple_windows()
        // And sum them
        .map(|(x1, x2, x3)| x1 + x2 + x3)
        .fold((0, None), |(n_incrs, last), elem| match last {
            None => (0, Some(elem)),
            Some(last) => (n_incrs + ((elem > last) as usize), Some(elem)),
        });

    (p1, p2)
}

#[test]
fn example_input() {
    let input = "199
200
208
210
200
207
240
269
260
263"
    .to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 7);
    assert_eq!(res.1, 5);
}

#[test]
fn prod_solution() {
    use std::fs::read_to_string;

    let input = read_to_string(format!("inputs/{}", "1.in")).unwrap();
    let res = solve_day(input);
    assert_eq!(res.0, 1711);
    assert_eq!(res.1, 1743);
}

aoc2021::day_main!("1.in");
