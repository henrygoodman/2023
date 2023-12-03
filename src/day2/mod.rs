pub fn solve1(input: Vec<String>) -> i32 {
    let mut ret: i32 = 0; 
    let max_red: i32 = 12;
    let max_green: i32 = 13;
    let max_blue: i32 = 14;

    for line in &input {
        let sets: Vec<&str> = line.split(":").collect();
        let mut impossible: bool = false;

        let game_number: i32 = sets.get(0).unwrap().split(' ').nth(1).unwrap().parse().unwrap();

        let game_vector: Vec<&str> = sets.get(1).unwrap().split(';').collect();
            
        for game in game_vector {
            let colors: Vec<&str> = game.split(',').collect();
            for color in colors {
                let color_amount: i32 = color.split(' ').collect::<Vec<&str>>().get(1).unwrap().parse().unwrap();
                if color.contains("green") {
                    if color_amount > max_green {
                        impossible = true;
                        break;
                    }
                }
                else if color.contains("red") {
                    if color_amount > max_red {
                        impossible = true;
                        break;
                    }
                }
                else if color.contains("blue") {
                    if color_amount > max_blue {
                        impossible = true;
                        break;
                    }
                }
            }
        }
        if !impossible {
            ret += game_number;
        }
        
    }
    println!("Part1");
    ret
}


pub fn solve2(input: Vec<String>) -> i32 {
    let mut ret: i32 = 0; 

    for line in &input {
        let sets: Vec<&str> = line.split(":").collect();
        let mut max_red: i32 = 0;
        let mut max_green: i32 = 0;
        let mut max_blue: i32 = 0;

        let game_vector: Vec<&str> = sets.get(1).unwrap().split(';').collect();
        
        for game in game_vector {

            let colors: Vec<&str> = game.split(',').collect();
            for color in colors {
                let color_amount: i32 = color.split(' ').collect::<Vec<&str>>().get(1).unwrap().parse().unwrap();
                if color.contains("green") {
                    if color_amount > max_green {
                        max_green = color_amount;
                    }
                }
                else if color.contains("red") {
                    if color_amount > max_red {
                        max_red = color_amount;
                    }
                }
                else if color.contains("blue") {
                    if color_amount > max_blue {
                        max_blue = color_amount;
                    }
                }
            }
        }
        ret += max_green * max_red * max_blue;
        
    }
    println!("Part2");
    ret
}





