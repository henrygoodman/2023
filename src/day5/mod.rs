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

pub fn solve2(input: Vec<String>) -> i128 {
    let seed_pairs: Vec<i128> = input.iter().nth(0).unwrap().split(':').nth(1).unwrap().split_whitespace().map(|x| x.parse::<i128>().unwrap()).collect();
    let max_range_size: i128 = 150_000_000; // Limit for the size of each range

    let mut min_values: Vec<i128> = Vec::new();

    for seed_pair in seed_pairs.chunks(2) {
        let range_start = seed_pair[0];
        let range_end = seed_pair[0] + seed_pair[1];
        let mut start = range_start;

        while start < range_end {
            let end = std::cmp::min(start + max_range_size, range_end);
            let mut seeds: Vec<i128> = (start..end).collect();
            let mut transformations: Vec<(usize, i128)> = Vec::new();
            let mut first_pass: bool = true;

            for (idx, line) in input.iter().enumerate().skip(1) {
                if line != "" && !line.contains("map") {
                    let ranges: Vec<i128> = line.split_whitespace().map(|x| x.parse::<i128>().unwrap()).collect();
                    for (jdx, seed) in seeds.iter().enumerate() {
                        if *seed >= ranges[1] && *seed < (ranges[1] + ranges[2]) {
                            transformations.push((jdx, *seed - ranges[1] + ranges[0]));
                        }
                    }
                } else if idx > 2 && line == "" {
                    for (index, new_value) in transformations.drain(..) {
                        seeds[index] = new_value;
                    }
                }
            }
            for (index, new_value) in transformations.drain(..) {
                seeds[index] = new_value;
            }
            min_values.push(*seeds.iter().min().unwrap());
            start = end;
        }
    }

    *min_values.iter().min().unwrap()
}








