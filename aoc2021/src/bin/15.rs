use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    x: usize,
    y: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.x.cmp(&other.x))
            .then_with(|| self.y.cmp(&other.y))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(
    risk_mat: &[Vec<usize>],
    start_x: usize,
    start_y: usize,
    goal_x: usize,
    goal_y: usize,
) -> Option<usize> {
    let mat_side = risk_mat.len();

    // The current lowest cost up to this index
    let mut cost_matrix: Vec<Vec<usize>> = (0..mat_side)
        .map(|_| (0..mat_side).map(|_| usize::MAX).collect())
        .collect();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    cost_matrix[start_y][start_x] = 0;
    heap.push(State {
        cost: 0,
        x: start_x,
        y: start_y,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, x, y }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if x == goal_x && y == goal_y {
            return Some(cost);
        }

        // Important as we may have already found a better way
        if cost > cost_matrix[y][x] {
            continue;
        }

        // TODO: Find a way to do this bounds check more elegantly
        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        if x > 0 {
            let next_x = x - 1;
            let next_y = y;
            let next = State {
                cost: cost + risk_mat[next_y][next_x],
                x: next_x,
                y: next_y,
            };

            // If so, add it to the frontier and continue
            if next.cost < cost_matrix[next.y][next.x] {
                heap.push(next);
                // Relaxation, we have now found a better way
                cost_matrix[next.y][next.x] = next.cost;
            }
        }
        if x < mat_side - 1 {
            let next_x = x + 1;
            let next_y = y;
            let next = State {
                cost: cost + risk_mat[next_y][next_x],
                x: next_x,
                y: next_y,
            };

            // If so, add it to the frontier and continue
            if next.cost < cost_matrix[next.y][next.x] {
                heap.push(next);
                // Relaxation, we have now found a better way
                cost_matrix[next.y][next.x] = next.cost;
            }
        }
        if y > 0 {
            let next_x = x;
            let next_y = y - 1;
            let next = State {
                cost: cost + risk_mat[next_y][next_x],
                x: next_x,
                y: next_y,
            };

            // If so, add it to the frontier and continue
            if next.cost < cost_matrix[next.y][next.x] {
                heap.push(next);
                // Relaxation, we have now found a better way
                cost_matrix[next.y][next.x] = next.cost;
            }
        }
        if y < mat_side - 1 {
            let next_x = x;
            let next_y = y + 1;
            let next = State {
                cost: cost + risk_mat[next_y][next_x],
                x: next_x,
                y: next_y,
            };

            // If so, add it to the frontier and continue
            if next.cost < cost_matrix[next.y][next.x] {
                heap.push(next);
                // Relaxation, we have now found a better way
                cost_matrix[next.y][next.x] = next.cost;
            }
        }
    }

    // Goal not reachable
    None
}

fn solve_day(input: String) -> (usize, usize) {
    let risk_mat = input
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|&c| (c - b'0') as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mat_len = risk_mat.len();
    let p1 = shortest_path(&risk_mat, 0, 0, mat_len - 1, mat_len - 1).unwrap();

    let risk_mat_p2 = (0..5)
        .flat_map(|j| {
            input
                .lines()
                .map(|line| {
                    (0..5)
                        .flat_map(|i| {
                            line.as_bytes()
                                .iter()
                                .map(|&c| (((c + i + j - b'0' - 1) % 9) + 1) as usize)
                                .collect::<Vec<_>>()
                        })
                        .collect()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mat_len = risk_mat_p2.len();
    let p2 = shortest_path(&risk_mat_p2, 0, 0, mat_len - 1, mat_len - 1).unwrap();

    (p1, p2)
}

#[test]
fn example_input() {
    let input = "\
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"
        .to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 40);
    assert_eq!(res.1, 315);
}

#[test]
fn prod_solution() {
    use std::fs::read_to_string;

    let input = read_to_string(format!("inputs/{}", "15.in")).unwrap();
    let res = solve_day(input);
    assert_eq!(res.0, 388);
    assert_eq!(res.1, 2819);
}

aoc2021::day_main!("15.in");
