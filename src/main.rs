use std::fs;
use std::time::Instant;
use structopt::StructOpt;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day20;
mod day21;

#[derive(StructOpt)]
struct Cli {
    day: i32,
    part: i32,
}

fn main() {
    let Cli { day, part } = Cli::from_args();
    let now = Instant::now();
    let file_name = format!("src/day{:02}/input.txt", day);
    let input_file = fs::read_to_string(file_name).expect("Failed to read file");
    let file_read_time = now.elapsed();
    let result = match (day, part) {
        (1, 1) => Ok(day01::part1(input_file)),
        (1, 2) => Ok(day01::part2(input_file)),
        (2, 1) => Ok(day02::part1(input_file).try_into().unwrap()),
        (2, 2) => Ok(day02::part2(input_file).try_into().unwrap()),
        (3, 1) => Ok(day03::part1(input_file)),
        (3, 2) => Ok(day03::part2(input_file)),
        (4, 1) => Ok(day04::part1(input_file)),
        (4, 2) => Ok(day04::part2(input_file)),
        (5, 1) => Ok(day05::part1(input_file)),
        (5, 2) => Ok(day05::part2(input_file)),
        (6, 1) => Ok(day06::part1(input_file)),
        (6, 2) => Ok(day06::part2(input_file)),
        (7, 1) => Ok(day07::part1(input_file)),
        (7, 2) => Ok(day07::part2(input_file)),
        (8, 1) => Ok(day08::part1(input_file)),
        (8, 2) => Ok(day08::part2(input_file)),
        (9, 1) => Ok(day09::part1(input_file)),
        (9, 2) => Ok(day09::part2(input_file)),
        (10, 1) => Ok(day10::part1(input_file)),
        (10, 2) => Ok(day10::part2(input_file)),
        (11, 1) => Ok(day11::part1(input_file)),
        (11, 2) => Ok(day11::part2(input_file)),
        (12, 1) => Ok(day12::part1(input_file)),
        (12, 2) => Ok(day12::part2(input_file)),
        (13, 1) => Ok(day13::part1(input_file)),
        (13, 2) => Ok(day13::part2(input_file)),
        (14, 1) => Ok(day14::part1(input_file)),
        (14, 2) => Ok(day14::part2(input_file)),
        (15, 1) => Ok(day15::part1(input_file)),
        (15, 2) => Ok(day15::part2(input_file)),
        (16, 1) => Ok(day16::part1(input_file)),
        (16, 2) => Ok(day16::part2(input_file)),
        (17, 1) => Ok(day17::part1(input_file)),
        (17, 2) => Ok(day17::part2(input_file)),
        (18, 1) => Ok(day18::part1(input_file)),
        (18, 2) => Ok(day18::part2(input_file)),
        (20, 1) => Ok(day20::part1(input_file)),
        (20, 2) => Ok(day20::part2(input_file)),
        (21, 1) => Ok(day21::part1(input_file)),
        (21, 2) => Ok(day21::part2(input_file)),
        _ => Err("Unsupported solution"),
    };

    println!(
        "Day {} Part {} - Result is: {} -- Took {:?} (file read took {:?})",
        day,
        part,
        result.expect(&format!("day {} part {} not implemented", day, part)),
        now.elapsed(),
        file_read_time,
    );
}
