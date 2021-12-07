use std::collections::LinkedList;

fn solve_day(input: String) -> (usize, usize) {
    let mut fishez: LinkedList<u8> = input
        .split(',')
        .map(str::trim)
        .map(str::parse::<u8>)
        .map(Result::unwrap)
        .collect();

    let mut n_fish = fishez.len();

    for _day in 0..80 {
        // println!("{} {:?}", _day, fishez);
        let mut new_fishez = LinkedList::<u8>::new();
        for fish in fishez.iter_mut() {
            if *fish == 0 {
                *fish = 6;
                new_fishez.push_back(8);
                n_fish += 1;
                continue;
            }
            *fish -= 1;
        }
        fishez.append(&mut new_fishez);
    }

    let p1 = n_fish;

    for _day in 0..176 {
        let mut new_fishez = LinkedList::<u8>::new();
        for fish in fishez.iter_mut() {
            if *fish == 0 {
                *fish = 6;
                new_fishez.push_back(8);
                n_fish += 1;
                continue;
            }
            *fish -= 1;
        }
        fishez.append(&mut new_fishez);
    }

    let p2 = n_fish;

    (p1, p2)
}

#[test]
fn example_input() {
    let input = "3,4,3,1,2".to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 5934);
    assert_eq!(res.1, 26984457539);
}

#[test]
fn prod_solution() {
    use std::fs::read_to_string;

    let input = read_to_string(format!("inputs/{}", "6.in")).unwrap();
    let res = solve_day(input);
    assert_eq!(res.0, 386536);
    // assert_eq!(res.1, 23864);
}

aoc2021::day_main!("6.in");
