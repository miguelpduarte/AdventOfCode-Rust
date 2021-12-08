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

---

Reimplementing with a list of 10 sets per board to compare performance.

Currently fighting the `Copy` trait needing to be implemented if I want to do something like `let mut x = [HashSet::new(); 10];` and then reassigning `x` to the same value to "reset" it. There is probably a more elegant option for this, like dropping the value by letting it go out of an inner scope or something.  
Anyway, "solving" that by using `Vec<HashSet>` instead, unfortunately (heap instead of stack). This way the `Vec` is moved into the function, which takes ownership of it without needing any copy. This has got me thinking that maybe passing a ref to the array and calling clone would also work hmm.

Runtime of this version is about 1-2ms, as opposed to the 0.4ms of the initial solution.

---

Reimplementing with arrays (fully in the stack) + helper arrays to register the number of already picked numbers for each row and col.

Much faster, 0.2ms average instead of the 0.4ms of the original solution, which is about half of the time.

As someone once said: `"Arrays go brrr"`

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

## Day 6

Naive implementations are fun, but not very efficient. Part 2 made me realise that when my code crashed my PC. (Well kinda, but let's pretend that was the case since that's funnier)

The typical problem of indexing `index -> stage` is solved with much lower memory usage by swapping to `stage -> nr_items`.

There are several solutions, the initial ones being `06.rs` (basically only works for part 1 due to the high memory usage) and `06_smort.rs` that uses a `VecDeque` to shift items around (decrementing states) using `rotate_left` instead of moving the values. `06_smort` score about 45-50-something microseconds on average, which was nice.

The other solutions are:

* `06_smort_arrs` - using native arrays instead of `VecDeque` and instead of moving the items around, just indexing on them with a "shift" integer that wraps around. Very similar in time to the `VecDeque` solution (though slightly faster and more consistent), averaging something like 40-48 microseconds.
* `06_rekicho` - [`@Rekicho`](https://github.com/Rekicho)'s solution, adapted to fit my structure so we could time it in the same basis. Kind of naive by copying the arrays in every iteration, but still ridiculously fast, averaging something like 5.5-6.5 microseconds.
* `06_rekicho_clipped` - The same as `06_rekicho` but after applying `clippy`'s suggestions (removing `.clone` and "replacing the loop by: `values[..(9 - 1)].clone_from_slice(&old_values[1..9]);`"). Seems to be marginally slower, though, which is funny, averaging 7-8 microseconds. Probably due to the assignments being grouped before the change, and now they are split in two parts: that memcpy and the assignment in the lines after. Removing the call to `.clone` had no impact in performance, which makes sense since `[u64]` is `Copy`, which makes it so that `.copy` is used instead of `.clone` AFAIK.
* `06_smort_arrs_better_init` - A reimplementation of my array solution, with faster initialization. Basically used iterators a bit less: `.map`'ed only once instead of 3 with method references, and instead of counting the number of elements of each type to initialize the array, iterated over the dataset, incrementing the respective index. Since the data initialization and access are both quite fast, this makes sense to be quite a bit faster. Average is around 5-6 microseconds.

 Takeaways:

* `.map`'ing several times with method references is much slower than doing it just once with a closure that chains all the operations. I thought this would've been optimized by the compiler, but it seems that this is not the case. This is the part where we are benchmarking input parsing though, which already shows how much we are squeezing lol.
* Turns out that moving the values is quite efficient, since they implement `Copy` and are quite small (only 9 64-bit unsigned values). I think implementing `Copy` enables the usage of `memcpy` which probably justifies the speed. (This was "discovered" by looking at `06_rekicho` and comparing it to the other ones - it had no right being this fast!)

Comparing the different solutions for today was quite fun! I've never felt the pull of this kind of optimization and algorithm tinkering, but AoC+Rust seems to be quite the combo for me!

 I'll probably start using [`criterion`](https://crates.io/crates/criterion) for benchmarking soon, especially if I can fit it nicely in my current macros. This should provide a more consistent base for benchmarking all the things :D

 A mathematical solution using an expression that says how many fish a fish will generate for day `d` is likely possible and quite interesting, as that could even be evaluated in compile time, I guess.

 ----

 On an unrelated note, today I had some trouble with `ALE` + `rust-analyzer` which sometimes seemed to lag behind a bit, showing errors for lines that were there 5-8 edits ago... Sometimes disabling and enabling ALE worked, or restarting vim, but it was still quite strange and a big productivity downgrade.

 ----

 Moving the trim to the initial string and removing it from inside the map reduced the runtime one more microsecond on average lol.

 Also have to try improving the initialization of the `VecDeque` version.

 ----

 Some more experimentation after solving day 7:

 * It checks out that the bottleneck of the `VecDeque` solution (at least initially) was the input parsing. `06_smort_betterinit` now has very similar runtime to its array-based counterpart, averaging something like 6-7 microseconds of runtime. The major changes were joining the parsing map operations as a single chained call; increment the values directly in the `VecDeque` instead of creating an array from "expensive" iterator operations and then initializing using that; and remove an unnecessary `.trim` call that was being ran on the map operation chain instead of only once in the initial string.
 * For the array-based solution: It seems that merging the parsing operations in the initialization loop has a very similar runtime to creating the iterator with the operations and then looping over it. This makes sense given that iterators are lazy-evaluated.
 * Also for the array-based solution: storing the count of the number of fish after initialization, and then incrementing it as days go by seems to be marginally more efficient than running 2 sums (1 for each part). This makes sense, but has a very minimal impact on performance. Nevertheless, it seems to be there, reducing the average from 5-6 us to 4.7-5.3 microseconds. This is a very _finicky_ comparison, though, given the very low run times which are easily impacted by external factors such as system load. (_I should really start using actual benchmarks to compare things..._)

 ## Day 7

Part 1 is simply the median to find the right position + calculate the fuel lol. Have to see if there is any way to make this more efficient, since it's already at 60-70 microseconds. Maybe it's due to the input size, but I doubt it.

For part 2 the median won't work. The average of the example is 4.9, the ideal position being 5. Maybe that works?  
Darn, it worked for the example but not for the provided input... What could be going wrong?

Supposedly this should be the right solution, and I even checked with a website that my calculation of the average is correct so IDK what's up.

It turns out that for the example input, which has an average of 4.9, the right solution is to round up. However, for my input, that has an average of 489.591, the right solution is to round down. Wat?

Well, [`xRuiAlves`](https://github.com/xRuiAlves) helped me by shedding some light onto the issue: in truth, before trying, we cannot know which would be better. This is due to the fact that the average is not a weighted metric, unlike the median! So, updating the code to try both, which probably means going from a `.map.sum` to a `for` loop (which in my experience has been faster anyway, so that should be fine).

---

Onto optimization:
* Having 2 separate `.map.sum`s  seems to be either similar or marginally better than having a single loop and accumulating both floor and ceiling count in two separate mutable accumulators.
* It is more efficient to use `.map.sum` for part 1 than to use a `for` loop and accumulate into a local variable.
* TODO: There is an O(n) way to implement median. Investigate that, it's probably faster :D

Today's implementation averaged about 64-70 microseconds. It seems a bit high, especially compared with yesterday which performed a lot of iterations on a loop. This is just calculating a median and average, which feels like it should be as hard. Maybe I can implement the improved median algorithm and try to merge operations, such as a sum and length calculation? Length might be optimized by the underlying container, but if the sum is joined with other operations, it may help.

