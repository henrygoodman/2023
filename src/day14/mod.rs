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

// Transposed such that North is now the LHS
// For any row that has an 'O' before a '#'
// For each 'O', count the number of 'O' to the right of it upto any '#'

pub fn solve1(input: Vec<String>) -> i64 {
    let mut ret: i64 = 0;
    let mut grid: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();
    let orig_len = grid[0].len();
    grid = transpose(&grid);

    for line in &grid {
        let mut start_idx = 0;
        let mut current_pos = 0;
    
        while start_idx < line.len() {
            // Find the next '#' or the end of the line
            let hash_idx = line.iter().skip(start_idx).enumerate().find(|&(_, &c)| c == '#').map(|(idx, _)| idx + start_idx).unwrap_or(line.len());
    
            for i in start_idx..hash_idx {
                if line[i] == 'O' {
                    ret += line.len() as i64 - current_pos as i64; // Update ret based on the 'slid' position
                    current_pos += 1; // Update current_pos for the next rock
                }
            }
    
            // Update start_idx and current_pos for the next section
            start_idx = hash_idx + 1;
            current_pos = start_idx;  // Rocks will start sliding from the next position after '#'
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

    for line in input {
    }
    ret
}
