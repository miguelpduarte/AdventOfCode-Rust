use std::collections::BinaryHeap;

fn get_basin_size(matrix: &mut [Vec<u8>], row: usize, col: usize) -> usize {
    let curr_val = matrix[row][col];

    // Already visited (in previous iterations), or "wall"
    if curr_val >= 9 {
        return 0;
    }

    let mut basin_size_acc = 0;

    // TODO: Maybe test with something other than pairs of coordinates? (As in, transform into a
    // single value) -> requires having a good estimate of matrix size and using that as a shift.
    // Since input matrix is 100x100, maybe we can use that as a baseline, and multiply value by
    // 1000 or something similar, then get both coords using / and %
    let mut to_visit_stack: Vec<(usize, usize)> = Vec::with_capacity(8);
    to_visit_stack.push((row, col));

    while !to_visit_stack.is_empty() {
        let (row, col) = to_visit_stack.pop().unwrap();
        let curr_val = matrix[row][col];
        // Already vistied, or "wall"
        if curr_val >= 9 {
            continue;
        }

        // Mark as visited
        matrix[row][col] += 10;
        // Increment as a basin element
        basin_size_acc += 1;

        // Add other elements to stack
        if col > 0 {
            to_visit_stack.push((row, col - 1));
        }
        if col < matrix[row].len() - 1 {
            to_visit_stack.push((row, col + 1));
        }
        if row > 0 {
            to_visit_stack.push((row - 1, col));
        }
        if row < matrix.len() - 1 {
            to_visit_stack.push((row + 1, col));
        }
    }

    basin_size_acc
}

macro_rules! reduce_by_10_if_over_10 {
    ($val:expr) => {
        if $val >= 10 {
            $val - 10
        } else {
            $val
        }
    };
}

fn solve_day(input: String) -> (usize, usize) {
    let mut height_mat: Vec<Vec<u8>> = input
        .trim()
        .lines()
        .map(|x|
            // x.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
            // Because we know it's all ascii (0-9), and this is faster
            x.as_bytes().iter().map(|c| c - b'0').collect())
        .collect();

    let mut local_mins_sum: usize = 0;

    let mut basin_sizes_maxheap: BinaryHeap<usize> = BinaryHeap::with_capacity(8);

    // println!("{}x{}", height_mat[0].len(), height_mat.len());

    for row in 0..height_mat.len() {
        for col in 0..height_mat[row].len() {
            let curr_val = reduce_by_10_if_over_10!(height_mat[row][col]);

            if curr_val == 0 {
                // Always a minimum (maybe?)
                local_mins_sum += 1 + curr_val as usize;
                basin_sizes_maxheap.push(get_basin_size(&mut height_mat, row, col));
                continue;
            }

            if curr_val == 9 {
                // Never a minimum
                continue;
            }

            if col > 0 {
                let top = reduce_by_10_if_over_10!(height_mat[row][col - 1]);
                if curr_val >= top {
                    continue;
                }
            }
            if col < height_mat[row].len() - 1 {
                let bottom = reduce_by_10_if_over_10!(height_mat[row][col + 1]);
                if curr_val >= bottom {
                    continue;
                }
            }
            if row > 0 {
                let left = reduce_by_10_if_over_10!(height_mat[row - 1][col]);
                if curr_val >= left {
                    continue;
                }
            }
            if row < height_mat.len() - 1 {
                let right = reduce_by_10_if_over_10!(height_mat[row + 1][col]);
                if curr_val >= right {
                    continue;
                }
            }

            local_mins_sum += 1 + curr_val as usize;
            basin_sizes_maxheap.push(get_basin_size(&mut height_mat, row, col));

            // let is_min = vals_arr.iter().all(|&x| curr_val < x);
            // if is_min {
            //     local_mins_sum += 1 + curr_val as usize;
            //     basin_sizes_maxheap.push(get_basin_size(&height_mat, curr_val, row, col));
            // }
        }
    }

    let p1 = local_mins_sum;

    let p2 = {
        assert!(basin_sizes_maxheap.len() >= 3);
        let elem1 = basin_sizes_maxheap.pop().unwrap();
        let elem2 = basin_sizes_maxheap.pop().unwrap();
        let elem3 = basin_sizes_maxheap.pop().unwrap();
        elem1 * elem2 * elem3
    };

    (p1, p2)
}

#[test]
fn example_input() {
    let input = "\
2199943210
3987894921
9856789892
8767896789
9899965678"
        .to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 15);
    assert_eq!(res.1, 1134);
}

#[test]
fn prod_solution() {
    use std::fs::read_to_string;

    let input = read_to_string(format!("inputs/{}", "9.in")).unwrap();
    let res = solve_day(input);
    assert_eq!(res.0, 524);
    // assert_eq!(res.1, 1046281);
}

aoc2021::day_main!("9.in");
