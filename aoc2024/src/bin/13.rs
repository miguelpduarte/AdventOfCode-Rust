#![feature(test)]
extern crate test;

fn parse_prefixed_coords(prefixed_coords: &str) -> (isize, isize) {
    let mut coords = prefixed_coords.split(", ").map(|prefixed_coord| {
        // Remove the X+/Y+/X=/Y= prefix and parse
        let (_, coord) = prefixed_coord.split_at(2);
        // Parsing it all as isize since it makes the math equation solving easier
        coord.parse::<isize>().unwrap()
    });

    (coords.next().unwrap(), coords.next().unwrap())
}

fn solve_day(input: String) -> (usize, usize) {
    let mut lines = input.lines();

    let mut total_cost = 0;

    while let (Some(a_line), Some(b_line), Some(prize_line)) =
        (lines.next(), lines.next(), lines.next())
    {
        let (_, a_coords) = a_line.split_once("Button A: ").unwrap();
        let (a_x, a_y) = parse_prefixed_coords(a_coords);

        // println!("a: {a_x},{a_y}");

        let (_, b_coords) = b_line.split_once("Button B: ").unwrap();
        let (b_x, b_y) = parse_prefixed_coords(b_coords);

        // println!("b: {b_x},{b_y}");

        let (_, prize_coords) = prize_line.split_once("Prize: ").unwrap();
        let (prize_x, prize_y) = parse_prefixed_coords(prize_coords);

        // println!("prize: {prize_x},{prize_y}");

        // Calculate the mathematical solution for the number of times we need to press each
        // button:
        match calc_prize_presses(prize_x, prize_y, a_x, a_y, b_x, b_y) {
            Some((a_presses, b_presses)) if a_presses < 100 && b_presses < 100 => {
                let cost = a_presses * 3 + b_presses;
                total_cost += cost;
                // println!("Solution was: {a_presses},{b_presses} ; cost = {cost}");
            }
            _ => {
                // println!("No solution for this one");
            }
        }

        // If parsing successful, skip the empty line regardless of if it exists or not
        lines.next();
    }

    let p1 = total_cost;

    let p2 = 0;

    (p1, p2)
}

/// Calculates the solution for the problem, considering it a 2nd degree linear equation system.
/// The final formula was achieved and validated using `maxima`:
/// (%i1) eq1: xp = ka*xa + kb*xb; eq2: yp = ka*ya + kb*yb;
/// (%o1)                         xp = kb xb + ka xa
/// (%o2)                         yp = kb yb + ka ya
/// (%i3) sol: solve([eq1,eq2], [ka,kb]);
///                           xp yb - xb yp       xp ya - xa yp
/// (%o3)            [[ka = - -------------, kb = -------------]]
///                           xb ya - xa yb       xb ya - xa yb
/// (%i4) subst([xp=8400,yp=5400,xa=94,xb=22,ya=34,yb=67], sol);
/// (%o4)                        [[ka = 80, kb = 40]]
///
/// For some reason, maxima got it wrong? The substitution worked, but applying Cramer's rule we
/// get a different outcome, now actually used in the code below.
fn calc_prize_presses(
    xp: isize,
    yp: isize,
    xa: isize,
    ya: isize,
    xb: isize,
    yb: isize,
) -> Option<(usize, usize)> {
    let denom = xb * ya - xa * yb;
    let ka = -(xp * yb - xb * yp) / denom;
    let kb = (xp * ya - xa * yp) / denom;

    Some((ka.try_into().ok()?, kb.try_into().ok()?))
}

#[test]
fn example_input() {
    let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"
        .to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 480);
    // assert_eq!(res.1, 81);
}

#[test]
fn prod_solution() {
    use std::fs::read_to_string;

    let input = read_to_string(format!("inputs/{}", "13.in")).unwrap();
    let res = solve_day(input);
    // 41635 is too high
    assert_eq!(res.0, 42);
    assert_eq!(res.1, 42);
}

aoc2024::day_main!("13.in");