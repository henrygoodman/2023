use std::str::FromStr;
use std::error::Error;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'U' => Some(Direction::Up),
            'D' => Some(Direction::Down),
            'L' => Some(Direction::Left),
            'R' => Some(Direction::Right),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    length: usize,
    color: String,
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() != 3 {
            return Err("Invalid instruction format".into());
        }

        let direction_char = parts[0].chars().next().ok_or("Empty direction string")?;
        let direction = Direction::from_char(direction_char)
            .ok_or_else(|| format!("Invalid direction character: {}", direction_char))?;

        let length = parts[1].parse::<usize>()
            .map_err(|e| format!("Invalid length: {}", e))?;

        let color = parts[2].to_string();

        Ok(Instruction { direction, length, color })
    }
}

#[derive(Debug)]
struct InstructionSet {
    instructions: Vec<Instruction>,
}

impl InstructionSet {
    fn new(input: Vec<String>) -> Result<Self, Box<dyn Error>> {
        let instructions = input.into_iter()
            .map(|line| line.parse())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(InstructionSet { instructions })
    }
}

#[derive(Copy, Clone, Debug)]
struct Point {
    position: (usize, usize),
    dug: bool
}

#[derive(Debug)]
struct Grid {
    width: usize, 
    height: usize,
    positions: Vec<Point>,
    vertices: Vec<(usize, usize)>,
    area_vertices: Vec<(usize, usize)>
}

impl Grid {
    fn new(instruction_set: &InstructionSet) -> Self {
        let mut grid = Grid {
            width: 0,
            height: 0,
            positions: Vec::new(),
            vertices: Vec::new(),
            area_vertices: Vec::new(),
        };

        // Preliminary pass to calculate the dimensions
        grid.calculate_preliminary_dimensions(instruction_set);
        grid.set_dimensions();

        // Now draw lines with the positions initialized
        grid.draw_lines(instruction_set);

        grid
    }

    fn calculate_preliminary_dimensions(&mut self, instruction_set: &InstructionSet) {
        let mut current_pos: (usize, usize) = (0, 0);
        self.vertices.push(current_pos); // Starting vertex

        for instruction in &instruction_set.instructions {
            match instruction.direction {
                Direction::Up => current_pos.1 = current_pos.1.saturating_sub(instruction.length),
                Direction::Down => current_pos.1 += instruction.length,
                Direction::Left => current_pos.0 = current_pos.0.saturating_sub(instruction.length),
                Direction::Right => current_pos.0 += instruction.length,
            }
            self.vertices.push(current_pos);
        }

        // Remove the last vertex if it is a duplicate
        if self.vertices.last() == Some(&(0, 0)) {
            self.vertices.pop();
        }
    }

    fn set_dimensions(&mut self) {
        let (max_x, max_y) = self.vertices.iter().fold((0, 0), |(max_x, max_y), &(x, y)| {
            (max_x.max(x), max_y.max(y))
        });

        self.width = max_x + 1;
        self.height = max_y + 1;
        // Initialize the positions with the correct coordinates
        self.positions = (0..self.height).flat_map(|y| {
            (0..self.width).map(move |x| Point { position: (x, y), dug: false })
        }).collect();
    }

    fn draw_lines(&mut self, instruction_set: &InstructionSet) {
        let mut current_pos: (usize, usize) = (0, 0);
    
        for instruction in &instruction_set.instructions {
            // Draw the line for each instruction
            for _ in 0..instruction.length {
                match instruction.direction {
                    Direction::Up => {
                        if current_pos.1 > 0 {
                            current_pos.1 -= 1;
                        }
                    },
                    Direction::Down => {
                        if current_pos.1 < self.height - 1 {
                            current_pos.1 += 1;
                        }
                    },
                    Direction::Left => {
                        if current_pos.0 > 0 {
                            current_pos.0 -= 1;
                        }
                    },
                    Direction::Right => {
                        if current_pos.0 < self.width - 1 {
                            current_pos.0 += 1;
                        }
                    },
                }
    
                let index = current_pos.1 * self.width + current_pos.0;
                if let Some(point) = self.positions.get_mut(index) {
                    point.dug = true;
                }
            }
            // Adjust vertex inward by 1 unit
            let adjusted_vertex = match instruction.direction {
                Direction::Up => (current_pos.0, current_pos.1 + 1),
                Direction::Down => (current_pos.0, current_pos.1 - 1),
                Direction::Left => (current_pos.0 + 1, current_pos.1),
                Direction::Right => (current_pos.0 - 1, current_pos.1),
            };
            self.area_vertices.push(adjusted_vertex);
        }
    }
    


    fn calc_area(&self) -> f64 {
        println!("{:?}", self.area_vertices);
        let mut area = 0;
        let n = self.area_vertices.len();
    
        for i in 0..n {
            let (x1, y1) = self.area_vertices[i];
            let (x2, y2) = self.area_vertices[(i + 1) % n];
            area += x1 * y2 - x2 * y1;
            println!("{:?}", area);
        }
    
        (area as f64 / 2.0).abs()
    }
    
    

    fn display(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = y * self.width + x;
                print!("{}", if self.area_vertices.contains(&(x, y)) {'A'} else { '.' });
            }
            println!();
        }
    }
}

// todo - convert each edge piece to a direction (horizontal, vert, corner, etc so we can run even odd rule)
pub fn solve1(input: Vec<String>) -> i64 {
    let instruction_set = InstructionSet::new(input).expect("Failed to parse instructions");
    let mut grid = Grid::new(&instruction_set);
    instruction_set.instructions.iter().for_each(|instruction| println!("{:?}", instruction));

    // Draw lines on the grid
    let ans = grid.calc_area() as i64;

    // Display the grid
    grid.display();

    ans
}



pub fn solve2(input: Vec<String>) -> i64 {
    5
}
