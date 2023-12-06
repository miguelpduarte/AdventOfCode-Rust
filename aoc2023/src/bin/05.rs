#![feature(test)]
#![feature(hash_extract_if)]

use std::{collections::HashSet, str::Lines};

extern crate test;

fn solve_day(input: String) -> (usize, usize) {
    let mut lines = input.lines();

    let (_label, seed_str) = lines.next().unwrap().split_once(':').unwrap();
    let seeds = seed_str
        .trim()
        .split_ascii_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect::<HashSet<_>>();

    // println!("seeds: {:?}", seeds);
    lines.next();
    lines.next();
    // Start of "seed-to-soil map"
    let soils = process_map(&mut lines, seeds);
    // println!("soils: {:?}", soils);

    // "soil-to-fertilizer"
    lines.next();
    // Advance the map label
    let fertilizers = process_map(&mut lines, soils);
    // println!("fertilizers: {:?}", fertilizers);

    // "fertilizer-to-water"
    lines.next();
    // Advance the map label
    let waters = process_map(&mut lines, fertilizers);
    // println!("waters: {:?}", waters);

    // "water-to-light"
    lines.next();
    // Advance the map label
    let lights = process_map(&mut lines, waters);
    // println!("lights: {:?}", lights);

    // "light-to-temperature"
    lines.next();
    // Advance the map label
    let temperatures = process_map(&mut lines, lights);
    // println!("temperatures: {:?}", temperatures);

    // "temperature-to-humidity"
    lines.next();
    // Advance the map label
    let humidities = process_map(&mut lines, temperatures);
    // println!("humidities: {:?}", humidities);

    // "humidity-to-location"
    lines.next();
    // Advance the map label
    let locations = process_map(&mut lines, humidities);
    // println!("locations: {:?}", locations);

    let p1 = *locations.iter().min().unwrap();
    let p2 = 0;

    (p1, p2)
}

/**
 * Expects a Lines iterator starting at the start of the map data lines, and leaves it after consuming the newline.
 */
fn process_map(data_lines: &mut Lines<'_>, mut input: HashSet<usize>) -> HashSet<usize> {
    let mut output = HashSet::with_capacity(input.len());

    for line in data_lines {
        if line.trim().is_empty() {
            // Newline that marks end of category
            break;
        }

        let mut numbers = line
            .split_ascii_whitespace()
            .map(|num| num.parse::<usize>().unwrap());
        let dest_start = numbers.next().unwrap();
        let source_start = numbers.next().unwrap();
        let range_size = numbers.next().unwrap();

        let source_range = source_start..source_start + range_size;

        let contained_items = input.extract_if(|item| source_range.contains(item));
        for contained_item in contained_items {
            output.insert(contained_item - source_start + dest_start);
        }
    }
    output.extend(input);
    output
}

#[test]
fn example_input() {
    let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
        .to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 35);
    assert_eq!(res.1, 46);
}

#[test]
fn prod_solution() {
    use std::fs::read_to_string;

    let input = read_to_string(format!("inputs/{}", "5.in")).unwrap();
    let res = solve_day(input);
    assert_eq!(res.0, 346433842);
    // assert_eq!(res.1, 5704953);
}

aoc2023::day_main!("5.in");
