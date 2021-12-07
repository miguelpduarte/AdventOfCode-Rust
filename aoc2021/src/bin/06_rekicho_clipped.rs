fn solve_day(input: String) -> (u64, u64) {
    let mut values: [u64; 9] = [0; 9];

    for number in input.split(',').map(|x| x.trim().parse::<usize>().unwrap()) {
        values[number] += 1;
    }

    for _ in 0..256 {
        let old_values = values;

        values[..8].clone_from_slice(&old_values[1..9]);

        values[8] = old_values[0];
        values[6] += old_values[0];
    }

    let res: u64 = values.iter().sum();
    // println!("{}", res);

    (1, res)
}

aoc2021::day_main!("6.in");
