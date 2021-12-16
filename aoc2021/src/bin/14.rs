use std::collections::HashMap;

fn solve_day(input: String) -> (usize, usize) {
    let mut input_lines = input.lines();

    // The pairs will be u16s. This array indexes pair_value -> count
    // TODO: This is quite large, and if the string is sparse maybe not worth it, consider using a
    // HashMap instead
    let mut polymer_pairs = [0_usize; u16::MAX as usize];

    let og_bytestring = input_lines.next().unwrap().as_bytes();
    // Counting the occurrences of each pair
    for char_pair in og_bytestring.windows(2) {
        polymer_pairs[u16::from_be_bytes([char_pair[0], char_pair[1]]) as usize] += 1;
    }

    // println!(
    //     "{:?}",
    //     polymer_pairs
    //         .iter()
    //         .enumerate()
    //         .filter(|(_idx, &count)| count != 0)
    //         .map(|(idx, _c)| {
    //             let bytes = (idx as u16).to_be_bytes();
    //             ([bytes[0] as char, bytes[1] as char], _c)
    //         })
    //         .collect::<Vec<_>>()
    // );

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

    // println!("{}", polymer_pairs.iter().sum::<usize>());
    for _i in 0..10 {
        // let mut new_polymer_pairs = polymer_pairs;
        let mut new_polymer_pairs = [0_usize; u16::MAX as usize];
        for (polymer_pair, count) in polymer_pairs.into_iter().enumerate() {
            if count == 0 {
                continue;
            }

            // This is the new char that will be inserted between the pair
            let new_elem = *rules.get(&(polymer_pair as u16)).unwrap();

            let polymer_pair_chars = (polymer_pair as u16).to_be_bytes();
            let first_pair = u16::from_be_bytes([polymer_pair_chars[0], new_elem]);
            let second_pair = u16::from_be_bytes([new_elem, polymer_pair_chars[1]]);

            // No longer need to uncount, since the array just starts at 0. Thanks silva
            // // As such, 'count' pairs of AB will stop existing,
            // // and 'count' pairs of A+New and New+B will start existing
            // // new_polymer_pairs[polymer_pair] = 0; // It was a bug to do this instead of -= count. Why?
            // // Because this is the new array and several pairs can originate this pair.
            // new_polymer_pairs[polymer_pair] -= count;
            new_polymer_pairs[first_pair as usize] += count;
            new_polymer_pairs[second_pair as usize] += count;
        }

        polymer_pairs = new_polymer_pairs;
        // println!("step {}", _i);
        // println!("{}", polymer_pairs.iter().sum::<usize>());
        // println!(
        //     "{:?}",
        //     polymer_pairs
        //         .iter()
        //         .enumerate()
        //         .filter(|(_idx, &count)| count != 0)
        //         .map(|(idx, _c)| {
        //             let bytes = (idx as u16).to_be_bytes();
        //             ([bytes[0] as char, bytes[1] as char], _c)
        //         })
        //         .collect::<Vec<_>>()
        // );
    }

    // TODO: Try using an array instead to see if it's faster
    let mut counter: HashMap<u8, usize> = HashMap::with_capacity(10);

    // Always sum the right character, so that we aren't duplicating any pairs
    for (pair, &count) in polymer_pairs.iter().enumerate() {
        if count == 0 {
            continue;
        }

        let right_char = (pair as u16).to_be_bytes()[1];
        *counter.entry(right_char).or_insert(0) += count;
    }

    // And also add the original string's first character
    let og_first_char = og_bytestring[0];
    *counter.entry(og_first_char).or_insert(0) += 1;

    // println!("{:?}", counter);

    let max = counter.iter().max_by_key(|&(_, count)| count).unwrap().1;
    let min = counter.iter().min_by_key(|&(_, count)| count).unwrap().1;

    let p1 = max - min;

    for _ in 0..30 {
        let mut new_polymer_pairs = [0_usize; u16::MAX as usize];

        for (polymer_pair, count) in polymer_pairs.into_iter().enumerate() {
            if count == 0 {
                continue;
            }

            // This is the new char that will be inserted between the pair
            let new_elem = *rules.get(&(polymer_pair as u16)).unwrap();

            let polymer_pair_chars = (polymer_pair as u16).to_be_bytes();
            let first_pair = u16::from_be_bytes([polymer_pair_chars[0], new_elem]);
            let second_pair = u16::from_be_bytes([new_elem, polymer_pair_chars[1]]);

            // As such, 'count' pairs of AB will stop existing,
            // and 'count' pairs of A+New and New+B will start existing
            new_polymer_pairs[first_pair as usize] += count;
            new_polymer_pairs[second_pair as usize] += count;
        }

        polymer_pairs = new_polymer_pairs;
    }

    // TODO: Try using an array instead to see if it's faster
    let mut counter: HashMap<u8, usize> = HashMap::with_capacity(10);

    // Always sum the right character, so that we aren't duplicating any pairs
    for (pair, &count) in polymer_pairs.iter().enumerate() {
        if count == 0 {
            continue;
        }

        let right_char = (pair as u16).to_be_bytes()[1];
        *counter.entry(right_char).or_insert(0) += count;
    }

    // And also add the original string's first character
    let og_first_char = og_bytestring[0];
    *counter.entry(og_first_char).or_insert(0) += 1;

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

#[test]
fn prod_solution() {
    use std::fs::read_to_string;

    let input = read_to_string(format!("inputs/{}", "14.in")).unwrap();
    let res = solve_day(input);
    assert_eq!(res.0, 5656);
    assert_eq!(res.1, 12271437788530);
}

aoc2021::day_main!("14.in");
