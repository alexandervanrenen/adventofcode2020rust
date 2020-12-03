My implementation of the [adventofcode 2020](https://adventofcode.com/2020), arguable the best advent calender on the internet :)

The goal is to use these little exercises to practice my pun and *Rust* skills, which are quite rusty at the moment.
I will keep a log of some thoughts on each exercise, about what I did and (mainly) learned.

Day 01
------
*Bin packing with three items => brute force.*

Just learning Rust using a nice [c++ to rust](https://github.com/nrc/r4cppp) tutorial.
Only at 4., but you don't need to understand memory or pointers to write a simple program.
Algorithmically, in this specialized form of bin packing we can always drop the last for loop and replace the lookup with a hash table.
Thus reducing the time complexity from ``O(n^x)`` to ``O(n^(x-1))`` with ``n`` being the number of inputs and ``x`` the number of inputs to use.

Day 02
------
*Parsing exercise*

Implemented parsing manually to get better in Rust .. still reading the tutorial :)
