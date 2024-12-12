#![feature(test)]
extern crate test;

#[derive(Debug, Clone)]
enum DiskItem {
    Free(usize),
    File { id: usize, size: usize },
}

fn solve_day(input: String) -> (usize, usize) {
    let mut disk = vec![];

    // trim_end because the input file has an ending newline which was crashing the subtraction.
    for (i, c) in input.trim_end().bytes().enumerate() {
        // All chars are just 0-9 so let's just simplify this.
        let item_val = c - b'0';
        let item_val = item_val.into();

        if item_val == 0 {
            // We are compacting anyway! So ignore anything with size 0 ;)
            continue;
        }

        if i % 2 == 0 {
            disk.push(DiskItem::File {
                id: i / 2,
                size: item_val,
            });
        } else {
            disk.push(DiskItem::Free(item_val));
        }
    }

    let mut p2_disk = disk.clone();

    compact_disk_fragmented(&mut disk);
    let p1 = checksum_empty_early_termination(&disk);

    compact_disk_cool(&mut p2_disk);
    // println!("{:?}", p2_disk);
    let p2 = checksum_full(&p2_disk);

    (p1, p2)
}

fn compact_disk_fragmented(disk: &mut Vec<DiskItem>) {
    let mut left_idx = 0;
    let mut right_idx = disk.len() - 1;

    // We will try to meet in the middle to know that we're done.
    // If we go past it, we're also done (probably just safety).
    // (This also covers the case where there is no free space, since right_idx starts as disk len)
    while left_idx < right_idx {
        // TODO: Consider the case where there is free space at the end of the file. Input and
        // example don't so let's ignore it for now.
        // Maybe related: Same but for start? However, this would work no problem.

        let left_item = &disk[left_idx];

        // If we are not over free space on our left side, move up to it.
        if !matches!(left_item, DiskItem::Free(_)) {
            left_idx += 1;
            continue;
        }

        let right_item = &disk[right_idx];

        // Left side is now free space, right should be a file.
        if !matches!(right_item, DiskItem::File { id: _, size: _ }) {
            right_idx -= 1;
            continue;
        }

        // Left side is free, right side is a file, so just copy over, counting the sizes.
        // We should move the smallest amount between the free space and file size.

        // Match is just to correctly unwrap the types... There's probably a more elegant solution
        // since we have guarantees above.
        // I'm sorry for the clone. TODO: Remove clone? Didn't make sense to fight it for now
        match (left_item.clone(), right_item.clone()) {
            (DiskItem::Free(free_size), DiskItem::File { id, size }) => {
                let shift_amount = free_size.min(size);
                // println!(
                //     "at {},{}: ({:?},{:?}) - spending {}",
                //     left_idx, right_idx, left_item, right_item, shift_amount
                // );

                let copied_file_maybe_part = DiskItem::File {
                    id,
                    size: shift_amount,
                };

                // We should always add the new field over where the free space was, unless the
                // free space still exists.
                if shift_amount == free_size {
                    // println!("left free fully spent");
                    // We have fully consumed the free space, replace it with the new file
                    disk[left_idx] = copied_file_maybe_part;
                    // We don't need to move our cursor since the next iteration will do that for
                    // us.
                } else {
                    // println!(
                    //     "left free partially spent, remainder:{}",
                    //     free_size - shift_amount
                    // );
                    // The free space was only partially exhausted.
                    // We need to add a new file to the left of the free space, and decrement it
                    // accordingly.
                    // We decrement first, to do it before the shift
                    let new_free = DiskItem::Free(free_size - shift_amount);
                    disk[left_idx] = new_free;

                    disk.insert(left_idx, copied_file_maybe_part);
                    // after .insert, we now have: [new_item, remaining_free_space]
                    // Let's save one iteration by putting our cursor back where it was:
                    left_idx += 1;
                    // Since we shifted all items, put the right index where it was:
                    right_idx += 1;
                }

                // Regardless of what happened on the left, we have to also decrement the right
                // side.
                if shift_amount == size {
                    // println!("full copy");
                    // File totally exhausted, mark it as free and continue
                    // This is necessary for the later checksum calculation, otherwise we'd have
                    // too many values.
                    // I think the value shouldn't matter, though. But just in case (maybe prep
                    // part2?) we do what we should.
                    disk[right_idx] = DiskItem::Free(shift_amount);
                    // Save one iteration by shifting
                    right_idx -= 1;
                } else {
                    // println!("partial copy: {}", size - shift_amount);
                    // If we only partially spent the file, "spend" it
                    disk[right_idx] = DiskItem::File {
                        id,
                        size: size - shift_amount,
                    };
                }
            }
            _ => unreachable!(),
        }
    }

    // TODO: (if you do the stuff below, shift the while loop by -1 on right side)
    // Handling specifically the case where we end iteration with the last file adjacent to an
    // empty space. It should just be copied over fully, without breaking it into parts.
}

