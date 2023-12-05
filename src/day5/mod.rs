pub fn solve1(input: Vec<String>) -> i128 {
    let mut seeds: Vec<i128> = input.iter().nth(0).unwrap().split(':').nth(1).unwrap().split_whitespace().map(|x| x.parse::<i128>().unwrap()).collect();
    let mut new_seeds: Vec<i128> = seeds.clone();

    for (idx, line) in input.iter().enumerate() {
        if idx > 0 && line != "" && !line.contains("map") {
            for (jdx, seed) in seeds.iter().enumerate() {
                let ranges: Vec<i128> = line.split_whitespace().map(|x| x.parse::<i128>().unwrap()).collect();

                // Transform if seed is within the range
                if *seed >= ranges[1] && *seed < (ranges[1] + ranges[2]) {
                    new_seeds[jdx] = *seed - ranges[1] + ranges[0];
                }
            }
        }
        // Update seeds after the end of each section
        else if idx > 2 && line == "" {
            seeds = new_seeds.clone();
        }
    }
    // Return the minimum seed transformation
    *new_seeds.iter().min().unwrap()
}

use rayon::prelude::*;

pub fn solve2(input: Vec<String>) -> i128 {
    let seed_pairs: Vec<i128> = input.iter().nth(0).unwrap().split(':').nth(1).unwrap().split_whitespace().map(|x| x.parse::<i128>().unwrap()).collect();
    let max_range_size: i128 = 500_000_000 / seed_pairs.len() as i128; // ~10GB RAM

    let min_values: Vec<i128> = seed_pairs.par_chunks(2).map(|seed_pair| {
        println!("Processing: {:?}", seed_pair);
        let range_start = seed_pair[0];
        let range_end = seed_pair[0] + seed_pair[1];
        let mut start = range_start;
        let mut local_min_values: Vec<i128> = Vec::new();

        while start < range_end {
            let end = std::cmp::min(start + max_range_size, range_end);
            let mut seeds: Vec<i128> = (start..end).collect();
            let mut transformations: Vec<(usize, i128)> = Vec::new();

            for (idx, line) in input.iter().enumerate().skip(1) {
                if line != "" && !line.contains("map") {
                    let ranges: Vec<i128> = line.split_whitespace().map(|x| x.parse::<i128>().unwrap()).collect();

                    for (jdx, seed) in seeds.iter().enumerate() {
                        if *seed >= ranges[1] && *seed < (ranges[1] + ranges[2]) {
                            transformations.push((jdx, *seed - ranges[1] + ranges[0]));
                        }
                    }
                } else if idx > 2 && line == "" && !transformations.is_empty() {
                    for (index, new_value) in transformations.drain(..) {
                        seeds[index] = new_value;
                    }
                }
            }
            for (index, new_value) in transformations.drain(..) {
                seeds[index] = new_value;
            }
            local_min_values.push(*seeds.iter().min().unwrap());
            start = end;
        }
        println!("Done: {:?}", seed_pair);
        *local_min_values.iter().min().unwrap()
    }).collect();

    *min_values.iter().min().unwrap()
}









