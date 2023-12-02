# Day 01 Recap

### What I struggled on
- I think my code not all idiomatic of Rust yet. Perhaps I think too much in a declarative kind of way
- I forgot a use case in Part 2: I used `str::find` at first, but if the number / digit is present multiple times, I couldn't get the other occurences. So I then used `str::match_indices`.
- I struggled on `str::matches`: I could find a pattern that regrouped checking if what I was checking was a digit or a digit as plain text

### What I could improve
- Iterating over `NUMBERS` in Part 2 is not the most elegant way of doing it
- Multiple steps to transform part of the string in Part 2 may not be necessary: perhaps there is a more idiomatic way to do it? Like a pipeline and chaining actions?
- The "map" to transform digit and plain text digits
