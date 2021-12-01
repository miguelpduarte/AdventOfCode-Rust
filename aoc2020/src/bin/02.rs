fn solve_day(input: String) -> (usize, usize) {
    let n_valid_p1 = input
        .lines()
        .map(|line| line.split(['-', ' ', ':'].as_ref()).collect::<Vec<_>>())
        .filter(|line_split| match *line_split.as_slice() {
            [low, high, chr, _empty, text] => {
                let low: usize = low.parse().unwrap();
                let high: usize = high.parse().unwrap();
                // println!("{} to {} by {}, and {}", low, high, chr, text);
                let n_chrs = text.matches(chr).count();
                low <= n_chrs && n_chrs <= high
            }
            _ => false,
        })
        .count();

    let n_valid_p2 = input
        .lines()
        .map(|line| line.split(['-', ' ', ':'].as_ref()).collect::<Vec<_>>())
        .filter(|line_split| match *line_split.as_slice() {
            [low, high, chr, _empty, text] => {
                let low: usize = low.parse().unwrap();
                let high: usize = high.parse().unwrap();
                // Assuming that chr is only one byte, aka one single ASCII character (it is)
                let chr: u8 = chr.as_bytes()[0];
                // Because rust handles UTF-8 gracefully, but we want to just index "easily" since
                // we are sure that input is only ASCII (See
                // https://stackoverflow.com/questions/24542115)
                let text = text.as_bytes();
                // Because a "logical XOR" is the same as a != in boolean context
                // "Either the low position has chr, or the high position has chr
                (text[low - 1] == chr) != (text[high - 1] == chr)
            }
            _ => false,
        })
        .count();

    (n_valid_p1, n_valid_p2)
}

#[test]
fn example_input() {
    let input = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"
        .to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 2);
    assert_eq!(res.1, 1);
}

#[test]
fn prod_solution() {
    use std::fs::read_to_string;

    let input = read_to_string(format!("inputs/{}", "2.in")).unwrap();
    let res = solve_day(input);
    assert_eq!(res.0, 607);
    assert_eq!(res.1, 321);
}

aoc2020::day_main!("2.in");
