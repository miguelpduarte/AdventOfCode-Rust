## Day 1

(Note to self: this one uses the new input, the others will probably use the old, at least until migration occurs)

Naive (nested loops lol):

```
Time: 30ms
```

"Smart" using `itertools`:

```
Time: 30ms
```

Anecdotal testing seems to indicate that on average the time is the same for both implementations.  
Despite nested for loops being ugly and in rust kind of annoying to work with (due to being unable to break with a value, or even return from the middle of for loops in functions), the naive approach does in fact avoid using a dependency...

---

Ok forgot to use `--release`.  
New times:
* Naive: 0ms
* Itertools: 1ms

Well, so that's even funnier. The naive solution does not even register a measurable elapsed time. But I'd consider both to be very close, since a ms difference is negligible considering possible errors in measurement I guess (much scientific such wow).

## Day 2

Shenanigans with splitting strings and pattern matching on indexes was actually more fun than expected.

This made it so that I still haven't used regex in rust! I think this is probably faster too, from what I heard from wiki at least.
