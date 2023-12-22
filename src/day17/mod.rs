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

    fn can_move_in_direction(&self, dir: Direction, width: usize, height: usize, min_steps: usize, max_steps: usize) -> bool {
        let (y, x) = self.position;

        // Check if the move is within the grid boundaries
        let within_bounds = match dir {
            Direction::Up => y > 0,
            Direction::Down => y < height - 1,
            Direction::Left => x > 0,
            Direction::Right => x < width - 1,
            Direction::Null => false,
        };


        // Early return if the move is out of bounds
        if !within_bounds {
            return false;
        }

        let (direction, steps_in_direction) = (self.current_direction(), self.steps_in_direction() as usize);

        // Check for moving in the opposite direction
        let is_opposite_direction = matches!(
            (direction, dir),
            (Direction::Up, Direction::Down) | (Direction::Down, Direction::Up) |
            (Direction::Left, Direction::Right) | (Direction::Right, Direction::Left)
        );

        if is_opposite_direction {
            return false;
        }

        // Check step constraints
        let can_move = if direction == dir {
            steps_in_direction < max_steps || max_steps == 0
        } else {
            steps_in_direction >= min_steps || direction == Direction::Null
        };
    
        println!("At position: {:?}, Trying to move: {:?}, CurrentDir: {:?}, Steps: {:?}, Can move: {}", self.position, dir, direction, steps_in_direction, can_move);
        can_move
    }
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    min_steps: usize,
    max_steps: usize,
    positions: Vec<Position>,
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
            positions
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

        let mut paths: HashMap<(usize, usize), Vec<Position>> = HashMap::new();
        paths.insert(initial_position.position, vec![initial_position]);

        let mut heap = BinaryHeap::new();
        heap.push(initial_position);

        while let Some(current) = heap.pop() {
            let best_cost_current = *tentative_distances.get(&current.position).unwrap_or(&usize::MAX);

            if let Some(neighbours) = self.get_neighbours(current) {
                for neighbour in neighbours {
                    let new_distance = best_cost_current + neighbour.cost;
                    let best_cost_neighbour = *tentative_distances.get(&neighbour.position).unwrap_or(&usize::MAX);

                    if new_distance < best_cost_neighbour {
                        tentative_distances.insert(neighbour.position, new_distance);
                        let mut new_path = vec![];
                        if let Some(path) = paths.get(&current.position) {
                            new_path.extend(path);
                        }
                        new_path.push(neighbour.clone());
                        paths.insert(neighbour.position, new_path);
                        heap.push(neighbour.clone());
                    }
                }
            }
        }

        self.print_path(paths.get(&end_position).unwrap_or(&vec![]));
        *tentative_distances.get(&end_position).unwrap_or(&usize::MAX)
    }

    fn print_path(&self, path: &Vec<Position>) {
        path.iter().for_each(|p| println!("{:?}", p));
        for (i, p) in self.positions.iter().enumerate() {
            if path.contains(&p) {
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
            // println!("{:?} {:?} {:?}", pos, dir, pos.can_move_in_direction(dir, self.width, self.height, self.min_steps, self.max_steps));
            if pos.can_move_in_direction(dir, self.width, self.height, self.min_steps, self.max_steps) {
                if let Some(new_pos) = self.next_position_in_direction(pos, dir) {
                    println!("  Pushing: {:?}", new_pos);
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
    let max_steps = 3;
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
