fn solve_day(input: String) -> (usize, usize) {
    // https://llogiq.github.io/2016/09/24/newline.html - good enough I guess
    let n_lines = input.as_bytes().iter().filter(|&&c| c == b'\n').count();

    let (gamma, n_used_bits) = input
        .lines()
        // line string to vec<ints>
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '0' => 0,
                    '1' => 1,
                    _ => panic!("Unexpected bit char {}", c),
                })
                .collect::<Vec<_>>()
        })
        // Sum each respective index of acc and the current elem, accumulating into each index the
        // number of 1s
        .reduce(|acc, elem| {
            acc.iter()
                .zip(elem.iter())
                .map(|(&a, &b)| a + b)
                .collect::<Vec<_>>()
        })
        // Because apparently .fold just outputs the result, but .reduce wraps it in an Option
        .unwrap()
        // Iterate the Vec of number of 1s
        .iter()
        // Create the usize from each bit
        .fold((0, 0), |(acc_gamma, n_used_bits), elem| {
            // If the number of 1s for an index is larger than half of the number of lines, 1s has
            // majority. Using a multiplication by 2 instead of division by 2 since that's easier
            // (and faster?)
            let new_bit = (elem * 2 > n_lines) as usize;
            ((acc_gamma << 1) + new_bit, n_used_bits + 1)
        });

    let gamma_complementer = !(usize::MAX << n_used_bits);
    let omega = gamma ^ gamma_complementer;
    let p1 = gamma * omega;

    // Part 2 why do you make me do this
    // Collect all lines into a Vec<Vec<u8>> (I think)
    let mut lines_bytes = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();

    let mut i = 0;

    loop {
        let n_items = lines_bytes.len();

        // Check if done
        if n_items == 1 {
            break;
        }

        // Otherwise, find the most famous bit and filter based on it
        let most_famous_sum: usize = lines_bytes
            .iter()
            .map(|line_bytes| match line_bytes[i] {
                b'0' => 0,
                b'1' => 1,
                c => panic!("Unexpected bit char {}", c),
            })
            .sum();

        let chosen_bit = if most_famous_sum * 2 >= n_items {
            b'1'
        } else {
            b'0'
        };

        lines_bytes = lines_bytes
            .into_iter()
            .filter(|line_bytes| line_bytes[i] == chosen_bit)
            .collect::<Vec<_>>();

        i += 1;
    }

    let oxygen: usize = lines_bytes[0].iter().fold(0, |acc, elem| {
        let new_bit = elem - b'0';
        (acc << 1) + new_bit as usize
    });

    // Round 2
    let mut lines_bytes = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();
    let mut i = 0;

    loop {
        let n_items = lines_bytes.len();

        // Check if done
        if n_items == 1 {
            break;
        }

        // Otherwise, find the most famous bit and filter based on it
        let most_famous_sum: usize = lines_bytes
            .iter()
            .map(|line_bytes| match line_bytes[i] {
                b'0' => 0,
                b'1' => 1,
                c => panic!("Unexpected bit char {}", c),
            })
            .sum();

        // Invert choice now
        let chosen_bit = if most_famous_sum * 2 >= n_items {
            b'0'
        } else {
            b'1'
        };

        lines_bytes = lines_bytes
            .into_iter()
            .filter(|line_bytes| line_bytes[i] == chosen_bit)
            .collect::<Vec<_>>();

        i += 1;
    }

    let co2: usize = lines_bytes[0].iter().fold(0, |acc, elem| {
        let new_bit = elem - b'0';
        (acc << 1) + new_bit as usize
    });

    let p2 = oxygen * co2;

    (p1, p2)
}

#[test]
fn example_input() {
    let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"
        .to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 198);
    assert_eq!(res.1, 230);
}

#[test]
fn prod_solution() {
    use std::fs::read_to_string;

    let input = read_to_string(format!("inputs/{}", "3.in")).unwrap();
    let res = solve_day(input);
    assert_eq!(res.0, 2595824);
    assert_eq!(res.1, 2135254);
}

aoc2021::day_main!("3.in");
