fn get_neighbours(maze: Vec<Vec<char>>, location: (usize, usize)) -> Vec<(usize, usize)> {
    let mut ret: Vec<(usize, usize)> = Vec::new();

    for i in 0..3 {
        for j in 0..3 {
            let i = i - 1 as i32;
            let j = j - 1 as i32;
            // Only consider NESW neighbours
            if !("-1").contains(&(i + j).to_string()) { continue ;}

            // Continue if any index out of bounds
            if location.0 as i32 + i < 0 || location.0 as i32 + i >= maze.len() as i32 || location.1 as i32 + j < 0 || location.1 as i32 + j >= maze[location.0].len() as i32 { continue; }


            let current_char = maze[location.0][location.1];
            let neighbour = maze[location.0 + i as usize][location.1 + j as usize];
            match (i, j) {
                (1, 0) => {
                    if ("LJ|").contains(neighbour) && ("S7F|").contains(current_char) {
                        ret.push((location.0 + i as usize, location.1 + j as usize));
                    }
                },
                (-1, 0) => {
                    if ("7F|").contains(neighbour) && ("SLJ|").contains(current_char) {
                        ret.push((location.0 + i as usize, location.1 + j as usize));
                    }
                },
                (0, 1) => {
                    if ("J7-").contains(neighbour) && ("SLF-").contains(current_char) {
                        ret.push((location.0 + i as usize, location.1 + j as usize));
                    }
                },
                (0, -1) => {
                    if ("LF-").contains(neighbour) && ("SJ7-").contains(current_char) {
                        ret.push((location.0 + i as usize, location.1 + j as usize));
                    }
                },
                _ => todo!()
            };
        }
    }
    return ret;
}

pub fn solve1(input: Vec<String>) -> i32 {
    let mut maze: Vec<Vec<char>> = Vec::new();
    let mut starting_location: (usize, usize) = (0, 0);
    let mut visited: Vec<(usize, usize)> = Vec::new();

    for line in input {
        maze.push(line.chars().collect());
    }
    for (i, line) in maze.iter().enumerate() {
        if !visited.is_empty() { break; }
        for (j, c) in line.iter().enumerate() {
            if *c == 'S' {
                starting_location = (i, j);
                visited.push(starting_location);
                break;
            }
        }
    }
    let mut neighbours = get_neighbours(maze.clone(), starting_location);
    
    while neighbours.iter().any(|n| !visited.contains(n)) {
        for n in &neighbours {
            if !visited.contains(&n) {
                visited.push(*n);
                neighbours = get_neighbours(maze.clone(), *n);
                break;
            }
        }
    }

    visited.len() as i32 / 2 as i32
}

pub fn solve2(input: Vec<String>) -> i32 {
    let mut ret: i32 = 0;
    let mut maze: Vec<Vec<char>> = Vec::new();
    let mut starting_location: (usize, usize) = (0, 0);
    let mut visited: Vec<(usize, usize)> = Vec::new();

    for line in input {
        maze.push(line.chars().collect());
    }
    for (i, line) in maze.iter().enumerate() {
        if !visited.is_empty() { break; }
        for (j, c) in line.iter().enumerate() {
            if *c == 'S' {
                starting_location = (i, j);
                visited.push(starting_location);
                break;
            }
        }
    }
    let mut neighbours = get_neighbours(maze.clone(), starting_location);
    
    while neighbours.iter().any(|n| !visited.contains(n)) {
        for n in &neighbours {
            if !visited.contains(&n) {
                visited.push(*n);
                neighbours = get_neighbours(maze.clone(), *n);
                break;
            }
        }
    }

    let mut inside: bool = false;

    // Traverse row by row, counting the dots when we are inside the loop
    // We are inside the loop if we pass an upward loop section, and outside once we pass another (toggling)
    for (i, line) in maze.iter().enumerate() {
        let mut tiles_inside: i32 = 0;
        for (j, c) in line.iter().enumerate() {
            if visited.contains(&(i, j)) && ("LJ|").contains(*c) {
                inside = !inside;
            }
            else if inside && !visited.contains(&(i, j)) {
                tiles_inside += 1;
            }
        }
        ret += tiles_inside;
    }
    ret
}









