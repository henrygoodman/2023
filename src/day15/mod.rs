use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref GLOBAL_MAP: Mutex<HashMap<Vec<char>, i64>> = Mutex::new(HashMap::new());
}

fn calculate_hash(sec: &[char], start_value: i64) -> i64 {
    let mut hash = start_value;

    for &c in sec {
        let ascii = c as i64;
        hash += ascii;
        hash *= 17;
        hash %= 256;
    }

    hash
}

fn get_section_hash(sec: &[char], map: &mut HashMap<Vec<char>, i64>) -> i64 {
    // Check if the hash for the entire sequence is already cached
    if let Some(&hash) = map.get(sec) {
        return hash;
    }

    // Initialize the hash of the last subsequence (initially 0)
    let mut last_hash = 0;

    // Iterate over all possible subsequences
    for i in 1..=sec.len() {
        let sub_sec = &sec[..i];
        let hash = if let Some(&cached_hash) = map.get(sub_sec) {
            // Use the cached hash if available
            cached_hash
        } else {
            // Otherwise, calculate the hash, cache it, and use it
            let new_hash = calculate_hash(sub_sec, 0);
            map.insert(sub_sec.to_vec(), new_hash);
            new_hash
        };

        // Store the hash of the entire sequence when reached
        if i == sec.len() {
            last_hash = hash;
        }
    }

    // Cache and return the hash of the entire sequence
    map.insert(sec.to_vec(), last_hash);
    last_hash
}

fn process(label: Vec<char>, box_num: i64, op: char, lens: &[char], ret_map: &mut HashMap<i64, Vec<(Vec<char>, i64)>>) {
    match op {
        '=' => {
            let digit = lens[0].to_digit(10).unwrap() as i64;
            let labels_vec = ret_map.entry(box_num).or_insert_with(Vec::new);

            if let Some(entry) = labels_vec.iter_mut().find(|l| l.0 == label) {
                // Update the label in place
                entry.1 = digit;
            } else {
                // Add the label if it does not exist
                labels_vec.push((label, digit));
            }
        },
        '-' => {
            if let Some(labels_vec) = ret_map.get_mut(&box_num) {
                // Remove the label if it exists
                if let Some(index) = labels_vec.iter().position(|l| l.0 == label) {
                    labels_vec.remove(index);
                }
                // Remove the key from the map if labels_vec is empty
                if labels_vec.is_empty() {
                    ret_map.remove(&box_num);
                }
            }
        },
        _ => {
            // Handle other operations if necessary
            println!("Unhandled operation: {}", op);
        }
    }
}

pub fn solve1(input: Vec<String>) -> i64 {
    let grid: Vec<Vec<char>> = input[0]
        .split(',')  // Splitting the borrowed string
        .map(|section| section.chars().collect())
        .collect();
    let mut global_map = GLOBAL_MAP.lock().unwrap();
    grid.iter().map(|sec| get_section_hash(sec, &mut global_map)).sum()
}

pub fn solve2(input: Vec<String>) -> i64 {
    let mut ret = 0;
    let grid: Vec<Vec<char>> = input[0]
        .split(',')
        .map(|section| section.chars().collect())
        .collect();

    let global_map = GLOBAL_MAP.lock().unwrap();
    let mut ret_map: HashMap<i64, Vec<(Vec<char>, i64)>> = HashMap::new();

    // Get the hash of each section up to any '-' or '=' chars
    for sec in &grid {
        let index = sec.iter().position(|&c| c == '-' || c == '=').unwrap_or(sec.len());
        let label = &sec[..index];

        // Label hash gives the box number
        let box_num = *global_map.get(label).unwrap_or(&0);

        // Check the operation
        let op_index = sec.iter().position(|&c| c == '-' || c == '=').unwrap();
        let op = sec[op_index];
        let lens = &sec[index+1..];

        // Process the label and mutate the HashMap
        process(label.to_vec(), box_num, op, &lens, &mut ret_map);
    }

    for (&box_num, labels_vec) in &ret_map {
        for (i, label) in labels_vec.iter().enumerate() {
            ret += (box_num + 1) * (i + 1) as i64 * label.1;
        }
    }
    ret
}




