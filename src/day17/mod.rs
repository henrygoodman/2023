use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Null,
    Up,
    Down,
    Left,
    Right,
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Position {
    position: (usize, usize),
    cost: usize,
    dx: isize,
    dy: isize
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.cost)
    }
}

impl Position {
    fn steps_in_direction(&self) -> isize {
        (self.dx + self.dy).abs() 
    }

    fn current_direction(&self) -> Direction {
        match (self.dx, self.dy) {
            (0, dy) if dy < 0 => Direction::Up,
            (0, dy) if dy > 0 => Direction::Down,
            (dx, 0) if dx < 0 => Direction::Left,
            (dx, 0) if dx > 0 => Direction::Right,
            _ => Direction::Null,
        }
    }

    fn direction_from(&self, other: &Position) -> Direction {
        let (self_y, self_x) = self.position;
        let (other_y, other_x) = other.position;

        if self_y == other_y && self_x == other_x {
            Direction::Null
        } else if self_y < other_y {
            Direction::Up
        } else if self_y > other_y {
            Direction::Down
        } else if self_x < other_x {
            Direction::Left
        } else {
            Direction::Right
        }
    }

    fn can_move_in_direction(&self, dir: Direction, min_steps: usize, max_steps: usize) -> bool {
        let (direction, steps_in_direction) = (self.current_direction(), self.steps_in_direction() as usize);
        
        if direction == Direction::Null {
            return true;
        }
    
        let is_opposite_direction = match (direction, dir) {
            (Direction::Up, Direction::Down) | (Direction::Down, Direction::Up) => true,
            (Direction::Left, Direction::Right) | (Direction::Right, Direction::Left) => true,
            _ => false
        };
    
        if is_opposite_direction {
            return false;
        }
    
        if direction == dir {
            if max_steps > 0 && steps_in_direction >= max_steps {
                return false;
            }
            return true;
        }
    
        // If changing direction, only allow if enough steps were made in the current direction
        min_steps == 0 || steps_in_direction >= min_steps
    }
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    min_steps: usize,
    max_steps: usize,
    positions: Vec<Position>,
    predecessors: HashMap<(usize, usize), (usize, usize)>,
}

impl Grid {
    fn new(input: Vec<String>, min_steps: usize, max_steps: usize) -> Self {
        let height = input.len();
        let width = input[0].len();
        let mut positions = Vec::new();

        for (y, row) in input.iter().enumerate() {
            for (x, c) in row.chars().enumerate() {
                let mut cost = c.to_digit(10).expect("Should be digit") as usize;
                if (y, x) == (0, 0) { cost = 0;}
                let pos = Position { position: (y, x), cost , dx: 0, dy: 0};
                positions.push(pos);
            }
        }

        Grid {
            width,
            height,
            min_steps,
            max_steps,
            positions,
            predecessors: HashMap::new()
        }
    }

    fn get_position(&self, coords: (usize, usize)) -> Position {
        let index = self.positions.iter().position(|&p| {
            p.position == coords
        });
        match index {
            Some(i) => self.positions[i],
            None => panic!("Invalid position"),
        }
    }

