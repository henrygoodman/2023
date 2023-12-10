use std::collections::{HashSet, HashMap};

fn get_neighbours(
    maze: &[Vec<char>], 
    location: (usize, usize), 
    prev: (usize, usize), 
    movement_map: &HashMap<char, Vec<(i32, i32)>>
) -> Option<(usize, usize)> {
    if let Some(directions) = movement_map.get(&maze[location.0][location.1]) {
        for &(i, j) in directions {
            let new_i = location.0 as i32 + i;
            let new_j = location.1 as i32 + j;

            if new_i < 0 || new_i >= maze.len() as i32 || new_j < 0 || new_j >= maze[location.0].len() as i32 {
                continue;
            }

            let new_location = (new_i as usize, new_j as usize);
            if new_location != prev {
                return Some(new_location);
            }
        }
    }
    None
}

fn create_movement_map() -> HashMap<char, Vec<(i32, i32)>> {
    let mut map = HashMap::new();
    map.insert('S', vec![(1, 0), (-1, 0), (0, 1), (0, -1)]);
    map.insert('7', vec![(1, 0), (0, -1)]);
    map.insert('F', vec![(1, 0), (0, 1)]);
    map.insert('L', vec![(-1, 0), (0, 1)]);
    map.insert('J', vec![(-1, 0), (0, -1)]);
    map.insert('|', vec![(1, 0), (-1, 0)]);
    map.insert('-', vec![(0, 1), (0, -1)]);
    map
}

fn find_start_location(maze: &[Vec<char>]) -> (usize, usize) {
    maze.iter().enumerate()
        .find_map(|(i, line)| {
            line.iter().position(|&c| c == 'S').map(|j| (i, j))
        })
        .unwrap_or((0, 0))
}

pub fn solve1(input: Vec<String>) -> i32 {
    let maze: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();
    let movement_map = create_movement_map();
    let starting_location = find_start_location(&maze);

    let mut visited = HashSet::new();
    let mut current = starting_location;
    let mut prev = starting_location;
    visited.insert(current);

    while let Some(next) = get_neighbours(&maze, current, prev, &movement_map) {
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
    let movement_map = create_movement_map();
    let starting_location = find_start_location(&maze);

    let mut visited = HashSet::new();
    let mut current = starting_location;
    let mut prev = starting_location;
    visited.insert(current);

    while let Some(next) = get_neighbours(&maze, current, prev, &movement_map) {
        if !visited.insert(next) {
            break;
        }
        prev = current;
        current = next;
    }

    let mut ret = 0;
    for (i, line) in maze.iter().enumerate() {
        let mut inside = false;
        for (j, &c) in line.iter().enumerate() {
            if visited.contains(&(i, j)) && ("LJ|").contains(c) {
                inside = !inside;
            } else if inside && !visited.contains(&(i, j)) {
                ret += 1;
            }
        }
    }

    ret
}
