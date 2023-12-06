### Advent of Code 2023 (Rust)

Day1:
- String splicing and replacements, parsing chars to ints

Day2:
- Learnt the value of `unwrap()` for AOC problems. Usually it's not the best idea to unwrap since it can make the program panic if not handling the errors, however it is necessary for AOC as it requires a lot of parsing, and we don't particularly care about panicking, just getting an output.
- Saw some solutions that used some functional programming methods like filter/map to iterate over each line to determine if a gameset is impossible, and then map the game number to the output vector if possible, and none if not. This allows calling `sum()` on the resulting vector to easily calculate the sum of game ID's that are possible. This is not as applicable to part 2, as it just involves a simple iteration to determine the max value of each cube type in a set of games.

Day3:
- Used HashMap to track gear locations and if they have been `seen` i.e. a binary association with another gear
- Parsing into 2d string array
- Using a lot of booleans to store state, probably need to think of a better solution as this is difficult to debug (i.e during pass-through, determine if we are currently within a number, and if we have seen a symbol adjacent to any of the digits within the number)

Day4:
- Used HashSet to calculate set intersection to find number of matches between 2 sets
- Initialized vec of fixed size with default values (this day was pretty straightforward)

Day5:
- Very tough, brute force approach results in initializing huge arrays that blow up memory.
- A slightly better solution is to compute the min of a subset of the ranges using smaller fixes sized arrays. This is still slow, but processable with smaller arrays as it all reduces down to the local min.
- Used rayon to parallelize the computation of each seed range, however this is still bottlenecked by the memory constraint (as we are now just processing arr.len()/2 chunks of memory at once, which requires minimuzing the max size of each range). This approach took 6m 40s to compute and 10GB of RAM.
- An optimal approach (not implemented) would involve some smart logic with the ranges, calculate overlaps, applying mapping rules to entire ranges etc.

Day6:
- Pretty straightforward, part1 uses a simple formula total distance = speed * (total time - speed). We can iterate over all possible speeds to get a count of all possible values
- Part2 brute force from part1 would take way too long. Instead, I noticed that the above formula resolves to a quadratic, where everything between the zeroes count as a speed whereby the total distance exceeds the race distance. Using the quadratic formula we can compute the zeros, then calculate the range between the zeros to get the total number of ways.