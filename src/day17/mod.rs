use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Null,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Position {
    position: (usize, usize),
    cost: usize,
    direction: Direction,
    steps_in_direction: usize,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.cost)
    }
}

impl Position {
    fn can_move_in_direction(&self, dir: Direction, min_steps: usize, max_steps: usize) -> bool {
        if self.direction == Direction::Null {
            return true;
        }
    
        let is_opposite_direction = match (self.direction, dir) {
            (Direction::Up, Direction::Down) | (Direction::Down, Direction::Up) => true,
            (Direction::Left, Direction::Right) | (Direction::Right, Direction::Left) => true,
            _ => false
        };
    
        if is_opposite_direction {
            return false;
        }
    
        if self.direction == dir {
            if max_steps > 0 && self.steps_in_direction >= max_steps {
                return false;
            }
            return true;
        }
    
        // If changing direction, only allow if enough steps were made in the current direction
        min_steps == 0 || self.steps_in_direction >= min_steps
    }
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    min_steps: usize,
    max_steps: usize,
    positions: Vec<Position>,
    neighbours: HashMap<Position, Vec<Position>>,
    predecessors: HashMap<((usize, usize), Direction, usize), ((usize, usize), Direction, usize)>,
}

impl Grid {
    fn new(input: Vec<String>, min_steps: usize, max_steps: usize) -> Self {
        let height = input.len();
        let width = input[0].len();
        let mut positions = Vec::new();
        let neighbours = HashMap::new();

        for (y, row) in input.iter().enumerate() {
            for (x, c) in row.chars().enumerate() {
                let cost = c.to_digit(10).expect("Should be digit") as usize;
                for &dir in &[Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
                    for step_count in 1..=max_steps {
                        if Grid::is_position_reachable((y, x), dir, step_count, width, height) {
                            let pos = Position { position: (y, x), cost, direction: dir, steps_in_direction: step_count };
                            positions.push(pos);
                        }
                    }
                }
                if (y, x) == (0, 0) {
                    positions.push(Position {
                        position: (0, 0),
                        cost,
                        direction: Direction::Null,
                        steps_in_direction: 0,
                    });
                }
            }
        }

        Grid {
            width,
            height,
            min_steps,
            max_steps,
            positions,
            neighbours,
            predecessors: HashMap::new()
        }
    }

    fn initialize_neighbours(&mut self) {
        for pos in &self.positions {
            self.neighbours.insert(*pos, self.compute_neighbours_for(pos));
        }
        println!("Finished initializing neighbours.");
    }

    fn get_position(&self, coords: (usize, usize), direction: Direction, steps: usize) -> Position {
        let index = self.positions.iter().position(|&p| {
            p.position == coords && p.direction == direction && p.steps_in_direction == steps
        });
        match index {
            Some(i) => self.positions[i],
            None => panic!("Invalid position or direction/steps not found"),
        }
    }

