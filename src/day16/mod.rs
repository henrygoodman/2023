use std::collections::HashMap;
use std::collections::HashSet;
use rayon::prelude::*;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn update_from_position(self, position: Position) -> [Option<Direction>; 2] {
        match (self, position) {
            (Direction::North, Position::MirrorNW) => [Some(Direction::West), None],
            (Direction::North, Position::MirrorSW) => [Some(Direction::East), None],
            (Direction::North, Position::SplitterH) => [Some(Direction::East), Some(Direction::West)],

            (Direction::South, Position::MirrorNW) => [Some(Direction::East), None],
            (Direction::South, Position::MirrorSW) => [Some(Direction::West), None],
            (Direction::South, Position::SplitterH) => [Some(Direction::East), Some(Direction::West)],

            (Direction::East, Position::MirrorNW) => [Some(Direction::South), None],
            (Direction::East, Position::MirrorSW) => [Some(Direction::North), None],
            (Direction::East, Position::SplitterV) => [Some(Direction::South), Some(Direction::North)],

            (Direction::West, Position::MirrorNW) => [Some(Direction::North), None],
            (Direction::West, Position::MirrorSW) => [Some(Direction::South), None],
            (Direction::West, Position::SplitterV) => [Some(Direction::South), Some(Direction::North)],

            _ => [Some(self), None],
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Position {
    Empty,
    MirrorNW,
    MirrorSW,
    SplitterH,
    SplitterV,
}

impl Position {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '.' => Some(Position::Empty),
            'J' => Some(Position::MirrorNW),
            '/' => Some(Position::MirrorSW),
            '-' => Some(Position::SplitterH),
            '|' => Some(Position::SplitterV),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
struct LightBeam {
    position: (usize, usize),
    direction: Direction,
    terminated: bool,
}

impl LightBeam {
    fn progress(&mut self, grid: &Vec<Position>, height: usize, width: usize) -> Option<LightBeam> {
        if let Some(new_position_index) = self.get_next_position(height, width) {
            let new_position: Position = grid[new_position_index.0 * width + new_position_index.1];
            let directions = self.direction.update_from_position(new_position);

            // Update position and direction
            self.position = new_position_index;
            self.direction = directions[0].unwrap();

            // If we split in multiple directions, return another beam
            if let Some(second_direction) = directions[1] {
                Some(LightBeam {
                    position: new_position_index,
                    direction: second_direction,
                    terminated: false,
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_next_position(&mut self, height: usize, width: usize) -> Option<(usize, usize)> {
        match self.direction {
            Direction::North if self.position.0 > 0 => Some((self.position.0 - 1, self.position.1)),
            Direction::South if self.position.0 < height - 1 => Some((self.position.0 + 1, self.position.1)),
            Direction::East if self.position.1 < width - 1 => Some((self.position.0, self.position.1 + 1)),
            Direction::West if self.position.1 > 0 => Some((self.position.0, self.position.1 - 1)),
            _ => {
                self.terminated = true;
                None
            },
        }
    }
}

fn process(grid: &Vec<Position>, height: usize, width: usize, initial_beam: LightBeam) -> i64 {
    let mut map: HashMap<((usize, usize), Direction), bool> = HashMap::new();
    let mut beams: Vec<LightBeam> = vec![initial_beam];

    while !beams.is_empty() {
        let mut new_beams: Vec<LightBeam> = Vec::new();

        for beam in beams.iter_mut() {
            if map.insert((beam.position, beam.direction), true).is_some() {
                beam.terminated = true;
            }

            if !beam.terminated {
                if let Some(new_beam) = beam.progress(grid, height, width) {
                    new_beams.push(new_beam);
                }
            }
        }

        beams = beams.into_iter().filter(|beam| !beam.terminated).collect();
        beams.extend(new_beams);
    }

    map.keys().map(|((x, y), _)| (*x, *y)).collect::<HashSet<_>>().len() as i64
}


pub fn solve1(input: Vec<String>) -> i64 {
    let grid: Vec<char> = input.iter().flat_map(|row| row.chars()).collect();
    let position_grid: Vec<Position> = grid.iter()
        .map(|&c| {
            if c == '\\' { Position::from_char('J').unwrap() }
            else { Position::from_char(c).unwrap() }
        })
        .collect();
    let height = input.len();
    let width = input[0].len();

    let initial_beam = LightBeam { 
        position: (0, 0), 
        direction: Direction::East.update_from_position(position_grid[0])[0].unwrap(), 
        terminated: false,
    };
    process(&position_grid, height, width, initial_beam)
}

pub fn solve2(input: Vec<String>) -> i64 {
    let grid: Vec<char> = input.iter().flat_map(|row| row.chars()).collect();
    let position_grid: Vec<Position> = grid.iter()
        .map(|&c| {
            if c == '\\' { Position::from_char('J').unwrap() }
            else { Position::from_char(c).unwrap() }
        })
        .collect();
    let height = input.len();
    let width = input[0].len();

    let mut start_positions: Vec<(usize, usize, Direction)> = Vec::new();

    // Top and bottom rows
    for j in 0..width {
        start_positions.push((0, j, Direction::South));
        start_positions.push((height - 1, j, Direction::North));
    }

    // Left and right columns
    for i in 1..height - 1 {
        start_positions.push((i, 0, Direction::East));
        start_positions.push((i, width - 1, Direction::West));
    }

    // Process each beam in parallel
    let results: Vec<i64> = start_positions
        .par_iter()
        .map(|&(i, j, dir)| {
            let initial_beam = LightBeam { position: (i, j), direction: dir, terminated: false };
            process(&position_grid, height, width, initial_beam)
        })
        .collect();

    *results.iter().max().unwrap()
}
