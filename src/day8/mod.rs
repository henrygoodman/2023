// This seems like a network of nodes with a head , and each node has a left and a right.
// we need to:
// - create a HashMap to store relationship between node and left/right
// - iterate the initial steps (repeating if necessary) until we reach node ZZZ

use std::collections::HashMap;
use num::integer::lcm;
use rayon::prelude::*;

pub fn solve1(input: Vec<String>) -> i32 {
    let mut steps: i32 = 0;
    let mut instruction: Vec<char> = vec![];
    let mut nodes: HashMap<String, (String, String)> = HashMap::new();

    let start_val = "AAA";
    let end_val = "ZZZ";

    for (idx, line) in input.iter().enumerate() {
        if idx == 0 {
            instruction = line.chars().collect();
        }
        else if line != "" {
            let curr = line.split('=').nth(0).unwrap().split_whitespace().nth(0).unwrap().to_string();
            let left = line.split('(').nth(1).unwrap().split(',').nth(0).unwrap().to_string();
            let right = line.split('(').nth(1).unwrap().split(',').nth(1).unwrap().split(')').nth(0).unwrap().split_whitespace().nth(0).unwrap().to_string();
            nodes.insert(curr, (left, right));
        }
    }

    // Find 'AAA', and then iterate the instruction set until we reach 'ZZZ'
    let mut curr = start_val;
    while curr != end_val {
        for i in &instruction {
            let next = match i {
                'L' => &nodes.get(curr).unwrap().0,
                'R' => &nodes.get(curr).unwrap().1,
                _ => todo!(),
            };
            curr = next;
            steps += 1;
        }
    }
    steps
}


// Determine all starting nodes and solve all paths, return when the current nodes all end in 'Z'
pub fn solve2(input: Vec<String>) -> i64 {
    let mut instruction: Vec<char> = vec![];
    let mut starting_nodes: Vec<String> = Vec::new();
    let mut nodes: HashMap<String, (String, String)> = HashMap::new();

    let start_val = 'A';
    let end_val = 'Z';

    for (idx, line) in input.iter().enumerate() {
        if idx == 0 {
            instruction = line.chars().collect();
        }
        else if line != "" {
            let curr = line.split('=').nth(0).unwrap().split_whitespace().nth(0).unwrap().to_string();
            let left = line.split('(').nth(1).unwrap().split(',').nth(0).unwrap().to_string();
            let right = line.split('(').nth(1).unwrap().split(',').nth(1).unwrap().split(')').nth(0).unwrap().split_whitespace().nth(0).unwrap().to_string();
            if curr.chars().nth(2).unwrap() == start_val {
                starting_nodes.push(curr.clone());
            }
            nodes.insert(curr, (left, right));
        }
    }

    // Track each starting val and the current node during iteration
    let currs: Vec<String> = starting_nodes;

    let step_vec: Vec<i64> = currs.par_iter().map(|start| {
        let mut steps: i64 = 0;
        let mut curr = start.clone();
        while curr.chars().nth(2).unwrap() != end_val {
            for i in &instruction {
                let next = match i {
                    'L' => &nodes.get(&curr).unwrap().0,
                    'R' => &nodes.get(&curr).unwrap().1,
                    _ => todo!(),
                };
                curr = next.to_string();
                steps += 1;
            }
        }
        steps
    }).collect();

    // It seems that the looping length is the same as the number of steps to get from A-Z (i.e. after Z goes back to a neighbour of A)
    // In this case, we can just get the LCM and return it
    let lcm_val = step_vec.iter().fold(1, |acc, &x| lcm(acc, x));
    lcm_val
}









