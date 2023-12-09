fn done(seq: Vec<i32>) -> bool {
    for num in seq {
        if num != 0 {
            return false;
        }
    }
    return true;
}

pub fn solve1(input: Vec<String>) -> i32 {
    let mut ret: i32 = 0;

    for line in input {
        let mut sequences : Vec<Vec<i32>> = Vec::new();
        let initial_sequence: Vec<i32> = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        sequences.push(initial_sequence.clone());

        let mut next: Vec<i32> = initial_sequence.clone();
        while !done(next.clone()) {
            next = next.windows(2).map(|window| window[1] - window[0]).collect();
            sequences.push(next.clone());
        }
        ret += sequences.iter().filter_map(|x| x.last()).sum::<i32>();
    }
    ret
}


pub fn solve2(input: Vec<String>) -> i32 {
    let mut ret: i32 = 0;
    fn done(seq: Vec<i32>) -> bool {
        for num in seq {
            if num != 0 {
                return false;
            }
        }
        return true;
    }

    for line in input {
        let mut sequences : Vec<Vec<i32>> = Vec::new();
        let initial_sequence: Vec<i32> = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        sequences.push(initial_sequence.clone());

        let mut next: Vec<i32> = initial_sequence.clone();
        while !done(next.clone()) {
            next = next.windows(2).map(|window| window[1] - window[0]).collect();
            sequences.push(next.clone());
        }

        let mut num = 0;
        for seq in sequences.iter().rev() {
            num = seq[0] - num;
        }
        ret += num;
    }
    ret
}









