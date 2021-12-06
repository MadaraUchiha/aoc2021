use std::fs;
use structopt::StructOpt;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

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
        _ => Err("Unsupported solution"),
    };

    println!(
        "Day {} Part {} - Result is: {}",
        day,
        part,
        result.expect(&format!("day {} part {} not implemented", day, part))
    );
}
