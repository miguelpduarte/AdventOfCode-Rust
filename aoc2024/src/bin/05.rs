#![feature(test)]

use std::collections::{HashMap, HashSet};
extern crate test;

fn solve_day(input: String) -> (usize, usize) {
    // Associating left side to right side
    // (We may have multiple post-requirements for the same pre-req)
    let mut rules: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut sum_of_valid_middles = 0;

    for line in input.lines() {
        if line.contains('|') {
            let (prev, post) = line.split_once('|').unwrap();
            rules
                .entry(prev.parse().unwrap())
                .or_default()
                .push(post.parse().unwrap());
        } else {
            // Handle the empty newlines between rules and updates
            if line.is_empty() {
                continue;
            }
            // From now on, it's all updates, but we can just keep going here
            let update_page_nrs = line
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            if update_respects_ordering_rules(&update_page_nrs, &rules) {
                let middle = update_page_nrs[update_page_nrs.len() / 2];
                sum_of_valid_middles += middle;
            }
        }
    }

    let p1 = sum_of_valid_middles;

    let p2 = 0;

    (p1, p2)
}

fn update_respects_ordering_rules(
    update_page_nrs: &[usize],
    rules: &HashMap<usize, Vec<usize>>,
) -> bool {
    let mut seen = HashSet::new();
    for n in update_page_nrs {
        // We only need to check if the rule was _not_ respected

        // We check if any of the post requirements for this number has already been seen
        // (which would mean the rule was broken)
        if let Some(true) = rules
            .get(n)
            .map(|rule_posts| rule_posts.iter().any(|post| seen.contains(post)))
        {
            return false;
        }

        // Mark this number as seen
        seen.insert(n);
    }

    true
}

#[test]
fn example_input() {
    let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
        .to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 143);
    // assert_eq!(res.1, 81);
}

#[test]
fn prod_solution() {
    use std::fs::read_to_string;

    let input = read_to_string(format!("inputs/{}", "5.in")).unwrap();
    let res = solve_day(input);
    assert_eq!(res.0, 4959);
    assert_eq!(res.1, 42);
}

aoc2024::day_main!("5.in");
