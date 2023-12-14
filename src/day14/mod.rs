use std::collections::HashMap;

fn shift_positions(grid: &mut Vec<Vec<char>>, direction: char) {
    match direction {
        'N' => shift_north(grid),
        'W' => shift_west(grid),
        'S' => shift_south(grid),
        'E' => shift_east(grid),
        _ => (),
    }
}

fn shift_north(grid: &mut Vec<Vec<char>>) {
    for col in 0..grid[0].len() {
        let mut write_row = 0;
        for row in 0..grid.len() {
            if grid[row][col] == '#' { write_row = row + 1; }
            else if grid[row][col] == 'O' {
                if row != write_row {
                    grid[write_row][col] = 'O';
                    grid[row][col] = '.';
                }
                write_row += 1;
            }
        }
    }
}

fn shift_west(grid: &mut Vec<Vec<char>>) {
    for row in grid.iter_mut() {
        let mut write_col = 0;
        for col in 0..row.len() {
            if row[col] == '#' { write_col = col + 1; }
            else if row[col] == 'O' {
                if col != write_col {
                    row[write_col] = 'O';
                    row[col] = '.';
                }
                write_col += 1;
            }
        }
    }
}

fn shift_south(grid: &mut Vec<Vec<char>>) {
    for col in 0..grid[0].len() {
        let mut write_row = grid.len() - 1;
        for row in (0..grid.len()).rev() {
            if grid[row][col] == '#' { write_row = row - 1; }
            else if grid[row][col] == 'O' {
                if row != write_row {
                    grid[write_row][col] = 'O';
                    grid[row][col] = '.';
                }
                write_row -= 1;
            }
        }
    }
}

fn shift_east(grid: &mut Vec<Vec<char>>) {
    for row in grid.iter_mut() {
        let mut write_col = row.len() - 1;
        for col in (0..row.len()).rev() {
            if row[col] == '#' { write_col = col - 1; }
            else if row[col] == 'O' {
                if col != write_col {
                    row[write_col] = 'O';
                    row[col] = '.';
                }
                write_col -= 1;
            }
        }
    }
}

fn serialize_grid(grid: &[Vec<char>]) -> String {
    grid.iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}

fn calculate_final_positions(grid: &[Vec<char>]) -> i64 {
    grid.iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter().filter(|&&c| c == 'O').count() as i64 * (grid.len() as i64 - i as i64)
        })
        .sum()
}

pub fn solve1(input: Vec<String>) -> i64 {
    let mut grid: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();

    shift_positions(&mut grid, 'N');
    calculate_final_positions(&grid)
}

pub fn solve2(input: Vec<String>) -> i64 {
    let mut grid: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();
    let mut seen_states = HashMap::new();
    let directions = ['N', 'W', 'S', 'E'];
    let total_iterations: i64 = 1_000_000_000 * 4;
    let mut iteration = 0;
    let mut cycle_length = 0;

    while iteration < total_iterations {
        for &direction in &directions {
            if iteration >= total_iterations {
                break;
            }

            shift_positions(&mut grid, direction);
            let serialized = serialize_grid(&grid);
            let state_key = (serialized, direction);

            if let Some(&previous_iteration) = seen_states.get(&state_key) {
                if cycle_length == 0 { // First time cycle is detected
                    cycle_length = iteration - previous_iteration;
                }

                // Calculate remaining iterations after skipping cycles
                let remaining_iterations = total_iterations - iteration;
                let full_cycles_to_skip = remaining_iterations / cycle_length;
                iteration += full_cycles_to_skip * cycle_length;

                // Clear the states to avoid false cycle detection in remaining iterations
                seen_states.clear();
            } else {
                seen_states.insert(state_key, iteration);
            }
            iteration += 1;
        }
    }
    calculate_final_positions(&grid)
}



