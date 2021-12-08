use std::collections::VecDeque;

fn solve_day(input: String) -> (usize, usize) {
    // Number of fish in each stage
    let mut fish_stages: VecDeque<usize> = VecDeque::from([0; 9]);
    let mut n_fish: usize = 0;

    for item in input.trim().split(',') {
        let init_fish_state = item.parse::<usize>().unwrap();
        fish_stages[init_fish_state] += 1;
        n_fish += 1;
    }

    // let mut n_fish = fish_stages.iter().sum();

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
    // let p1 = fish_stages.iter().sum();

    for _day in 0..176 {
        let n_new_fishes = fish_stages[0];
        fish_stages.rotate_left(1);
        fish_stages[6] += n_new_fishes;
        n_fish += n_new_fishes;
    }

    let p2 = n_fish;
    // let p2 = fish_stages.iter().sum();

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
