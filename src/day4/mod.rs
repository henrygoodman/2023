// Points are determined by the length of the set intersection between the winning numbers and drawn numbers
use std::collections::HashSet;

pub fn solve1(input: Vec<String>) -> i32 {
    let mut ret: i32 = 0;

    for (idx, line) in input.iter().enumerate() {
        let winning_numbers_str = line.split('|').nth(0).unwrap().split(':').nth(1).unwrap();
        let drawn_numbers_str = line.split('|').nth(1).unwrap();

        let winning_numbers: Vec<i32> = winning_numbers_str.split_whitespace().map(|x| x.parse::<i32>()).collect::<Result<Vec<i32>, _>>().unwrap();
        let drawn_numbers: Vec<i32> = drawn_numbers_str.split_whitespace().map(|x| x.parse::<i32>()).collect::<Result<Vec<i32>, _>>().unwrap();

        // Convert into hashsets
        let winning_set: HashSet<i32> = winning_numbers.into_iter().collect();
        let drawn_set: HashSet<i32> = drawn_numbers.into_iter().collect();
        
        // Calculate the set intersection between 2 vecs
        let intersection_set: HashSet<i32> = winning_set.intersection(&drawn_set).cloned().collect();

        // Append points based on length of intersection set\
        if intersection_set.len() >= 1 {
            ret += 2_i32.pow(intersection_set.len() as u32 - 1);
        }
    }
    ret
}

pub fn solve2(input: Vec<String>) -> i32 {
    let mut num_copies: Vec<i32> = vec![1; input.len()];

    for (idx, line) in input.iter().enumerate() {
        let winning_numbers_str = line.split('|').nth(0).unwrap().split(':').nth(1).unwrap();
        let drawn_numbers_str = line.split('|').nth(1).unwrap();

        let winning_numbers: Vec<i32> = winning_numbers_str.split_whitespace().map(|x| x.parse::<i32>()).collect::<Result<Vec<i32>, _>>().unwrap();
        let drawn_numbers: Vec<i32> = drawn_numbers_str.split_whitespace().map(|x| x.parse::<i32>()).collect::<Result<Vec<i32>, _>>().unwrap();

        // Convert into hashsets
        let winning_set: HashSet<i32> = winning_numbers.into_iter().collect();
        let drawn_set: HashSet<i32> = drawn_numbers.into_iter().collect();
        
        // Calculate the set intersection between 2 vecs
        let intersection_set: HashSet<i32> = winning_set.intersection(&drawn_set).cloned().collect();

        let copies_len = intersection_set.len();

        for i in 1..copies_len + 1 {
            for _ in 0..num_copies[idx] {
                num_copies[idx+i] += 1;
            }
        }
    }
    num_copies.iter().sum()
}





