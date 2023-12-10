use std::collections::HashSet;

fn get_neighbours(maze: &[Vec<char>], location: (usize, usize), prev: (usize, usize)) -> Option<(usize, usize)> {
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)]; // NESW

    for &(i, j) in &directions {
        let new_i = location.0 as i32 + i;
        let new_j = location.1 as i32 + j;

        if new_i < 0 || new_i >= maze.len() as i32 || new_j < 0 || new_j >= maze[location.0].len() as i32 {
            continue;
        }

        let new_location = (new_i as usize, new_j as usize);
        if new_location == prev {
            continue;
        }

        let current_char = maze[location.0][location.1];
        let neighbour = maze[new_i as usize][new_j as usize];

        match (i, j) {
            (1, 0) if ("LJ|").contains(neighbour) && ("S7F|").contains(current_char) => return Some(new_location),
            (-1, 0) if ("7F|").contains(neighbour) && ("SLJ|").contains(current_char) => return Some(new_location),
            (0, 1) if ("J7-").contains(neighbour) && ("SLF-").contains(current_char) => return Some(new_location),
            (0, -1) if ("LF-").contains(neighbour) && ("SJ7-").contains(current_char) => return Some(new_location),
            _ => {}
        }
    }

    None
}

fn find_start_location(maze: &[Vec<char>]) -> (usize, usize) {
    for (i, line) in maze.iter().enumerate() {
        if let Some(j) = line.iter().position(|&c| c == 'S') {
            return (i, j);
        }
    }
    (0, 0) // Default start location if 'S' is not found
}

pub fn solve1(input: Vec<String>) -> i32 {
    let maze: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();
    let starting_location = find_start_location(&maze);

    let mut visited = HashSet::new();
    let mut current = starting_location;
    let mut prev = starting_location;
    visited.insert(current);

    while let Some(next) = get_neighbours(&maze, current, prev) {
        if !visited.insert(next) {
            break;
        }
        prev = current;
        current = next;
    }

    visited.len() as i32 / 2
}

pub fn solve2(input: Vec<String>) -> i32 {
    let maze: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();
    let starting_location = find_start_location(&maze);

    let mut visited = HashSet::new();
    let mut current = starting_location;
    let mut prev = starting_location;
    visited.insert(current);

    while let Some(next) = get_neighbours(&maze, current, prev) {
        if !visited.insert(next) {
            break;
        }
        prev = current;
        current = next;
    }

    let mut ret = 0;

    for (i, line) in maze.iter().enumerate() {
        let mut inside = false;
        let mut tiles_inside = 0;

        for (j, &c) in line.iter().enumerate() {
            if visited.contains(&(i, j)) && ("LJ|").contains(c) {
                inside = !inside;
            } else if inside && !visited.contains(&(i, j)) {
                tiles_inside += 1;
            }
        }

        ret += tiles_inside;
    }

    ret
}