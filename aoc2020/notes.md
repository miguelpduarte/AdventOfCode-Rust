## Day 1

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
