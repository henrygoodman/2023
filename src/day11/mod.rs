// Parse input
// Calculate manhatten distance between each and every galaxy (n * n) distances
// Add 1 for every empty column/row it traverses
use std::cmp::max;
use std::cmp::min;

pub fn solve1(input: Vec<String>) -> i64 {
    let mut ret: i64 = 0;
    let mut maze: Vec<Vec<char>> = Vec::new();
    let mut empty_rows: Vec<usize> = Vec::new();
    let mut empty_cols: Vec<usize> = Vec::new();
    let mut locations: Vec<(usize, usize)> = Vec::new();

    for (idx, line) in input.iter().enumerate() {
        let row: Vec<char> = line.chars().collect();
        let mut contains_hash = false;

        for (jdx, &c) in row.iter().enumerate() {
            if c == '#' {
                contains_hash = true;
                locations.push((idx, jdx));
            }
        }
        if !contains_hash {
            empty_rows.push(idx);
        }
        maze.push(row);
    }

    // Find empty columns
    let col_count = maze[0].len();
    for col in 0..col_count {
        if maze.iter().all(|row| row[col] != '#') {
            empty_cols.push(col);
        }
    }

    empty_rows.sort_unstable();
    empty_cols.sort_unstable();
    
    // Calculate distance between each location
    for (i, location) in locations.iter().enumerate() {
        for j in i+1..locations.len() {
            let loc1 = location;
            let loc2 = locations[j];

            let mut distance = max(loc1.0, loc2.0) - min(loc1.0, loc2.0) + max(loc1.1, loc2.1) - min(loc1.1, loc2.1);

            // Add 1 if the path traverses an empty row/column
            let row_range = min(loc1.0, loc2.0)..max(loc1.0, loc2.0);
            for row in &empty_rows {
                if row >= max(&loc1.0, &loc2.0) { break; }
                if row_range.contains(row) {
                    distance += 1;
                }
            }
            let col_range = min(loc1.1, loc2.1)..max(loc1.1, loc2.1);
            for col in &empty_cols {
                if col >= max(&loc1.1, &loc2.1) { break; }
                if col_range.contains(col) {
                    distance += 1;
                }
            }
            ret += distance as i64;
        }
    }
    ret
}

pub fn solve2(input: Vec<String>) -> i64 {
    let mut ret: i64 = 0;
    let mut maze: Vec<Vec<char>> = Vec::new();
    let mut empty_rows: Vec<usize> = Vec::new();
    let mut empty_cols: Vec<usize> = Vec::new();
    let mut locations: Vec<(usize, usize)> = Vec::new();

    for (idx, line) in input.iter().enumerate() {
        let row: Vec<char> = line.chars().collect();
        let mut contains_hash = false;

        for (jdx, &c) in row.iter().enumerate() {
            if c == '#' {
                contains_hash = true;
                locations.push((idx, jdx));
            }
        }
        if !contains_hash {
            empty_rows.push(idx);
        }
        maze.push(row);
    }

    // Find empty columns
    let col_count = maze[0].len();
    for col in 0..col_count {
        if maze.iter().all(|row| row[col] != '#') {
            empty_cols.push(col);
        }
    }

    empty_rows.sort_unstable();
    empty_cols.sort_unstable();
    
    // Calculate distance between each location
    for (i, location) in locations.iter().enumerate() {
        for j in i+1..locations.len() {
            let loc1 = location;
            let loc2 = locations[j];

            let mut distance = max(loc1.0, loc2.0) - min(loc1.0, loc2.0) + max(loc1.1, loc2.1) - min(loc1.1, loc2.1);

            // Add 1 if the path traverses an empty row/column
            let row_range = min(loc1.0, loc2.0)..max(loc1.0, loc2.0);
            for row in &empty_rows {
                if row > max(&loc1.0, &loc2.0) { break; }
                if row_range.contains(row) {
                    distance += 999999;
                }
            }
            let col_range = min(loc1.1, loc2.1)..max(loc1.1, loc2.1);
            for col in &empty_cols {
                if col > max(&loc1.1, &loc2.1) { break; }
                if col_range.contains(col) {
                    distance += 999999;
                }
            }
            ret += distance as i64;
        }
    }
    ret
}
