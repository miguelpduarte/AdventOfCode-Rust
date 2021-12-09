type CoordPair = ((usize, usize), (usize, usize));

fn create_inclusive_range(from: usize, to: usize) -> Box<dyn Iterator<Item = usize>> {
    if to > from {
        Box::new(from..=to)
    } else {
        Box::new((to..=from).rev())
    }
}

fn solve_day(input: String) -> (usize, usize) {
    let coord_pairs: Vec<CoordPair> = input
        .lines()
        .map(|line| line.split([',', '-', '>'].as_ref()).collect::<Vec<_>>())
        .map(|line_split| match *line_split.as_slice() {
            [x1, y1, _, x2, y2] => {
                let x1 = x1.trim().parse::<usize>().unwrap();
                let y1 = y1.trim().parse::<usize>().unwrap();
                let x2 = x2.trim().parse::<usize>().unwrap();
                let y2 = y2.trim().parse::<usize>().unwrap();
                ((x1, y1), (x2, y2))
            }
            _ => {
                panic!("No matcherino")
            }
        })
        .collect::<Vec<_>>();

    // println!("{:#?}", coord_pairs);

    // TODO: Maybe there can be less u8's if I used a larger uint and bit operations?
    // const MAT_SIZE: usize = 10; // For the example input, to have readable printing
    const MAT_SIZE: usize = 990;
    let mut vent_overlap_matrix = [[0_u8; MAT_SIZE]; MAT_SIZE];

    // for line in vent_overlap_matrix.iter() {
    //     println!("{:?}", line);
    // }

    // Only horizontal and vertical
    let (hor_vert_lines, diag_lines): (Vec<CoordPair>, Vec<CoordPair>) = coord_pairs
        .iter()
        .partition(|((x1, y1), (x2, y2))| x1 == x2 || y1 == y2);
    // .inspect(|((x1, y1), (x2, y2))| println!("{},{} to {},{}", x1, y1, x2, y2))
    hor_vert_lines.iter().for_each(|((x1, y1), (x2, y2))| {
        // Because someone thought it was funny to go in reverse
        // (By the way this is faster than the range reversing used for part 2 - Maybe due to the
        // Box heap allocations?)
        use std::cmp;
        let x_start = cmp::min(*x1, *x2);
        let x_end = cmp::max(*x1, *x2);
        let y_start = cmp::min(*y1, *y2);
        let y_end = cmp::max(*y1, *y2);

        for i in x_start..=x_end {
            #[allow(clippy::needless_range_loop)]
            for j in y_start..=y_end {
                vent_overlap_matrix[j][i] += 1;
            }
        }

        // // Made due to clippy::needless_range_loop
        // for vent_overlap_line in vent_overlap_matrix.iter_mut().take(y_end + 1).skip(y_start) {
        //     for vent_item in vent_overlap_line.iter_mut().take(x_end + 1).skip(x_start) {
        //         *vent_item += 1;
        //     }
        // }
    });

    // for line in vent_overlap_matrix.iter() {
    //     println!("{:?}", line);
    // }

    let p1_matrix_sum: usize = vent_overlap_matrix
        .iter()
        .map(|line| line.iter().filter(|&item| *item >= 2).count())
        .sum();

    diag_lines.iter().for_each(|((x1, y1), (x2, y2))| {
        for (i, j) in create_inclusive_range(*x1, *x2).zip(create_inclusive_range(*y1, *y2)) {
            vent_overlap_matrix[j][i] += 1;
        }
    });

    // for line in vent_overlap_matrix.iter() {
    //     println!("{:?}", line);
    // }

    let p2_matrix_sum: usize = vent_overlap_matrix
        .iter()
        .map(|line| line.iter().filter(|&item| *item >= 2).count())
        .sum();

    (p1_matrix_sum, p2_matrix_sum)
}

#[test]
fn example_input() {
    let input = "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2\
"
    .to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 5);
    assert_eq!(res.1, 12);
}

#[test]
fn prod_solution() {
    use std::fs::read_to_string;

    let input = read_to_string(format!("inputs/{}", "5.in")).unwrap();
    let res = solve_day(input);
    assert_eq!(res.0, 6005);
    assert_eq!(res.1, 23864);
}

aoc2021::day_main!("5.in");
