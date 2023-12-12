fn count_valid_combinations(row: &[u8], ins: &[u8]) -> usize {
    // Remove trailing '.' if present in the row (free reduction in input size as trailing . doesn't impact result)
    let row_without_trailing_dot = if row.last() == Some(&b'.') {
        &row[..row.len() - 1]
    } else {
        row
    };

    // Prepare the row for dynamic programming by adding a leading '.' (simplify boundary conditions)
    let mut cleaned_row = Vec::with_capacity(row_without_trailing_dot.len() + 1);
    cleaned_row.push(b'.');
    cleaned_row.extend_from_slice(row_without_trailing_dot);

    // Initialize state vectors for dynamic programming
    // i.e. we will iteratively determine the number of valid states based on the state prior
    let (mut current_state, mut next_state) = (vec![0; cleaned_row.len() + 1], vec![0; cleaned_row.len() + 1]);

    // Initialize the initial state
    current_state[0] = 1;

    // Set up initial state based on filled cells in the row
    for i in 1..cleaned_row.len() {
        if cleaned_row[i] != b'#' {
            current_state[i] = 1;
        } else {
            break;
        }
    }

    // Iterate over the instructions (block sizes)
    for &block_size in ins {
        let block_size = block_size as usize;
        let mut group_size = 0;

        // DP to compute the new state
        for (i, &cell) in cleaned_row.iter().enumerate() {
            if cell == b'.' {
                group_size = 0;
            } else {
                group_size += 1;
            }

            if cell != b'#' {
                next_state[i + 1] += next_state[i];
            }

            // If we have reached end of block size, check if valid (check if the start-1 point allows us to make a .)
            // If a valid block, then we can incr the count
            if group_size >= block_size && cleaned_row[i - block_size] != b'#' {
                next_state[i + 1] += current_state[i - block_size];
            }
        }

        // Swap state vectors for the next iteration (just move the next_state into the current_state for our next iter)
        current_state.iter_mut().for_each(|x| *x = 0);
        (current_state, next_state) = (next_state, current_state);
    }

    // Return the total valid combinations
    current_state[cleaned_row.len()]
}

pub fn solve1(input: Vec<String>) -> i128 {
    let mut ret: i128 = 0;
    for line in &input {
        let mut parts = line.split_whitespace();
        let row = parts.next().unwrap().chars().collect::<String>();
        let ins = parts
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect::<Box<_>>();

        let count = count_valid_combinations(row.as_bytes(), ins.as_ref());
        ret += count as i128;
    }
    ret
}

pub fn solve2(input: Vec<String>) -> i128 {
    let mut ret: i128 = 0;
    for line in &input {
        let mut parts = line.split_whitespace();
        let row = parts.next().unwrap().chars().collect::<String>();
        let ins = parts
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();

        // Expand 'row' 5 times with '?' as separator
        let expanded_row = format!("{}?{}?{}?{}?{}", row, row, row, row, row);
        
        // Expand 'ins' 5 times without a separator
        let expanded_ins = ins.iter().cloned().cycle().take(5 * ins.len()).collect::<Vec<u8>>();

        let count = count_valid_combinations(expanded_row.as_bytes(), &expanded_ins);
        ret += count as i128;
    }
    ret
}
