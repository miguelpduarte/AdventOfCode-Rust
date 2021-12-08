fn solve_day(input: String) -> (usize, usize) {
    let input_iter = input.trim().split(',').map(|x| x.parse::<usize>().unwrap());

    let mut fish_stages = [0; 9];
    let mut n_fish: usize = 0;

    for elem in input_iter {
        fish_stages[elem] += 1;
        n_fish += 1;
    }

    // for elem in input.trim().split(',') {
    //     let fish_init_state = elem.parse::<usize>().unwrap();
    //     fish_stages[fish_init_state] += 1;
    // }

    // let mut n_fish = input_iter.count();
    // Instead of rotating we can just index with a shift
    let mut index_shift = 0;

    for _day in 0..80 {
        let n_new_fishes = fish_stages[index_shift];
        // fish_stages.rotate_left(1); // Done via the index_shift
        index_shift = (index_shift + 1) % 9;
        fish_stages[(6 + index_shift) % 9] += n_new_fishes;
        n_fish += n_new_fishes;
    }

    let p1 = n_fish;

    for _day in 0..176 {
        let n_new_fishes = fish_stages[index_shift];
        // fish_stages.rotate_left(1); // Done via the index_shift
        index_shift = (index_shift + 1) % 9;
        fish_stages[(6 + index_shift) % 9] += n_new_fishes;
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
