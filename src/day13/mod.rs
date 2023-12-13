// Processes a grid and returns the reflection index score
// For a proper mirror, we need to mirror until we hit the edge of the grid
fn process1(grid: &Vec<Vec<char>>) -> i64 {
    println!();
    for (i, line) in grid.iter().enumerate() {
        println!("{:>2} {:?}", i, line);
    }
    // Look for horizontal mirror
    for (i, line) in grid.iter().enumerate().skip(1) {
        let mut mirror_index: i32 = -1;
        if i > 0 && line == &grid[i-1] {
            println!("Potential horizontal mirror index found: {:?}", i-1);
            mirror_index = i as i32 -1;
        }

        // Explore potential mirror index
        if mirror_index != -1 {
            let mut prev: i32 = i as i32 -1;
            let mut next: i32 = i as i32;
            while prev >= 0 && next < grid.len() as i32 {
                println!("{} {}", prev, next);
                if grid[next as usize] != grid[prev as usize] {
                    println!("Index not mirrored: {} {}", prev, next);
                    mirror_index = -1;
                    break;
                }
                prev -= 1;
                next += 1;
            }
            if mirror_index != -1 {
                println!("Mirror found {}", i);
                return i as i64;
            }
        }
    }
    0
}

fn process2(grid: &Vec<Vec<char>>) -> i64 {
    let mut smudge_fixed: bool = false;
    let mut mirror_index: i32 = -1;
    for (i, line) in grid.iter().enumerate() {
        println!("{:>2} {:?}", i, line);
    }
    // Look for horizontal mirror
    for (i, line) in grid.iter().enumerate().skip(1) {
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

        if diff_count <= 1 {
            println!("Potential mirror index found: {}", i-1);
            mirror_index = i as i32 - 1;
            if diff_count == 1 { smudge_fixed = true };
        }

        if smudge_fixed && mirror_index == 0 || mirror_index == grid.len() as i32 - 1 {
            println!("Mirror found {}", i);
            return i as i64;
        }

        // Explore potential mirror index
        if mirror_index != -1 {
            let mut prev: i32 = i as i32 -1;
            let mut next: i32 = i as i32;
            while prev >= 0 && next < grid.len() as i32 {
                println!("{} {}", prev, next);

                if smudge_fixed && grid[next as usize] != grid[prev as usize] {
                    println!("Index not mirrored: {} {}", prev, next);
                    mirror_index = -1;
                    break;
                }
                else if !smudge_fixed {
                    println!("Analysing row where smudge hasnt been fixed yet");
                    for (ch1, ch2) in grid[next as usize].iter().zip(&grid[prev as usize]) {
                        if ch1 != ch2 {
                            diff_count += 1;
                            if diff_count > 1 {
                                break;
                            }
                        }
                    }
                    if diff_count == 1 {
                        smudge_fixed = true;
                        println!("Index can be mirrored by fixing smudge: {} {}", prev, next);
                    } else if diff_count > 1 {
                        println!("Index not mirrored: {} {}", prev, next);
                        mirror_index = -1;
                        break;
                    }
                }
                prev -= 1;
                next += 1;
            }
            if smudge_fixed && mirror_index != -1 {
                println!("Mirror found {}", i);
                return i as i64;
            }
        }
    }
    0
}

fn transpose(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    println!("Transposing");
    let rows = grid.len();
    let cols = if rows == 0 { 0 } else { grid[0].len() };

    let mut transposed = vec![vec![' '; rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            transposed[j][i] = grid[i][j];
        }
    }
    println!("Transposing done");
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
                ret += 100 * process1(&current_grid);
                current_grid = transpose(&current_grid);
                ret += process1(&current_grid);
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
                ret += 100 * process2(&current_grid);
                current_grid = transpose(&current_grid);
                ret += process2(&current_grid);
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
