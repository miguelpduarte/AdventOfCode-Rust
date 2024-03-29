#![feature(test)]
extern crate test;

fn solve_day(input: String) -> (String, String) {
    // Each column takes 3-4 characters, both truncating a division by 3 or rounding up a division by 4 seem to work so sticking with it for now.
    let n_cols = input.lines().next().unwrap().len() / 4 + 1;
    let mut cols: Vec<Vec<u8>> = vec![Vec::new(); n_cols];

    let mut lines = input.lines();

    // Parsing
    for line in &mut lines {
        // Stop parsing the crates when we get the index numbers rather than the empty line as otherwise we are inserting the indexes as crates
        // This didn't make any difference to the end result, but it's neater to not add unnecessary crates.
        if line.as_bytes()[1] == b'1' {
            // Skip the empty line
            lines.next();
            break;
        }

        let line_bytes = line.as_bytes();
        // each column has the format '[X] ' or '   ' if empty.
        for col_idx in 0..n_cols {
            let item = line_bytes[col_idx * 4 + 1];
            if item != b' ' {
                let column = &mut (cols[col_idx]);
                column.insert(0, item);
            }
        }
    }

    // Cloning so that we can compute part 1 and 2 simultaneously
    let mut cols_p2 = cols.clone();

    // Computing moves
    for action in &mut lines {
        let (n, from, to) = parse_action(action);
        // Converting to 0-based indexes
        let from = from - 1;
        let to = to - 1;

        for _times in 0..n {
            let item = cols[from].pop().unwrap();
            cols[to].push(item);
        }

        // p1 can also be implemented in a similar way to p2. Benchmarks show no real difference (it's already quite fast so the difference is hard to tell anyway)
        // let split_index = cols[from].len() - n;
        // let moved_items = cols[from].split_off(split_index);
        // cols[to].extend(moved_items.iter().rev());

        let split_index = cols_p2[from].len() - n;
        let moved_items = cols_p2[from].split_off(split_index);
        cols_p2[to].extend(moved_items);
    }

    let p1_bytes = cols
        .iter_mut()
        .map(|col| col.pop().unwrap())
        .collect::<Vec<_>>();

    let p1 = String::from_utf8_lossy(&p1_bytes).to_string();

    let p2_bytes = cols_p2
        .iter_mut()
        .map(|col| col.pop().unwrap())
        .collect::<Vec<_>>();
    let p2 = String::from_utf8_lossy(&p2_bytes).to_string();

    (p1, p2)
}

fn parse_action(action: &str) -> (usize, usize, usize) {
    // move $amount from $from to $to
    let mut words = action.split(' ');
    // "move"
    words.next();
    let amount = words.next().unwrap().parse::<usize>().unwrap();
    // "from"
    words.next();
    let from = words.next().unwrap().parse::<usize>().unwrap();
    // "to"
    words.next();
    let to = words.next().unwrap().parse::<usize>().unwrap();

    (amount, from, to)
}

#[test]
fn example_input() {
    let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
        .to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, "CMZ");
    assert_eq!(res.1, "MCD");
}

#[test]
fn prod_solution() {
    use std::fs::read_to_string;

    let input = read_to_string(format!("inputs/{}", "5.in")).unwrap();
    let res = solve_day(input);
    assert_eq!(res.0, "ZRLJGSCTR");
    assert_eq!(res.1, "PRTTGRFPB");
}

aoc2022::day_main!("5.in");
