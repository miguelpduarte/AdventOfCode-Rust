## Day 1

Using Rust now! :D

Part 1 was pretty simple and I'm proud to have done it very _"functional"_-y.

Part 2 taught me about `.windows`, which is not a method for a iterator, so I am trying to use itertools' `tuple_windows`. It's interesting that one can just pattern match on the arguments for the number of items to iterate at a time. Wonder how they do that...

## Day 2

Pretty simple and quick. I started implementing the fold before finishing reading the problem description and thankfully was not shot in the foot due to that, which is nice :D

"Good" (enough) code also made part 2 pretty easy. But it's only day 2 anyway, it gets harder later on so let's see...

Oh! And my solution for 2020's day 2 helped **a LOT**, since simply iterating every line and `.map`ping a `.split(' ').collect::<Vec<_>>()` made it easy to destructure the accumulated `vec` for each line :)  
Now I just have to understand why I need the `.as_slice()`, as well as the dereferencing (`*`) of the `Vec` param (presumably since it's passed as a reference instead of as a copy?).

Oh, and I switched to `rust-analyzer` instead of `rls` since it's supposedly the way to go now (even according to `rls` creator in Reddit), but I am frequently getting the annoying error of `unlinked-file: file not included in module tree`, probably due to using a macro to generate the main function inside the binary "packages"... Since I'm not finding any explanation for this online, maybe I should create an issue in the `rust-analyzer` repo.
