#![feature(test)]

use std::collections::HashMap;
extern crate test;

fn solve_day(input: String) -> (usize, usize) {
    let mut curr_dir = vec![""];
    let mut sizes: HashMap<String, usize> = HashMap::new();

    for line in input.lines() {
        if line == "$ cd /" {
            // Only happens at the start, so we don't even have to handle it.
            continue;
        }
        if line == "$ cd .." {
            curr_dir.pop();
            continue;
        }
        if line.starts_with("$ cd ") {
            let (_, new_dir) = line.split_at(5);
            curr_dir.push(new_dir);
            continue;
        }
        if line == "$ ls" {
            // We don't need to prepare anything to get data, so we just skip the line.
            continue;
        }

        // All other alternatives covered, these lines are data from files or directories.
        let (size_or_dir, name) = line.split_once(' ').unwrap();
        if size_or_dir == "dir" {
            // We learn that 'name' is a child of 'curr_dir'
            continue;
        }

        let size = size_or_dir.parse::<usize>().unwrap();
        // We must add 'size' to the size of the current directory and all of its parents.
        let mut dir = curr_dir.clone();
        while !dir.is_empty() {
            let dir_str = if dir.len() == 1 {
                // Just to make it prettier for printing, actually not necessary.
                "/".to_string()
            } else {
                dir.join("/")
            };
            // println!("Adding {} to {}", name, dir_str);
            sizes
                .entry(dir_str)
                .and_modify(|dir_size| *dir_size += size)
                .or_insert(size);
            dir.pop();
        }
    }

    // println!("{:?}", sizes);

    let p1 = sizes
        .iter()
        .filter_map(|(_path, &size)| if size <= 100000 { Some(size) } else { None })
        .sum();

    const TOTAL_FS_SPACE: usize = 70_000_000;
    const UPDATE_SPACE: usize = 30_000_000;
    let root_size = sizes.get("/").unwrap();
    // unused space is the total - the occupied space
    let unused_space = TOTAL_FS_SPACE - root_size;
    // We need to free enough space to run the update, other than what is already free
    let space_to_free = UPDATE_SPACE - unused_space;

    // Find the smallest directory that would still free up enough space for the update
    let p2 = sizes
        .iter()
        .filter_map(|(_path, &size)| {
            if size >= space_to_free {
                Some(size)
            } else {
                None
            }
        })
        .min()
        .unwrap();

    (p1, p2)
}

#[test]
fn example_input() {
    let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"
        .to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 95437);
    assert_eq!(res.1, 24933642);
}

#[test]
fn prod_solution() {
    use std::fs::read_to_string;

    let input = read_to_string(format!("inputs/{}", "7.in")).unwrap();
    let res = solve_day(input);
    assert_eq!(res.0, 1989474);
    assert_eq!(res.1, 1111607);
}

aoc2022::day_main!("7.in");
