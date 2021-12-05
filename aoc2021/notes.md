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

Oh and 0ms in `--release` once again :D

## Day 3

My initial approach was not pretty and assumed 5 bits.

```rust
let gamma = input
    .lines()
    .fold(vec![(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)], |acc, line| {
	line.chars()
	    .zip(acc.iter())
	    .map(|(chr, (zero, one))| match chr {
		'0' => (zero + 1, *one),
		'1' => (*zero, one + 1),
		_ => panic!("Input had something that was neither 0 nor 1: {}", chr),
	    })
	    .collect()
    })
    .iter()
    .fold(0, |acc, (zero, one)| {
	let new_bit = (one > zero) as u8;
	(acc << 1) + new_bit
});
```

Turns out the problem input is 12 bits, so have to make something that works for all sizes.  
I'm thinking of mapping over each char to return tuples which then are summed in a reduce, and then folded into gamma like before. This avoids having to know the number of bits previously.

Understood that was probably dumb because one can't map "over each char" (that would be iterating "vertically", dummy). So I still have to reduce the lines into some structure representing each bit -> (`n_zeros`, `n_ones`)

Ok so maybe instead of reducing to convert and accumulate at the same time, I can map to tuples and then reduce over it, because `fold` with a None type and then having to convert it does not work since the option does not know the size of the tuple before hand

Thanks Rekicho for taking me off this hole I was digging myself into. We just have to sum each number for each index, since if `sum*2` is larger than the number of lines, the bit is 1, otherwise it is 0.

---

Finally got part 1 after a lot of tomfoolery with `usize::leading_zeros`. I was trying to get the number of "actually used bits" to use bit operations to invert `gamma` to get `omega`. Unfortunately I was getting the wrong number of bits due to how, well, computers and bits actually work! Rude!

So instead I just returned an accumulator to the number of bits used from the last `fold` operation and GG!

Also took a bit longer since it took me a while to notice that while `.fold` returns the accumulator type without issue, `.reduce` returns an Option... And the error was being suppressed, pointing to other issues, which made finding it harder - had to put an inspect afterwards to get an error saying that it was not implemented for `Option<Vec<{integer}>>` which made me look for `reduce`'s docs and understand the problem.

---

Now for part 2...

Well, that wasn't as hard I thought it would be... Mostly because I skipped trying to optimize anything since I notice I most likely wouldn't get away without iterating all of the data twice...  
Furthermore, Rust in release mode still nets a staggering "0ms" performance. :D

In hindsight, maybe something like a `trie` would be relevant for this problem. I've never worked with one so that might be interesting.

The code is kind of ugly though, and would benefit from at least some extraction to other functions. It's late though, and I'm not feeling like spending any more time looking at the same code, so maybe sometime in the future I'll try to clean this a bit, I guess.

## Day 4

Changed the benchmarks to use debug print with `now.elapsed()` since that can print smaller than `0ms`! :D  
Idea stolen from [this repo](https://github.com/wfxr/advent-of-code-2021).

Well, this one was mostly parsing and all. I had a couple of bugs and had to fight the borrow checker for a bit, before simply thinking of using indices to the `Vec`. That was much easier and avoided having to convert references back and forth, by being able to access the item with a mutable reference and change it via the index, and then get an immutable reference via the index again, using `.get`.

For part 1 an annoying issue was in parsing the board, which was fixed by using `.split_whitespace` instead of `.split(' ')`. This is probably due to there being several spaces between some values. A particularly insidious bug in this part was also that my parsing assumed the file had an empty line in the end, to "close" the board creation. It did not, so I just had to copy paste some code to below the loop over the input lines :grin:  
Thankfully it was "easy" to notice this after deriving debug on the used structs, and then printing them in the loop. It became obvious that the boards were fine, but one was missing!

In part 2 I forgot to check if a board had already won. This was easily fixed by adding a `bool` to `Board`. Not the most elegant solution, but it works.

## Day 5

First day that running with `--release` is over 1ms... :(

Might optimize later just due to that. However, large matrix, not sure if there is much to be done.

Part 1 was relatively simple, just had to handle when the movement was "in reverse". In part 2 I can't use the same technique since getting the lowest x and y and going to the highest x and y is not the same in diagonal (for example, `0,2` to `2,0` is not the same as `0,0` to `2,2`, which my code was doing).

Had to create a helper function to handle getting the iterator to go in the right direction, despite `Range` only going in the increasing direction. The helper function was kind of troublesome due to the return type, which I wanted to be an `Iterator`, which is a trait. `Box` helped, and I learned a little about this type.

Sadly runtime is still at 2.6 to 3 milliseconds for both parts. It is still quite fast, but I would like to see if I can get it a bit lower with some sort of optimizations, let's see.

---

Post-initial-implementation: Trying some things to see if they would make the code faster.

* Make the array contiguous and access the values with `j * width + i`: The runtime seems to be marginally better, with an average of maybe 0.1ms less? (all "non-scientific" benchmarks - should really consider using nightly for `cargo bench` or use `Criterion` instead).
* Use a Map instead of Array: This is mostly to learn the syntax since sometimes maps are useful. Figured out the Entry API is pretty awesome. The time went from about 3ms or lower, to 25-28ms though. Oof.
* Use a Map with `usize` as key instead of `(usize, usize)`: Trying to reduce the overhead for the keys. Actually reduces the time a bit, now around 18-20ms. If initial capacity is given via `HashMap::with_capacity`, this value decreases to about 17-19ms.

Probably should try something like `rayon` just to see how fast I can go. The issue is that the way the access to the matrix is done currently is not friendly with parallelism. Maybe with clippy's recommendation to use iterators + take and skip instead of the current for loops over a range it would work. TODO
