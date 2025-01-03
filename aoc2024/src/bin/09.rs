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

        let DiskItem::Free(free_size) = disk[left_idx] else {
            // If we are not over free space on our left side, move up to it.
            left_idx += 1;
            continue;
        };

        // Left side is now free space, right should be a file.
        let DiskItem::File { id, size } = disk[right_idx] else {
            right_idx -= 1;
            continue;
        };

        // Left side is free space, right side is a file
        // So, just copy over, keeping in mind the different sizes.

        // This used to be a bunch of if/elseif/else but clippy::comparison_chain.
        match size.cmp(&free_size) {
            std::cmp::Ordering::Equal => {
                // File fits perfectly in the free space, just swap the items around
                disk.swap(left_idx, right_idx);
                // Save one iteration since right is now done and left is used.
                right_idx -= 1;
                left_idx += 1;
            }
            std::cmp::Ordering::Greater => {
                // Only part of the file was moved, as it did not totally fit in the free space.
                // Move over the partly copied file
                disk[left_idx] = DiskItem::File {
                    id,
                    size: free_size,
                };
                // Deplete the copied contents on the right side.
                disk[right_idx] = DiskItem::File {
                    id,
                    size: size - free_size,
                };
                // Since left_idx is now over a file, let's save one iteration.
                left_idx += 1;
                // We don't move right_idx as we still want to finish moving that file, if we can.
            }
            std::cmp::Ordering::Less => {
                // The last option is that we had leftover free space.
                // In that case, we can still just swap items around since we did a complete copy of
                // the file, and then add the partially depleted free item as our next item to check.
                // (And also the free space being wrong on the right side doesn't matter for p1 since
                // we compact piece by piece).
                disk.swap(left_idx, right_idx);
                disk.insert(left_idx + 1, DiskItem::Free(free_size - size));
                // Since we inserted an item, we don't need to shift right_idx downwards.
                // We can save an iteration by moving to the new free item, though:
                left_idx += 1;
            }
        }
    }

    // TODO: (if you do the stuff below, shift the while loop by -1 on right side)
    // Handling specifically the case where we end iteration with the last file adjacent to an
    // empty space. It should just be copied over fully, without breaking it into parts.
}

// Mostly copied over from previous one
fn compact_disk_cool(disk: &mut Vec<DiskItem>) {
    // Previous implementation for day 2 had a massive misunderstanding, and I think that just a
    // bunch of bugs piled together and made the initial buggy approach work for the example input
    // lmao.
    //
    // We don't want to move the highest file that we can fit to the leftmost space.
    // We, instead, just want to try moving the highest file to the leftmost space that it can fit,
    // only trying once.
    // This is shown by the fact that "2" only moves on the last step in the example in the
    // website.
    //
    // To do this, since file IDs increase from left to right and we always move over just one
    // item, we just need to walk from right to left, and insert it into the leftmost space.
    // As an optimization, we can have a left_bound that we increase as the empty space gets filled
    // up, so that we don't need to always search through everything.
    //
    // TODO: we might need to handle the case in which somehow this right to left iteration means
    // that we touch the same file twice? I tried to think of when this could happen but I can't
    // see it, as the rightmost file should generally be the highest one. If a file with larger ID
    // could be moved further left, it would have already been moved there, unless we are iterating
    // left to right, since we put it in the leftmost spot that it could go...

    let mut left_bound = 0;

    let mut right_bound = disk.len() - 1;

    while left_bound < right_bound {
        // Get to the rightmost file
        // If we are not over a file, keep searching
        let DiskItem::File { id: _, size: _ } = disk[right_bound] else {
            right_bound -= 1;
            continue;
        };

        if let Some(MovedToLeftmostFreeSpace {
            move_destination_idx,
            only_saw_files,
        }) = try_move_to_leftmost_free_space(disk, left_bound, &mut right_bound)
        {
            // Interesting note: some initial benchmarking suggested that this caused a roughly 50%
            // speedup - from 60ms to 40ms. I guess that there is a lot of repeated iteration,
            // which hints at the necessary optimization to bring the runtime of this a bit lower:
            // Somehow caching results or indexing information in a different way, like hashmaps of
            // free space, or something of the sort, didn't think about it too much yet.
            if only_saw_files {
                // From the current value of left_bound until where we copied the file to, we only
                // saw other files (we already filled in the free space we had found).
                // As such, the disk is already compacted from the start, so we don't need to
                // search there anymore.
                // +1 since move_destination_idx is where we moved to, and it's a file, and we want
                // to find `Free`s.
                left_bound = move_destination_idx + 1;
            }
        }

        // Regardless of if the move suceeded or not, we continue looking at the next item, from
        // right to left.
        right_bound -= 1;
    }
}

struct MovedToLeftmostFreeSpace {
    move_destination_idx: usize,
    only_saw_files: bool,
}

fn try_move_to_leftmost_free_space(
    disk: &mut Vec<DiskItem>,
    left_bound: usize,
    right_item_idx: &mut usize,
) -> Option<MovedToLeftmostFreeSpace> {
    // Optimization: If everything to the left of this swap was just files, we don't need to
    // search there anymore.
    let mut only_saw_files = true;

    // Unwrap the right item - we checked this in the caller.
    let DiskItem::File { id: _, size } = disk[*right_item_idx] else {
        unreachable!();
    };

    // We are over a file with our right item idx, so now we search up the first empty space that
    // the file can fit in.
    for left_idx in left_bound..*right_item_idx {
        let DiskItem::Free(free_space) = disk[left_idx] else {
            continue;
        };

        if free_space < size {
            // We have a free space that we did not use and might potentially be useful later.
            only_saw_files = false;
            continue;
        }

        // The current file fits in this space.
        // So, copy it around.
        disk.swap(left_idx, *right_item_idx);

        // Check for partial space being "spent"
        if size < free_space {
            // Since there is still free space remaining, we should create a new free space
            // with the resulting remaining free space, as well as update the new free space
            // that we left on the right side with the correct size, as it may affect checksum
            // calculation.

            disk[*right_item_idx] = DiskItem::Free(size);
            disk.insert(left_idx + 1, DiskItem::Free(free_space - size));
            // Due to the insert our right bound actually means something else, so fix that
            *right_item_idx += 1;
        }

        // We managed to copy a file around.
        return Some(MovedToLeftmostFreeSpace {
            move_destination_idx: left_idx,
            only_saw_files,
        });
    }

    None
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
    // 6467290479134 after actually reading the problem statement lol.
    assert_eq!(res.1, 6467290479134);
}

aoc2024::day_main!("9.in");
