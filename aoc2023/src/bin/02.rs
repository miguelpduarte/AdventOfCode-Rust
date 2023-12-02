#![feature(test)]
extern crate test;

// By the way, note to self: clippy's lint for single_char_pattern improved time from ~93us to ~48us
// See https://rust-lang.github.io/rust-clippy/master/index.html#/single_char_pattern

fn solve_day(input: String) -> (usize, usize) {
    const MAX_RED: usize = 12;
    const MAX_GREEN: usize = 13;
    const MAX_BLUE: usize = 14;
    let p1 = input
        .lines()
        .map(|line| {
            let (game_str, game_list) = line.split_once(": ").unwrap();
            // println!("Gamestr: {}, resto: {}", game_str, game_list);
            let game_id = game_str.replace("Game ", "").parse::<usize>().unwrap();

            for game in game_list.split(';') {
                // println!("The game: {}", game);
                for color in game.split(',') {
                    // println!("The colorlist: {:?}", color);
                    let (num, color_name) = color.trim().split_once(' ').unwrap();
                    let num = num.parse::<usize>().unwrap();
                    let color_possible = match color_name {
                        "red" => num <= MAX_RED,
                        "green" => num <= MAX_GREEN,
                        "blue" => num <= MAX_BLUE,
                        _ => panic!("Unexpected color"),
                    };
                    if !color_possible {
                        // Game isn't possible, don't count it.
                        return 0;
                    }
                }
            }

            game_id
        })
        .sum();

    let p2 = input
        .lines()
        .map(|line| {
            let (_game_str, game_list) = line.split_once(": ").unwrap();
            // println!("Gamestr: {}, resto: {}", game_str, game_list);
            // let game_id = game_str.replace("Game ", "").parse::<usize>().unwrap();

            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            for game in game_list.split(';') {
                // println!("The game: {}", game);
                for color in game.split(',') {
                    // println!("The colorlist: {:?}", color);
                    let (num, color_name) = color.trim().split_once(' ').unwrap();
                    let num = num.parse::<usize>().unwrap();
                    match color_name {
                        "red" => red = std::cmp::max(num, red),
                        "green" => green = std::cmp::max(num, green),
                        "blue" => blue = std::cmp::max(num, blue),
                        _ => panic!("Unexpected color"),
                    }
                }
            }

            red * green * blue
        })
        .sum();

    (p1, p2)
}

#[test]
fn example_input() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
        .to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 8);
    assert_eq!(res.1, 2286);
}

#[test]
fn prod_solution() {
    use std::fs::read_to_string;

    let input = read_to_string(format!("inputs/{}", "2.in")).unwrap();
    let res = solve_day(input);
    assert_eq!(res.0, 2727);
    assert_eq!(res.1, 56580);
}

aoc2023::day_main!("2.in");
