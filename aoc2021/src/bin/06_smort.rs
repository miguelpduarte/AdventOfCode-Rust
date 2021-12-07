use std::collections::VecDeque;

fn solve_day(input: String) -> (usize, usize) {
    let input_iter = input
        .split(',')
        .map(str::trim)
        .map(str::parse::<u8>)
        .map(Result::unwrap);

    let initial_fish = [
        0,
        input_iter.clone().filter(|&x| x == 1).count(),
        input_iter.clone().filter(|&x| x == 2).count(),
        input_iter.clone().filter(|&x| x == 3).count(),
        input_iter.clone().filter(|&x| x == 4).count(),
        input_iter.clone().filter(|&x| x == 5).count(),
        input_iter.clone().filter(|&x| x == 6).count(),
        0,
        0,
    ];

    // Number of fish in each stage
    let mut fish_stages: VecDeque<usize> = VecDeque::from(initial_fish);

    let mut n_fish = input_iter.count();

    for _day in 0..80 {
        let n_new_fishes = fish_stages[0];
        fish_stages.rotate_left(1);
        fish_stages[6] += n_new_fishes;
        n_fish += n_new_fishes;

        // [0, 1, 2, 3, 4, 5, 6, 7, 8]
        // [6, 0, 1, 2, 3, 4, 5, 6, 7]

        // [0, 1, 2, 3, 4, 5, 6, 7, 8]
        // rotate_left
        // [8, 0, 1, 2, 3, 4, 5, 6, 7]
        // Which is kinda right, so we just have to add the "old" fish that are now in state 6 back

        // println!("{} {:?}", _day, fishez);
    }

    let p1 = n_fish;

    for _day in 0..176 {
        let n_new_fishes = fish_stages[0];
        fish_stages.rotate_left(1);
        fish_stages[6] += n_new_fishes;
        n_fish += n_new_fishes;
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
    assert_eq!(res.1, 1732821262171);
}

aoc2021::day_main!("6.in");
