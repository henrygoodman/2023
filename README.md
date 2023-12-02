### Advent of Code 2023 (Rust)

Day1:
- String splicing and replacements, parsing chars to ints

Day2:
- Learnt the value of `unwrap()` for AOC problems. Usually it's not the best idea to unwrap since it can make the program panic if not handling the errors, however it is necessary for AOC as it requires a lot of parsing, and we don't particularly care about panicking, just getting an output.
- Saw some solutions that used some functional programming methods like filter/map to iterate over each line to determine if a gameset is impossible, and then map the game number to the output vector if possible, and none if not. This allows calling `sum()` on the resulting vector to easily calculate the sum of game ID's that are possible. This is not as applicable to part 2, as it just involves a simple iteration to determine the max value of each cube type in a set of games.