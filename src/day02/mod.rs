use std::str::FromStr;

enum Instruction {
    Forwards(i32),
    Up(i32),
    Down(i32),
}

#[derive(Debug, Clone)]
struct Coordinates {
    x: i32,
    y: i32,
    aim: i32,
}

fn parse_input(input: String) -> Vec<Instruction> {
    return input
        .split_terminator('\n')
        .map(Instruction::from_str)
        .collect::<Result<Vec<_>, _>>() // Cast vector of results to results of vector, like Promise.all()
        .expect("Parsing failed!");
}

#[derive(Debug, Clone)]
struct InstructionParseError;

impl FromStr for Instruction {
    type Err = InstructionParseError;
    fn from_str(input: &str) -> std::result::Result<Self, <Self as FromStr>::Err> {
        match input.split_ascii_whitespace().collect::<Vec<&str>>()[..] {
            ["forward", n_str] => n_str
                .parse()
                .map(|n| Instruction::Forwards(n))
                .map_err(|_| InstructionParseError),
            ["up", n_str] => n_str
                .parse()
                .map(|n| Instruction::Up(n))
                .map_err(|_| InstructionParseError),
            ["down", n_str] => n_str
                .parse()
                .map(|n| Instruction::Down(n))
                .map_err(|_| InstructionParseError),
            _ => Err(InstructionParseError),
        }
    }
}

pub fn part1(input: String) -> i32 {
    let instructions = parse_input(input);
    let coords = Coordinates { x: 0, y: 0, aim: 0 };

    let Coordinates { x, y, aim: _ } =
        instructions
            .into_iter()
            .fold(coords, |acc, instruction| match instruction {
                Instruction::Forwards(n) => Coordinates {
                    x: acc.x + n,
                    ..acc
                },
                Instruction::Up(n) => Coordinates {
                    y: acc.y - n,
                    ..acc
                },
                Instruction::Down(n) => Coordinates {
                    y: acc.y + n,
                    ..acc
                },
            });

    return x * y;
}

pub fn part2(input: String) -> i32 {
    let instructions = parse_input(input);
    let coords = Coordinates { x: 0, y: 0, aim: 0 };

    let Coordinates { x, y, aim: _ } =
        instructions
            .into_iter()
            .fold(coords, |acc, instruction| match instruction {
                Instruction::Forwards(n) => Coordinates {
                    x: acc.x + n,
                    y: acc.y + (acc.aim * n),
                    ..acc
                },
                Instruction::Up(n) => Coordinates {
                    aim: acc.aim - n,
                    ..acc
                },
                Instruction::Down(n) => Coordinates {
                    aim: acc.aim + n,
                    ..acc
                },
            });

    return x * y;
}
