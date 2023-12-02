pub fn solve1(input: Vec<String>) -> i32 {
    let mut ret: i32 = 0;  // Make 'ret' mutable to update its value.

    for line in &input {  // Use a reference to the input vector to avoid ownership issues.
        let mut ret_str: String = "".to_string();
        for c in line.chars() {
            if c.is_digit(10) {
                ret_str.push_str(&c.to_string());
            }
        }
        let first_char = ret_str.chars().next().unwrap();
        let last_char = ret_str.chars().last().unwrap();
        let num_str = format!("{}{}", first_char, last_char);
        ret += num_str.parse::<i32>().unwrap();
        
    }
    println!("Part1");
    ret
}


pub fn solve2(input: Vec<String>) -> i32 {
    let mut ret: i32 = 0;

    for line in &input {
        let mut mutable_line = line.clone();
        mutable_line = mutable_line
                        .replace("one", "o1e")
                        .replace("two", "t2o")
                        .replace("three", "t3e")
                        .replace("four", "4")
                        .replace("five", "5e")
                        .replace("six", "6")
                        .replace("seven", "7")
                        .replace("eight", "e8t")
                        .replace("nine", "n9e");

        let mut ret_str: String = "".to_string();
        for c in mutable_line.chars() {
            if c.is_digit(10) {
                ret_str.push_str(&c.to_string());
            }
        }

        let first_char = ret_str.chars().next().unwrap();
        let last_char = ret_str.chars().last().unwrap();
        let num_str = format!("{}{}", first_char, last_char);
        ret += num_str.parse::<i32>().unwrap();
    }
    println!("Part2");
    ret
}