// Mostly copied over from previous one
fn compact_disk_cool(disk: &mut Vec<DiskItem>) {
    let mut left_idx = 0;
    // Reducing this is actually just an optimization. We could always scan from the end until the
    // left_idx, it would just mean a lot of iterating over empty space. We can later try it
    // without the optimization to see how it's affected.
    let mut right_bound = disk.len() - 1;

    // The termination can stay the same now, but instead of just copying whatever is on the right
    // side, we have to attempt to copy a full file found in [right_idx, left_idx[.
    // If that fails, just move on and ignore that free space.
    // (actually renaming right_idx to right_bound as it makes more sense here)
    while left_idx < right_bound {
        let left_item = &disk[left_idx];

        // If we are not over free space on our left side, move up to it.
        if !matches!(left_item, DiskItem::Free(_)) {
            left_idx += 1;
            continue;
        }

        println!("{} {:?}", left_idx, left_item);
        println!("{:?}", disk);

        // Condense "fragmented" free space that we might have due to previous file moves:
        if left_idx < disk.len() - 2 {
            println!("Trying to condense at {} {}", left_idx, left_idx + 1);
            // If we have contiguous free space, merge it.
            if let (DiskItem::Free(free1), DiskItem::Free(free2)) =
                (&disk[left_idx], &disk[left_idx + 1])
            {
                println!(
                    "Merging {}:{:?}, {}:{:?}",
                    left_idx,
                    disk[left_idx],
                    left_idx + 1,
                    disk[left_idx + 1]
                );

                // Merge all the free space into the first element, and remove the second one
                disk[left_idx] = DiskItem::Free(free1 + free2);
                disk.remove(left_idx + 1);
                // Since we removed an item, right_bound needs to be lowered:
                right_bound -= 1;
            }
        }

        // Let's try all files from right_bound until left_idx+1, stopping if we get a transfer.
        // Optimization: If we transfer the first file that we found, all space to the right is
        // free and we no longer need to search there, reducing right_bound and our search space.        let mut encountered_free = false;
        if let Some(WholeFileCopy {
            copied_file_original_idx,
            pending_files_to_the_right,
        }) = try_copy_whole_file(disk, left_idx, right_bound)
        {
            // We managed to copy a whole file!
            // If this was the first file we encountered when searching from the right, we no
            // longer need to search to the right of that file!
            // if !pending_files_to_the_right {
            //     // Since the copied file's original index is now a Free space, we can even save 1
            //     // more step by shifting it by 1. (FIXME: Potentially dangerous, check here first for
            //     // bugs)
            //     right_bound = copied_file_original_idx - 1;
            // }
        }
        // Regardless of success or failure, we move along since we already tried all options.
        left_idx += 1;
    }
}

struct WholeFileCopy {
    copied_file_original_idx: usize,
    pending_files_to_the_right: bool,
}

