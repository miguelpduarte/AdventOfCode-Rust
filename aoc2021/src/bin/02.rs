fn solve_day(input: String) -> (usize, usize) {
    let (hor, vert) = input
        .lines()
        .map(|line| line.split(' ').collect::<Vec<_>>())
        .fold((0, 0), |(hor, vert), command| match *command.as_slice() {
            [direction, amount] => {
                let amount: usize = amount.parse().unwrap();
                match direction {
                    "forward" => (hor + amount, vert),
                    "down" => (hor, vert + amount),
                    "up" => (hor, vert - amount),
                    _ => panic!("Unknown command {}", direction),
                }
            }
            _ => (hor, vert),
        });

    let p1 = hor * vert;

    let (hor, vert, _aim) = input
        .lines()
        .map(|line| line.split(' ').collect::<Vec<_>>())
        .fold((0, 0, 0), |(hor, vert, aim), command| {
            match *command.as_slice() {
                [direction, amount] => {
                    let amount: usize = amount.parse().unwrap();
                    match direction {
                        "forward" => (hor + amount, vert + aim * amount, aim),
                        "down" => (hor, vert, aim + amount),
                        "up" => (hor, vert, aim - amount),
                        _ => panic!("Unknown command {}", direction),
                    }
                }
                _ => (hor, vert, aim),
            }
        });

    let p2 = hor * vert;

    (p1, p2)
}

#[test]
fn example_input() {
    let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2"
        .to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 150);
    assert_eq!(res.1, 900);
}

#[test]
fn prod_solution() {
    use std::fs::read_to_string;

    let input = read_to_string(format!("inputs/{}", "2.in")).unwrap();
    let res = solve_day(input);
    assert_eq!(res.0, 1499229);
    assert_eq!(res.1, 1340836560);
}

aoc2021::day_main!("2.in");
