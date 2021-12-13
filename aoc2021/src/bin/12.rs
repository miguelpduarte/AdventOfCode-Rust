use std::collections::HashMap;
use std::collections::HashSet;

fn count_paths<'a>(
    current: &'a str,
    connections: &HashMap<&str, HashSet<&'a str>>,
    visited_small: &mut HashSet<&'a str>,
) -> usize {
    if current == "end" {
        return 1;
    }

    // If the current cave is small, mark as visited
    if current.chars().all(|c| c.is_ascii_lowercase()) {
        visited_small.insert(current);
    }

    // Not done yet, get next nodes and go
    let next_candidates = connections.get(current).unwrap();

    // Don't go into the already visited small caves
    let next_caves = next_candidates - visited_small;

    let mut paths_from_here = 0;
    for next_cave in &next_caves {
        // println!("I'm at {}, sending to visit {}", current, next_cave);
        paths_from_here += count_paths(next_cave, connections, visited_small);
    }

    // After visiting children unvisit them so this doesn't affect other branches
    // (yay recursion :D)
    visited_small.remove(current);

    paths_from_here
}

fn count_paths_p2<'a>(
    current: &'a str,
    connections: &HashMap<&str, HashSet<&'a str>>,
    visited_small: &mut HashSet<&'a str>,
    doubled_small: Option<&str>,
) -> usize {
    if current == "end" {
        return 1;
    }

    // If the current cave is small, mark as visited
    if current.chars().all(|c| c.is_ascii_lowercase()) {
        visited_small.insert(current);
    }

    // Not done yet, get next nodes and go
    let next_candidates = connections.get(current).unwrap();

    match doubled_small {
        // Business as usual
        Some(the_doubled_small) => {
            // Don't go into the already visited small caves
            let next_caves = next_candidates - visited_small;

            let mut paths_from_here = 0;
            for next_cave in &next_caves {
                // println!("->zbr1 {} to {}", current, next_cave);
                paths_from_here +=
                    count_paths_p2(next_cave, connections, visited_small, doubled_small);
            }

            // After visiting children unvisit them so this doesn't affect other branches
            // (yay recursion :D)
            if the_doubled_small != current {
                // But first make sure that we are not "unvisiting" the node that we were
                // "double-visiting" (we can only reap what we sow)
                visited_small.remove(current);
            }

            paths_from_here
        }
        // Have the added possibility of visiting a small cave twice
        None => {
            let mut paths_from_here = 0;
            for next_cave_candidate in next_candidates {
                if visited_small.contains(next_cave_candidate)
                    && *next_cave_candidate != "start"
                    && *next_cave_candidate != "end"
                {
                    // println!("At {}, 2nd chance to {}", current, next_cave_candidate);
                    // Give some small caves a second chance!
                    paths_from_here += count_paths_p2(
                        next_cave_candidate,
                        connections,
                        visited_small,
                        Some(next_cave_candidate),
                    );
                // Probably the more elegant solution would be to start with "start" inside the
                // visited set? TODO: Consider that
                } else if *next_cave_candidate != "start" {
                    // println!("->zbr2 {} to {}", current, next_cave_candidate);
                    paths_from_here += count_paths_p2(
                        next_cave_candidate,
                        connections,
                        visited_small,
                        doubled_small,
                    );
                }
            }

            // After visiting children unvisit them so this doesn't affect other branches
            // (yay recursion :D)
            visited_small.remove(current);
            paths_from_here
        }
    }
}

fn solve_day(input: String) -> (usize, usize) {
    let mut origin_to_dest = HashMap::<&str, HashSet<&str>>::new();

    for line in input.lines() {
        let split = line.split('-').collect::<Vec<_>>();
        assert!(split.len() == 2);
        let (origin, dest) = (split[0], split[1]);

        let dest_set = origin_to_dest.entry(origin).or_insert_with(HashSet::new);
        dest_set.insert(dest);
        let rev_dest_set = origin_to_dest.entry(dest).or_insert_with(HashSet::new);
        rev_dest_set.insert(origin);
    }

    let mut visited_small = HashSet::<&str>::new();
    let p1 = count_paths("start", &origin_to_dest, &mut visited_small);
    let p2 = count_paths_p2("start", &origin_to_dest, &mut visited_small, None);

    (p1, p2)
}

#[test]
fn example_input() {
    let input = "\
start-A
start-b
A-c
A-b
b-d
A-end
b-end"
        .to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 10);
    assert_eq!(res.1, 36);
}

#[test]
fn larger_example_input() {
    let input = "\
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"
        .to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 19);
    assert_eq!(res.1, 103);
}

#[test]
fn even_larger_example_input() {
    let input = "\
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"
        .to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 226);
    assert_eq!(res.1, 3509);
}

#[test]
fn prod_solution() {
    use std::fs::read_to_string;

    let input = read_to_string(format!("inputs/{}", "12.in")).unwrap();
    let res = solve_day(input);
    assert_eq!(res.0, 3679);
    assert_eq!(res.1, 107395);
}

aoc2021::day_main!("12.in");