    fn find_shortest_path(&mut self, initial_position: Position, end_position: (usize, usize)) -> usize {
        let mut tentative_distances = HashMap::new();
        tentative_distances.insert(initial_position.position, 0);
    
        let mut heap = BinaryHeap::new();
        heap.push(initial_position);
    
        let mut current = initial_position;

        while let Some(current) = heap.pop() {
            println!("Curr: {:?}", current);
            // Check if current is an end position state
            if current.position == end_position {
                println!("Reached end position, breaking.");
                break;
            }

            let best_cost = tentative_distances.get(&current.position).unwrap_or(&usize::MAX);
            println!("Curr: {:?} Best: {:?}", current.cost, best_cost);
            if current.cost > *best_cost {
                continue;
            }

            if let Some(neighbours) = self.get_neighbours(current) {
                println!("Curr: {:?} Neigh: {:?}", current, neighbours);
                for neighbour in neighbours {
                    let best_cost_neighbour = tentative_distances.get(&current.position).unwrap_or(&usize::MAX);
                    let new_distance =  *best_cost_neighbour + neighbour.cost;
                    if new_distance < *best_cost_neighbour {
                        tentative_distances.insert(neighbour.position, new_distance);
                        println!("Pushing: {:?}", neighbour);
                        heap.push(neighbour);
                        self.predecessors.insert(neighbour.position, current.position);
                    }
                }
            }
            println!()
        }


        // Find the minimum distance to any state at the end position
        let min_distance = *tentative_distances.get(&end_position).unwrap_or(&usize::MAX);
        
        // min_distance

        if min_distance < usize::MAX {
            let mut path = Vec::new();
            let mut current_state = end_position;

            while current_state != (initial_position.position) {
                path.push(current_state);
                current_state = self.predecessors[&current_state];
            }
            path.push((initial_position.position.0, initial_position.position.1));
            path.reverse();
    
            println!("Shortest path distance: {}", min_distance);

            for pos in &path {
                println!("Position: {:?}", pos);
            }

            println!("{:?}", self.positions);
            
            self.print_path(&path);
            min_distance
        } else {
            println!("No valid path found");
            usize::MAX
        }
    }

    fn print_path(&self, path: &Vec<(usize, usize)>) {
        for (i, p) in self.positions.iter().enumerate() {
            if path.contains(&p.position) {
                print!("[{:?}]", p.cost);
            }
            else {
                print!(" {:?} ", p.cost);
            }
            if i != 0 && (i+1) % self.width == 0 { println!(); }
        }
        println!();
    }

    fn get_neighbours(&self, pos: Position) -> Option<Vec<Position>> {
        let (y, x) = pos.position;
        let mut neighbours = Vec::new();
    
        let directions = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];
        for &dir in &directions {
            if pos.can_move_in_direction(dir, self.min_steps, self.max_steps) {
                if let Some(new_pos) = self.next_position_in_direction(pos, dir) {
                    println!("NEWPOS: {:?}", new_pos);
                    neighbours.push(new_pos);
                }
            }
        }
        if neighbours.len() > 0 { Some(neighbours) } else { None }
    }

    fn next_position_in_direction(&self, pos: Position, dir: Direction) -> Option<Position> {
        let (y, x) = pos.position;
        let (dx, dy) = match dir {
            Direction::Up => (0, if pos.current_direction() == Direction::Up { pos.dy - 1 } else { -1 }),
            Direction::Down => (0, if pos.current_direction() == Direction::Down { pos.dy + 1 } else { 1 }),
            Direction::Left => (if pos.current_direction() == Direction::Left { pos.dx - 1 } else { -1 }, 0),
            Direction::Right => (if pos.current_direction() == Direction::Right { pos.dx + 1 } else { 1 }, 0),
            Direction::Null => (0, 0),
        };

        let new_position = match dir {
            Direction::Up => if y > 0 { Some((y - 1, x)) } else { None },
            Direction::Down => if y < self.height - 1 { Some((y + 1, x)) } else { None },
            Direction::Left => if x > 0 { Some((y, x - 1)) } else { None },
            Direction::Right => if x < self.width - 1 { Some((y, x + 1)) } else { None },
            _ => None,
        };

        new_position.map(|position| Position { position, cost: self.get_position(position).cost, dx, dy })
    }
}

pub fn solve1(input: Vec<String>) -> i64 {
    let min_steps = 0;
    let max_steps = 5;
    let mut grid = Grid::new(input, min_steps, max_steps);
    let initial_position: Position = grid.get_position((0,0));
    let end_position = (grid.height- 1, grid.width - 1);
    let cost = grid.find_shortest_path(initial_position, end_position);
    println!("Minimum cost from left to right: {}", cost);
    cost as i64
}



pub fn solve2(input: Vec<String>) -> i64 {
    // let min_steps = 4;
    // let max_steps = 10;
    // let mut grid = Grid::new(input, min_steps, max_steps);
    // let initial_position: Position = grid.get_position((0,0));
    // let end_position = (grid.height- 1, grid.width - 1);
    // let cost = grid.find_shortest_path(initial_position, end_position);
    // println!("Minimum cost from left to right: {}", cost);
    // cost as i64
    5
}
