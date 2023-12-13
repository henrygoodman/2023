// Processes a grid and returns the reflection index score
// For a proper mirror, we need to mirror until we hit the edge of the grid
fn process1(grid: &Vec<Vec<char>>) -> (i64, bool) {
    // Look for horizontal mirror
    for (i, line) in grid.iter().enumerate().skip(1) {
        let mut mirror_index: i32 = -1;

        // Potential mirror found
        if line == &grid[i-1] {
            mirror_index = i as i32 -1;
        }

        // Explore potential mirror index
        if mirror_index != -1 {
            let mut prev: i32 = i as i32 -1;
            let mut next: i32 = i as i32;
            while prev >= 0 && next < grid.len() as i32 {
                // Stop checking if the rows are not equal
                for (ch1, ch2) in grid[next as usize].iter().zip(&grid[prev as usize]) {
                    if ch1 != ch2 {
                        mirror_index = -1;
                        break;
                    }
                }
                prev -= 1;
                next += 1;
            }
            // If we have exhausted the loop, all rows upto the edges are mirrored.
            // Return the index if we have found a successful match
            if mirror_index != -1 {
                return (i as i64, true);
            }
        }
    }
    // Return 0 if no mirrors found
    (0, false)
}

fn process2(grid: &Vec<Vec<char>>) -> (i64, bool) {
    let mut mirror_index: i32 = -1;

    // Look for horizontal mirror
    for (i, line) in grid.iter().enumerate().skip(1) {
        let mut smudge_fixed: bool = false;
        let mut diff_count = 0;
        let prev_line = &grid[i-1];

        for (ch1, ch2) in line.iter().zip(prev_line) {
            if ch1 != ch2 {
                diff_count += 1;
                if diff_count > 1 {
                    break;
                }
            }
        }

        // Potential mirror found
        if diff_count <= 1 {
            mirror_index = i as i32 - 1;
        }

        // Explore potential mirror index
        if mirror_index != -1 {
            let mut prev: i32 = i as i32 -1;
            let mut next: i32 = i as i32;

            while prev >= 0 && next < grid.len() as i32 {

                if grid[next as usize] != grid[prev as usize] {
                    // Handle if current rows are unequal but we have already fixed a smudge
                    if smudge_fixed {
                        mirror_index = -1;
                        break;
                    }
                    // Handle checking row when smudge has not yet been fixed
                    else {
                        let mut diff_count2 = 0;
                        for (ch1, ch2) in grid[next as usize].iter().zip(&grid[prev as usize]) {
                            if ch1 != ch2 {
                                diff_count2 += 1;
                                if diff_count2 > 1 {
                                    break;
                                }
                            }
                        }
                        // Indicates rows can be mirrored by fixing a smudge
                        if diff_count2 == 1 {
                            smudge_fixed = true;
                        }
                         // Row differs too much to be fixable
                        else if diff_count2 > 1 {
                            mirror_index = -1;
                            break;
                        }
                    }
                }
                // If nothing runs in this loop, just means lines are equal
                prev -= 1;
                next += 1;
            }
            // Handle where we exit with a mirror, only return the index if the mirror fixed a smudge, else continue
            if smudge_fixed && mirror_index != -1 {
                return (i as i64, true);
            }
            // If no smudge fixed, reset the mirror index
            else if mirror_index != -1 {
                mirror_index = -1;
            }
        }
    }
    // Return 0 if no mirrors found
    (0, false)
}

// Transposes a matrix to make it easier to do comparisons between rows
fn transpose(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = grid.len();
    let cols = if rows == 0 { 0 } else { grid[0].len() };

    let mut transposed = vec![vec![' '; rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            transposed[j][i] = grid[i][j];
        }
    }
    transposed
}

// Assume there is only ever 1 line of reflection
// Find if the line is horizontal or vertical
// Find the index of the line by checking the lines surrounding it are also mirrored
pub fn solve1(input: Vec<String>) -> i64 {
    let mut ret: i64 = 0;
    let mut current_grid = Vec::new();

    for line in input {
        if line.trim().is_empty() {
            // When an empty line is encountered, push the current grid to the result and start a new grid
            if !current_grid.is_empty() {
                let (idx, found) = process1(&current_grid);
                ret += 100 * idx;
                if !found {
                    current_grid = transpose(&current_grid);
                    let (jdx, _) = process1(&current_grid);
                    ret += jdx;
                }
                current_grid = Vec::new();
            }
        } else {
            // Convert the line into a vector of characters and add it to the current grid
            let row = line.chars().collect::<Vec<char>>();
            current_grid.push(row);
        }
    }

    ret
}

// We need to find 1 new line of reflection (either horizontal or vertical)
// i.e. we cannot simply return on perfectly mirrored.
// - the question sounds like we should only ever be able to find 1 smudge that gives a new line
// - if this is the case, then we should only ever find a smudge in 1 location (so it will show up in the same spot)
pub fn solve2(input: Vec<String>) -> i64 {
    let mut ret: i64 = 0;
    let mut current_grid = Vec::new();

    for line in input {
        if line.trim().is_empty() {
            // When an empty line is encountered, push the current grid to the result and start a new grid
            if !current_grid.is_empty() {
                let (idx, found) = process2(&current_grid);
                ret += 100 * idx;
                if !found {
                    current_grid = transpose(&current_grid);
                    let (jdx, _) = process2(&current_grid);
                    ret += jdx;
                }
                current_grid = Vec::new();
            }
        } else {
            // Convert the line into a vector of characters and add it to the current grid
            let row = line.chars().collect::<Vec<char>>();
            current_grid.push(row);
        }
    }
    ret
}