// Moved this looping to a separate function because of E0571: You can't break with a value in a
// for loop.
fn try_copy_whole_file(
    disk: &mut Vec<DiskItem>,
    left_idx: usize,
    right_bound: usize,
) -> Option<WholeFileCopy> {
    let left_item = &disk[left_idx];
    let mut pending_files_to_the_right = false;

    // Comment on below: Actually this was not a bug. As the unoptimized solution got the same
    // answer.
    // The real bug is: If we create a bunch of smaller free-spaces due to moving stuff around,
    // we need to merge them together. So that Free(1) Free(1) Free(1) wouldn't fit 3, while it
    // actually should.
    //
    // /// IMPORTANT: Figured out the bug!
    // /// We should go in order of decreasing file ID number. With specific setups, we might get
    // /// the wrong one if we just always grab the last one, but the example input does not cover
    // /// for this.
    let mut highest_file_id_and_index: Option<(usize, usize)> = None;

    // Since Rust's ranges must be increasing, we create it like so and reverse it to get the
    // desired effect of going from right bound down to left_idx.
    // Since the lower bound is inclusive and upper is exclusive, we need to hack it a bit, sadly,
    // to shift correctly.
    for right_idx in (left_idx + 1..=right_bound).rev() {
        // Skip until a file is found
        let right_item = &disk[right_idx];
        if !matches!(right_item, DiskItem::File { id: _, size: _ }) {
            continue;
        }

        // // println!("Trying {}->{:?} {:?}", left_idx, left_item, right_item);

        // We are now at a file, mark it for copying *only if it fits*
        match (left_item, right_item) {
            (DiskItem::Free(free_size), DiskItem::File { id, size }) => {
                if size > free_size {
                    // File is too big, skip it
                    // Mark that there is a pending file to the right, so it's not ignored in
                    // the future.
                    pending_files_to_the_right = true;
                    continue;
                }

                // The file fits!
                // Check if it's the highest, setting if so!
                if let Some((highest_file_id, _index)) = highest_file_id_and_index {
                    if *id > highest_file_id {
                        highest_file_id_and_index = Some((*id, right_idx));
                    }
                } else {
                    highest_file_id_and_index = Some((*id, right_idx));
                }
            }
            _ => unreachable!(),
        }
    }

    // We have now found a file which fits, or we return None as there was no move for this free
    // space:
    let (_file_id_to_move, file_index_to_move) = highest_file_id_and_index?;

    let right_item = &disk[file_index_to_move];

    match (left_item.clone(), right_item.clone()) {
        (DiskItem::Free(free_size), DiskItem::File { id: _, size }) => {
            // Thus, move the file into place.
            disk.swap(file_index_to_move, left_idx);

            if free_size > size {
                // There is still free space remaining.
                // Correctly update the size of the right side's "newly freed space" as it may
                // be between files and affect future compacting.
                disk[file_index_to_move] = DiskItem::Free(size);
                // Also, add a new smaller Free, with its new size being the leftover free space.
                // Insert the new free space in left_idx+1, as left_idx is where the newly
                // moved file is.
                disk.insert(left_idx + 1, DiskItem::Free(free_size - size));

                // Due to the indexes shift, we have to return the idx + 1
                return Some(WholeFileCopy {
                    copied_file_original_idx: file_index_to_move + 1,
                    pending_files_to_the_right,
                });
            }

            // Perfect fit with no index shift!
            Some(WholeFileCopy {
                copied_file_original_idx: file_index_to_move,
                pending_files_to_the_right,
            })
        }
        _ => unreachable!(),
    }
}

fn checksum_full(disk: &[DiskItem]) -> usize {
    // since we are not actually using full indexes, we need these because of criss-crossed files.
    let mut real_disk_idx = 0;
    let mut acc = 0;

    for item in disk.iter() {
        match item {
            DiskItem::Free(free_size) => {
                // Since now we might have free space between files, this optimization no longer
                // applies.
                // Also, we need to count the indexes since free space is still counted as indexes.
                real_disk_idx += free_size;
                continue;
            }
            DiskItem::File { id, size } => {
                acc += (real_disk_idx..real_disk_idx + size)
                    .map(|idx| idx * id)
                    .sum::<usize>();
                real_disk_idx += size;
            }
        }
    }

    acc
}

fn checksum_empty_early_termination(disk: &[DiskItem]) -> usize {
    // since we are not actually using full indexes, we need these because of criss-crossed files.
    let mut real_disk_idx = 0;
    let mut acc = 0;

    for item in disk.iter() {
        match item {
            DiskItem::Free(_) => {
                // Found the trailing freespace, no need to continue
                break;
            }
            DiskItem::File { id, size } => {
                acc += (real_disk_idx..real_disk_idx + size)
                    .map(|idx| idx * id)
                    .sum::<usize>();
                real_disk_idx += size;
            }
        }
    }

    acc
}

#[test]
fn example_input() {
    let input = "2333133121414131402".to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 1928);
    assert_eq!(res.1, 2858);
}

#[test]
fn prod_solution() {
    use std::fs::read_to_string;

    let input = read_to_string(format!("inputs/{}", "9.in")).unwrap();
    let res = solve_day(input);
    assert_eq!(res.0, 6432869891895);
    // 6557710953246 is too high!!
    // With the unoptimized version I got 6472326329261 which is still also too high.
    // 6538975976890
    // 6472326329261
    // 6462112749117 after "fixing" condensing the empty space
    assert_eq!(res.1, 42);
}

aoc2024::day_main!("9.in");
