fn solve_day(input: String) -> (usize, usize) {
    let calories_iter = input.split("\n\n").map(|elf_calories_str| {
        elf_calories_str
            .lines()
            .map(|x| x.parse::<usize>().unwrap())
            .sum::<usize>()
    });

    let mut top3 = [0; 3];

    for calories in calories_iter {
        let top_min_elem = top3.iter_mut().min().unwrap();
        if calories > *top_min_elem {
            *top_min_elem = calories;
        }
    }

    let p1 = *top3.iter().max().unwrap();

    let p2 = top3.iter().sum::<usize>();

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
