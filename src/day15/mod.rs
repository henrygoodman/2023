use std::collections::HashMap;

fn calculate_hash(label: &[char]) -> i64 {
    label.iter().fold(0, |hash, &c| {
        ((hash + c as i64) * 17) % 256
    })
}

struct Lens {
    label: Vec<char>,
    focal_length: i64,
}

fn process_step(step: &str, lens_map: &mut HashMap<i64, Vec<Lens>>) {
    let op_index = step.find(['-', '='].as_ref()).unwrap();
    let (label_str, operation) = step.split_at(op_index);
    let label: Vec<char> = label_str.chars().collect();
    let box_num = calculate_hash(&label);

    if let Some(op_char) = operation.chars().next() {
        match op_char {
            '=' => {
                let focal_length = operation[1..].parse::<i64>().unwrap();
                let lens = Lens { label, focal_length };
                let entry = lens_map.entry(box_num).or_insert_with(Vec::new);

                if let Some(pos) = entry.iter().position(|l| l.label == lens.label) {
                    entry[pos] = lens;
                } else {
                    entry.push(lens);
                }
            }
            '-' => {
                if let Some(entry) = lens_map.get_mut(&box_num) {
                    if let Some(pos) = entry.iter().position(|l| l.label == label) {
                        entry.remove(pos);
                    }
                }
            }
            _ => panic!("Invalid operation"),
        }
    } else {
        panic!("No operation found in step");
    }
}

pub fn solve1(input: Vec<String>) -> i64 {
    input[0].split(',')
         .map(|step| calculate_hash(&step.chars().collect::<Vec<_>>()))
         .sum()
}

pub fn solve2(input: Vec<String>) -> i64 {
    let mut lens_map: HashMap<i64, Vec<Lens>> = HashMap::new();

    for step in input[0].split(',') {
        process_step(step, &mut lens_map);
    }

    lens_map.iter().map(|(&box_num, lenses)| {
        lenses.iter().enumerate().map(|(i, lens)| {
            (box_num + 1) * (i as i64 + 1) * lens.focal_length
        }).sum::<i64>()
    }).sum()
}
