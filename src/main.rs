const CURRENT_DAY: i32 = 14;  // Set the current day here

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;

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
    (f(), start.elapsed().as_micros())
}

fn main() -> io::Result<()> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let input_path = PathBuf::from(manifest_dir.clone())
        .join("src")
        .join("inputs")
        .join(format!("day{}", CURRENT_DAY))
        .join("1.txt");

    match CURRENT_DAY {
        1 => {
            let input = read_lines(input_path)?;
            println!("{:?}", day1::solve1(input.clone()));
            println!("{:?}", day1::solve2(input.clone()));
        },
        2 => {
            let input = read_lines(input_path)?;
            println!("{:?}", day2::solve1(input.clone()));
            println!("{:?}", day2::solve2(input.clone()));
        },
        3 => {
            let input = read_lines(input_path)?;
            println!("{:?}", day3::solve1(input.clone()));
            println!("{:?}", day3::solve2(input.clone()));
        },
        4 => {
            let input = read_lines(input_path)?;
            println!("{:?}", day4::solve1(input.clone()));
            println!("{:?}", day4::solve2(input.clone()));
        },
        5 => {
            let input = read_lines(input_path)?;
            println!("{:?}", day5::solve1(input.clone()));
            println!("{:?}", day5::solve2(input.clone()));
        },
        6 => {
            let input = read_lines(input_path)?;
            println!("{:?}", day6::solve1(input.clone()));
            println!("{:?}", day6::solve2(input.clone()));
        },
        7 => {
            let input = read_lines(input_path)?;
            let (part1_result, part1_time) = time_it(|| day7::solve1(input.clone()));
            println!("Part1: {:?}, took {}µs", part1_result, part1_time);
            let (part2_result, part2_time) = time_it(|| day7::solve2(input.clone()));
            println!("Part2: {:?}, took {}µs", part2_result, part2_time);
        },
        8 => {
            let input = read_lines(input_path)?;
            let (part1_result, part1_time) = time_it(|| day8::solve1(input.clone()));
            println!("Part1: {:?}, took {}µs", part1_result, part1_time);
            let (part2_result, part2_time) = time_it(|| day8::solve2(input.clone()));
            println!("Part2: {:?}, took {}µs", part2_result, part2_time);
        },
        9 => {
            let input = read_lines(input_path)?;
            let (part1_result, part1_time) = time_it(|| day9::solve1(input.clone()));
            println!("Part1: {:?}, took {}µs", part1_result, part1_time);
            let (part2_result, part2_time) = time_it(|| day9::solve2(input.clone()));
            println!("Part2: {:?}, took {}µs", part2_result, part2_time);
        },
        10 => {
            let input = read_lines(input_path)?;
            let (part1_result, part1_time) = time_it(|| day10::solve1(input.clone()));
            println!("Part1: {:?}, took {}µs", part1_result, part1_time);
            let (part2_result, part2_time) = time_it(|| day10::solve2(input.clone()));
            println!("Part2: {:?}, took {}µs", part2_result, part2_time);
        },
        11 => {
            let input = read_lines(input_path)?;
            let (part1_result, part1_time) = time_it(|| day11::solve1(input.clone()));
            println!("Part1: {:?}, took {}µs", part1_result, part1_time);
            let (part2_result, part2_time) = time_it(|| day11::solve2(input.clone()));
            println!("Part2: {:?}, took {}µs", part2_result, part2_time);
        },
        12 => {
            let input = read_lines(input_path)?;
            let (part1_result, part1_time) = time_it(|| day12::solve1(input.clone()));
            println!("Part1: {:?}, took {}µs", part1_result, part1_time);
            let (part2_result, part2_time) = time_it(|| day12::solve2(input.clone()));
            println!("Part2: {:?}, took {}µs", part2_result, part2_time);
        },
        13 => {
            let input = read_lines(input_path)?;
            let (part1_result, part1_time) = time_it(|| day13::solve1(input.clone()));
            println!("Part1: {:?}, took {}µs", part1_result, part1_time);
            let (part2_result, part2_time) = time_it(|| day13::solve2(input.clone()));
            println!("Part2: {:?}, took {}µs", part2_result, part2_time);
        },
        14 => {
            let input = read_lines(input_path)?;
            let (part1_result, part1_time) = time_it(|| day14::solve1(input.clone()));
            println!("Part1: {:?}, took {}µs", part1_result, part1_time);
            let (part2_result, part2_time) = time_it(|| day14::solve2(input.clone()));
            println!("Part2: {:?}, took {}µs", part2_result, part2_time);
        },
        _ => println!("Day not implemented"),
    }

    Ok(())
}
