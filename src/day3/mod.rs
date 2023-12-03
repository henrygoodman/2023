// Determine a `number` by a string of consecutive digits. once we have determined a number start and end, we can parse each
// digit and determine if there is a symbol surrounding it.

pub fn solve1(input: Vec<String>) -> i32 {
    let mut ret: i32 = 0;
    let mut number: bool = false;
    let mut symbol_detected: bool = false;
    let mut current_number: String = String::new();

    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                number = true;
                current_number.push(c);
                
                // Determine if there is a symbol surrounding current char
                for yoff in 0..3 {
                    for xoff in 0..3 {
                        let check_y = y as isize + yoff - 1;
                        let check_x = x as isize + xoff - 1;
                        if check_y >= 0 && (check_y as usize) < input.len() && check_x >= 0 && (check_x as usize) < line.len() {
                            if !("1234567890.").contains(input[check_y as usize].chars().nth(check_x as usize).unwrap()) {
                                symbol_detected = true;
                            }
                        }
                    }
                }
            } else {
                // If we were parsing a number that has now finished, sum if there is a symbol detected
                if number && symbol_detected {
                    ret += current_number.parse::<i32>().unwrap_or(0);
                }
                number = false;
                symbol_detected = false;
                current_number.clear();
            }
        }
    }

    ret
}

// Same as part 1, except we need to track co-ordinates of gears such that we can associate numbers with a gear. 
// Keep track of gear co-ordinates when parsing numbers, i.e. when we find a symbol if it is a gear then add coords to list along with the current number.
// Next time we find a gear, check if exists in the list. If it does, then we multiply the current number to the number associated with the gear.
use std::collections::HashMap;

pub fn solve2(input: Vec<String>) -> i128 {
    let mut ret: i128 = 0;
    let mut number: bool = false;
    let mut gear_detected: bool = false;
    let mut gear_location: (i32, i32) = (0,0);
    let mut current_number: String = String::new();
    let mut map: HashMap<(i32, i32) , i32> = HashMap::new();

    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                number = true;
                current_number.push(c);
                
                // Determine if there is a symbol surrounding current char
                for yoff in 0..3 {
                    for xoff in 0..3 {
                        let check_y = y as isize + yoff - 1;
                        let check_x = x as isize + xoff - 1;
                        if check_y >= 0 && (check_y as usize) < input.len() && check_x >= 0 && (check_x as usize) < line.len() {
                            if "*".contains(input[check_y as usize].chars().nth(check_x as usize).unwrap()) {
                                // Check if the gear has already been found, if it has multiple the other number and add to ret sum
                                gear_detected = true;
                                gear_location = (check_y as i32, check_x as i32);
                            }
                        }
                    }
                }
            } else {
                // If we were parsing a number that has now finished, sum if there is a symbol detected
                if number {
                    if gear_detected {
                        if map.contains_key(&gear_location) {
                            let ratio: i128 = current_number.parse::<i32>().unwrap() as i128 * *map.get(&gear_location).unwrap() as i128;
                            ret += ratio;
                        } else {
                            map.insert(gear_location, current_number.parse().unwrap());
                        }
                        gear_detected = false;
                        gear_location = (0,0);
                    }
                    number = false;
                    current_number.clear();
                }
            }
        }
    }
    ret
}





