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
- Probably could have used map/reduce in part 1 (these functions seem useful whenever we perform an iteration and then calculation based on the current iterated values)

Day7:
- Used set logic to determine overall strength of hands based on the unique number of cards (with some differentiation when set length is equal)
- Implemented a custom sorting function and used `sort_by` to sort the hands based on overall strength and then by the relative value of each card
- Part2 pretty straightforward, just needed recognizing the possible changes in strength with a `J` in the hand.

Day8:
- Part 1 was pretty trivial, just added nodes to a HashMap and traversed until reached the end node
- Part 2 came with the realization that I could apply LCM to all paths to get the time at which all paths would repeat and hit 'Z' node.
    - Initially thought I had to consider any loops after reaching the 'Z' node, i.e. may take 60 steps from A->Z, but then loop for 6 steps over and over. Afterwards investigated that the neighbours of all 'Z' nodes are the same as the neighbours of all 'A' nodes, meaning the loops are the exact same as A->Z.
    - Also confirmed that each starting node and each endign node are distinct (i.e. every A node goes to exactly one Z node)

Day9:
- Learnt the `windows` attribute of an iterable, helpful to work on individual slices of a vector (especially here to calculate the difference between 2 elements)

Day10:
- Hardest day so far, had to look up some tips to nudge in the right direction, as I knew in the back of my mind there is some mathematical way to determine inside/outside the loop depending on the parity of the loop segments currently passed on each line on a finite plane.

Day11:
- Not bad, I noticed the trick about expanding the distance early, and figured I didn't have to literally expand it. Followed nicely into p2 which was a 1 line change to consider the bigger expansion.

Day12:
- By far hardest day so far, tried a bunch of solutions (brute-force p1, recursive, DP). Finally got DP solution to work by simplifying a row to a base case and building up character by character (determining the total amount of valid solutions at each state) for each potential block, until we run out of blocks (guaranteed to end a block at end of input).
- Still don't fully understand this solution, some details were inspired a lot by other AOC creators, will try fuly understand over coming days.

Day13:
- Not bad, a straightforward approach that just requires thinking logically and fixing any off-by-one errors. Used a matrix transposition trick to work on columns as rows (as it is easier to iterate rows vs columns)

Day14:
- Tricky, but doable. Trick was in recognizing the potential cycles in the board (which is feasible to reason about since its a finite board with a large amount of iterations).