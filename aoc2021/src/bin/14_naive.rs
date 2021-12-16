use itertools::interleave;
use std::collections::HashMap;

fn solve_day(input: String) -> (usize, usize) {
    let mut input_lines = input.lines();

    let mut polymer = input_lines.next().unwrap().as_bytes().to_vec();

    // Clear empty line
    input_lines.next();

    let rules: HashMap<u16, u8> = input_lines
        .map(|line| {
            // Since size is fixed we can parse like this
            let line_bytes = line.as_bytes();
            assert!(line_bytes.len() >= 6);
            // (second_char | first_char) -> new_char
            (
                u16::from_be_bytes([line_bytes[0], line_bytes[1]]),
                line_bytes[6],
            )
        })
        .collect();

    // println!(
    //     "{:?}",
    //     polymer.iter().map(|c| *c as char).collect::<Vec<_>>()
    // );
    for _ in 0..10 {
        // (c1, c2)
        let new_chars_iter = polymer.windows(2).map(|char_pair| {
            *rules
                .get(&u16::from_be_bytes([char_pair[0], char_pair[1]]))
                .unwrap()
        });

        // Spoiler: did not work, sometimes ate some values
        // // "intersperse" but with two iterators, I think?
        // polymer = polymer
        //     .iter()
        //     .zip(new_chars_iter)
        //     .flat_map(|(a, b)| [*a, b])
        //     .collect::<Vec<u8>>();

        // Thus, I am surrendering to itertools, at least for now
        polymer = interleave(polymer.iter().copied(), new_chars_iter).collect::<Vec<u8>>();
        // TODO: Test https://stackoverflow.com/questions/59322812 talking about zip -> chain

        // println!(
        //     "{:?}",
        //     polymer.iter().map(|c| *c as char).collect::<Vec<_>>()
        // );
    }

    // TODO: Try using an array instead to see if it's faster
    let mut counter: HashMap<u8, usize> = HashMap::with_capacity(10);

    for item in &polymer {
        *counter.entry(*item).or_insert(0) += 1;
    }

    let max = counter.iter().max_by_key(|&(_, count)| count).unwrap().1;
    let min = counter.iter().min_by_key(|&(_, count)| count).unwrap().1;

    let p1 = max - min;

    // This will probably crash me but oh well
    for _ in 0..30 {
        // (c1, c2)
        let new_chars_iter = polymer.windows(2).map(|char_pair| {
            *rules
                .get(&u16::from_be_bytes([char_pair[0], char_pair[1]]))
                .unwrap()
        });

        // Spoiler: did not work, sometimes ate some values
        // // "intersperse" but with two iterators, I think?
        // polymer = polymer
        //     .iter()
        //     .zip(new_chars_iter)
        //     .flat_map(|(a, b)| [*a, b])
        //     .collect::<Vec<u8>>();

        // Thus, I am surrendering to itertools, at least for now
        polymer = interleave(polymer.iter().copied(), new_chars_iter).collect::<Vec<u8>>();
        // TODO: Test https://stackoverflow.com/questions/59322812 talking about zip -> chain

        // println!(
        //     "{:?}",
        //     polymer.iter().map(|c| *c as char).collect::<Vec<_>>()
        // );
    }

    // TODO: Try using an array instead to see if it's faster
    let mut counter: HashMap<u8, usize> = HashMap::with_capacity(10);

    for item in &polymer {
        *counter.entry(*item).or_insert(0) += 1;
    }

    let max = counter.iter().max_by_key(|&(_, count)| count).unwrap().1;
    let min = counter.iter().min_by_key(|&(_, count)| count).unwrap().1;

    let p2 = max - min;
    // let p2 = 2;
    (p1, p2)
}

#[test]
fn example_input() {
    let input = "\
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"
        .to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 1588);
    assert_eq!(res.1, 2188189693529);
}

// #[test]
// fn prod_solution() {
//     use std::fs::read_to_string;

//     let input = read_to_string(format!("inputs/{}", "14.in")).unwrap();
//     let res = solve_day(input);
//     assert_eq!(res.0, None);
//     // assert_eq!(res.1, 107395);
// }

aoc2021::day_main!("14.in");
