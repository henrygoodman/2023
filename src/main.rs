const CURRENT_DAY: i32 = 8;  // Set the current day here

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

fn read_lines(filename: PathBuf) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let buf_reader = io::BufReader::new(file);
    buf_reader.lines().collect()
}

fn time_it<F, T>(f: F) -> (T, u128)
where
    F: FnOnce() -> T,
{
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed().as_micros();
    (result, duration)
}

fn main() -> io::Result<()> {
    // Get the directory containing Cargo.toml
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let input1_path = PathBuf::from(manifest_dir.clone())
        .join("src")
        .join("inputs")
        .join(format!("day{}", CURRENT_DAY))
        .join("1.txt");
    
    let input2_path = PathBuf::from(manifest_dir.clone())
        .join("src")
        .join("inputs")
        .join(format!("day{}", CURRENT_DAY))
        .join("2.txt");

    match CURRENT_DAY {
        1 => {
            let input1 = read_lines(input1_path)?;
            let input2 = read_lines(input2_path)?;
            println!("{:?}", day1::solve1(input1));
            println!("{:?}", day1::solve2(input2));
        },
        2 => {
            let input1 = read_lines(input1_path)?;
            let input2 = read_lines(input2_path)?;
            println!("{:?}", day2::solve1(input1));
            println!("{:?}", day2::solve2(input2));
        },
        3 => {
            let input1 = read_lines(input1_path)?;
            let input2 = read_lines(input2_path)?;
            println!("{:?}", day3::solve1(input1));
            println!("{:?}", day3::solve2(input2));
        },
        4 => {
            let input1 = read_lines(input1_path)?;
            let input2 = read_lines(input2_path)?;
            println!("{:?}", day4::solve1(input1));
            println!("{:?}", day4::solve2(input2));
        },
        5 => {
            let input1 = read_lines(input1_path)?;
            let input2 = read_lines(input2_path)?;
            println!("{:?}", day5::solve1(input1));
            println!("{:?}", day5::solve2(input2));
        },
        6 => {
            let input1 = read_lines(input1_path)?;
            let input2 = read_lines(input2_path)?;
            println!("{:?}", day6::solve1(input1));
            println!("{:?}", day6::solve2(input2));
        },
        7 => {
            let input1 = read_lines(input1_path)?;
            let input2 = read_lines(input2_path)?;
            let (part1_result, part1_time) = time_it(|| day7::solve1(input1));
            println!("Part1: {:?}, took {}µs", part1_result, part1_time);
            let (part2_result, part2_time) = time_it(|| day7::solve2(input2));
            println!("Part2: {:?}, took {}µs", part2_result, part2_time);
        },
        8 => {
            let input1 = read_lines(input1_path)?;
            let input2 = read_lines(input2_path)?;
            let (part1_result, part1_time) = time_it(|| day8::solve1(input1));
            println!("Part1: {:?}, took {}µs", part1_result, part1_time);
            let (part2_result, part2_time) = time_it(|| day8::solve2(input2));
            println!("Part2: {:?}, took {}µs", part2_result, part2_time);
        },
        _ => println!("Day not implemented"),
    }

    Ok(())
}
