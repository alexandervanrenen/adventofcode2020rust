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

Day 03
------
*Using vector exercise*

Implemented my first structs and methods in rust.
Known roughly how to parse and work with iterators makes this a lot smoother :)

Day 04
------
*Text processing*

Tried to embrace streams and might have taken it a bit to far in task 1.
Also installed my first crate for task 2: ``regex``!
Sadly, I ignored memory management a lot.
I think I could have used more ``&String`` instead of always creating new strings (``to_string``).
Also, my handling of ``Option<_>`` needs improvement. 
All in all, I should try focusing on understanding these concepts (``String`` vs ``str`` and lifetime) more in the next task.

Day 05
------
*Binary -> numbers*

Just convert a binary string to a number -> done.
Nice and easy in rust.

Day 06
------
*Counting*

Fun and easy.

Day 07
------
*Simple graph problem*

Tried using Rust memory management to build a bi-directional graph.
It's horrible, annoying and frustrating to no end: super verbose, complicated and unnecessary overhead.
What's the point of making me write ``unwrap`` all the time???
Just let it crash if there is no value and don't bother if there is fft!
... let's hope this gets better with more familiarity with Rust.
However, even with my limited understanding and 100-monkey-coding-style, I never had a failure at runtime.
Not sure if this can be entirely tributed to Rust, because ``unwrap`` would have also caused a failure if I screw up.

Day 08
------
*Starting the interpreter <3*

Rust is dumb.
I am really considering to switch to something else, this just sucks the fun out of coding.
I started coding in a few languages and never got so disillusioned before.
It's like riding a bicycle with training wheels.
Useless and not a lot of fun once you know how to ride a bicycle.
Rust's more explicit ways of doing things (e.g., dereferencing memory) is just more verbose, nothing else.
I was hoping for so much more from a newly designed language...

Day 09
------
*Window functions*

Feels like I am getting more familiar with Rust.
Still feels unnecessary, but knowing how to do things helps lower the frustration.
However, this task was also way less memory management intensive.
Will continue with rust, for now :)

...
---

Day 14
------

I am over my Rust depression.
My solution for today might be a bit over-engineered.
I should pay more attention to the input, but day13 scared me.
Also, I still need to improve rust skills.
I think there are many common code snippets you have to learn for every programming language: i.e., iterate over a vector, mutate values in a map, manipulate a string (in 100 different ways).
In Rust I am still missing many of these and therefore struggle a bit.
In addition, there are probably some snippets I have already figured, but are not in a good Rust style.
I also need to improve these.

Day 15
------
*Number sequence*

Just had to unroll a number sequence till 30M.
I think the best possible algorithm is simply playing the game.

Day 16
------
*Simple optimization problem*

A bit heavy on parsing.
Maybe I should not write a parser, but instead create the input problem directly in code (i.e., manual parsing).
I couldn't figure out a way to iterate over a hash map and change it at the same time in Rust ... o.O