    fn find_shortest_path(&mut self, initial_position: Position, end_position_coords: (usize, usize)) -> usize {
        // Populate the neighbours cache
        self.initialize_neighbours();

        let mut tentative_distances = HashMap::new();
        self.positions.iter().for_each(|&pos| {
            tentative_distances.insert((pos.position, pos.direction, pos.steps_in_direction), usize::MAX);
        });
        tentative_distances.insert((initial_position.position, initial_position.direction, initial_position.steps_in_direction), 0);
    
        let mut unvisited = self.positions.clone().into_iter().collect::<HashSet<_>>();
    
        let mut current = initial_position;
        let mut visited_end_positions = HashSet::new();
        let mut end_position_states = HashSet::new();
        for position in &self.positions {
            if position.position == end_position_coords {
                end_position_states.insert((position.position, position.direction, position.steps_in_direction));
            }
        }

        while !unvisited.is_empty() {
            if let Some(neighbours) = self.get_neighbours(current) {
                for neighbour in neighbours {
                    if unvisited.contains(&neighbour) {
                        let current_state = (current.position, current.direction, current.steps_in_direction);
                        let neighbour_state = (neighbour.position, neighbour.direction, neighbour.steps_in_direction);
                        
                        let new_distance = tentative_distances[&current_state] + neighbour.cost;
                        if new_distance < tentative_distances[&neighbour_state] {
                            tentative_distances.insert(neighbour_state, new_distance);
                            self.predecessors.insert(neighbour_state, current_state);
                        }
                    }
                }
            }

            // Mark current as visited
            unvisited.remove(&current);

            // Check if current is an end position state
            if current.position == end_position_coords {
                // Check if the current state meets the min_steps requirement
                if self.min_steps == 0 || current.steps_in_direction >= self.min_steps {
                    visited_end_positions.insert((current.position, current.direction, current.steps_in_direction));
                }
                // Break if all reachable end position states have been visited
                if visited_end_positions == end_position_states {
                    break;
                }
            }

            // Select the next node with the smallest tentative distance
            if let Some(next) = unvisited.iter().min_by_key(|pos| tentative_distances[&(pos.position, pos.direction, pos.steps_in_direction)]) {
                current = *next;
            } else {
                println!("No more nodes to visit, breaking out of the loop.");
                break;
            }
        }

        // Find the minimum distance to any state at the end position
        let min_distance = visited_end_positions.iter()
            .map(|state| tentative_distances[state])
            .min()
            .unwrap_or(usize::MAX);
        
        min_distance

        // if min_distance < usize::MAX {
        //     // Reconstruct and print the shortest path
        //     let min_state = visited_end_positions.iter()
        //         .find(|&&state| tentative_distances[&state] == min_distance)
        //         .unwrap();
    
        //     let mut path = Vec::new();
        //     let mut current_state = *min_state;

        //     while current_state != (initial_position.position, initial_position.direction, initial_position.steps_in_direction) {
        //         path.push(current_state);
        //         current_state = self.predecessors[&current_state];
        //     }
        //     path.push(((initial_position.position.0, initial_position.position.1), initial_position.direction, initial_position.steps_in_direction));
        //     path.reverse();
    
        //     println!("Shortest path distance: {}", min_distance);

        //     for (pos, dir, steps) in path {
        //         println!("Position: {:?}, Direction: {:?}, Steps: {}", pos, dir, steps);
        //     }
    
        //     min_distance
        // } else {
        //     println!("No valid path found");
        //     usize::MAX
        // }
    } 

    fn get_neighbours(&self, pos: Position) -> Option<Vec<Position>> {
        self.neighbours.get(&pos).cloned()
    }

    fn next_position_in_direction(&self, (y, x): (usize, usize), dir: Direction) -> Option<(usize, usize)> {
        match dir {
            Direction::Up => if y > 0 { Some((y - 1, x)) } else { None },
            Direction::Down => if y < self.height - 1 { Some((y + 1, x)) } else { None },
            Direction::Left => if x > 0 { Some((y, x - 1)) } else { None },
            Direction::Right => if x < self.width - 1 { Some((y, x + 1)) } else { None },
            _ => None,
        }
    }

    fn compute_neighbours_for(&self, pos: &Position) -> Vec<Position> {
        let (y, x) = pos.position;
        let mut neighbours = Vec::new();
    
        let directions = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];
        for &dir in &directions {
            if pos.can_move_in_direction(dir, self.min_steps, self.max_steps) {
                if let Some(new_pos_coords) = self.next_position_in_direction((y, x), dir) {
                    let steps_in_new_direction = if dir == pos.direction && pos.steps_in_direction < self.max_steps  {
                        pos.steps_in_direction + 1
                    } else if dir != pos.direction {
                        1
                    } else {
                        continue;
                    };
                    let new_pos = self.get_position(new_pos_coords, dir, steps_in_new_direction);
                    neighbours.push(new_pos);
                }
            }
        }
        neighbours
    }

    fn is_position_reachable(pos: (usize, usize), dir: Direction, steps: usize, width: usize, height: usize) -> bool {
        match dir {
            Direction::Up => pos.0 + steps < height,
            Direction::Down => pos.0 as isize - steps as isize >= 0,
            Direction::Left => pos.1 + steps < width,
            Direction::Right => pos.1 as isize - steps as isize >= 0,
            _ => true, // For Null direction, always reachable
        }
    }
}


pub fn solve1(input: Vec<String>) -> i64 {
    let min_steps = 0;
    let max_steps = 3;
    let mut grid = Grid::new(input, min_steps, max_steps);
    let initial_position: Position = grid.get_position((0,0), Direction::Null, 0);
    let end_position = (grid.height- 1, grid.width - 1);
    let cost = grid.find_shortest_path(initial_position, end_position);
    println!("Minimum cost from left to right: {}", cost);
    cost as i64
}



pub fn solve2(input: Vec<String>) -> i64 {
    let min_steps = 4;
    let max_steps = 10;
    let mut grid = Grid::new(input, min_steps, max_steps);
    let initial_position: Position = grid.get_position((0,0), Direction::Null, 0);
    let end_position = (grid.height- 1, grid.width - 1);
    let cost = grid.find_shortest_path(initial_position, end_position);
    println!("Minimum cost from left to right: {}", cost);
    cost as i64
}
