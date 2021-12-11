use std::fs;
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

#[derive(StructOpt)]
struct Cli {
    day: i32,
    part: i32,
}

fn main() {
    let Cli { day, part } = Cli::from_args();
    let file_name = format!("src/day{:02}/input.txt", day);
    let input_file = fs::read_to_string(file_name).expect("Failed to read file");
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
        _ => Err("Unsupported solution"),
    };

    println!(
        "Day {} Part {} - Result is: {}",
        day,
        part,
        result.expect(&format!("day {} part {} not implemented", day, part))
    );
}
