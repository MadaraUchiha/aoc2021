#[derive(Debug)]
enum Operation {
    Sum,
    Product,
    Min,
    Max,
    Gt,
    Lt,
    Eq,
}

impl From<usize> for Operation {
    fn from(n: usize) -> Self {
        match n {
            0 => Self::Sum,
            1 => Self::Product,
            2 => Self::Min,
            3 => Self::Max,
            5 => Self::Gt,
            6 => Self::Lt,
            7 => Self::Eq,
            _ => panic!("Unknown operation {}", n),
        }
    }
}

#[derive(Debug)]
enum Packet {
    Literal {
        version: usize,
        value: usize,
    },
    Operator {
        op: Operation,
        version: usize,
        sub_packets: Vec<Packet>,
    },
}

#[derive(Debug)]
enum ParseContext {
    Header,
    ReadLiteral { version: usize },
    ReadOperator { version: usize, op: usize },
}

fn convert_to_binary_from_hex(hex: &str) -> String {
    hex.chars().map(to_binary).collect()
}

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}

fn parse_input(input: String) -> Packet {
    let mut binary_input = convert_to_binary_from_hex(&input);

    parse_packet(&mut binary_input)
}

fn read_number_mut(string: &mut String, digits: usize) -> usize {
    usize::from_str_radix(&string.drain(..digits).collect::<String>(), 2).unwrap()
}

fn parse_packet(packet: &mut String) -> Packet {
    let mut stack = vec![ParseContext::Header];

    while let Some(current_context) = stack.pop() {
        match current_context {
            ParseContext::Header => {
                let version = read_number_mut(packet, 3);
                let packet_type = read_number_mut(packet, 3);
                match packet_type {
                    4 => stack.push(ParseContext::ReadLiteral { version }),
                    op => stack.push(ParseContext::ReadOperator { version, op }),
                }
            }
            ParseContext::ReadLiteral { version } => {
                let mut should_continue = true;
                let mut value = 0;
                while should_continue {
                    should_continue = read_number_mut(packet, 1) == 1;
                    value <<= 4;
                    value += read_number_mut(packet, 4);
                }

                return Packet::Literal { version, value };
            }
            ParseContext::ReadOperator { version, op } => match read_number_mut(packet, 1) {
                0 => {
                    let total_read_length = read_number_mut(packet, 15);
                    let mut subpackets_string: String = packet.drain(..total_read_length).collect();
                    let mut sub_packets: Vec<Packet> = Vec::default();
                    while subpackets_string.len() > 0 {
                        sub_packets.push(parse_packet(&mut subpackets_string));
                    }
                    return Packet::Operator {
                        op: Operation::from(op),
                        version,
                        sub_packets,
                    };
                }
                1 => {
                    let total_subpackets_length = read_number_mut(packet, 11);
                    let mut sub_packets: Vec<Packet> = Vec::default();
                    for _ in 0..total_subpackets_length {
                        sub_packets.push(parse_packet(packet));
                    }

                    return Packet::Operator {
                        op: Operation::from(op),
                        version,
                        sub_packets,
                    };
                }
                n => panic!("Unfamiliar length ID: {}", n),
            },
        }
    }

    panic!("Reached end of stack but did not return a packet");
}

fn count_versions(packet: &Packet) -> usize {
    match packet {
        Packet::Literal { version, .. } => *version,
        Packet::Operator {
            version,
            sub_packets,
            ..
        } => *version + sub_packets.iter().map(count_versions).sum::<usize>(),
    }
}

fn eval_packet(packet: &Packet) -> usize {
    match packet {
        Packet::Literal { value, .. } => *value,
        Packet::Operator {
            op, sub_packets, ..
        } => match op {
            Operation::Sum => sub_packets.iter().map(eval_packet).sum(),
            Operation::Product => sub_packets.iter().map(eval_packet).product(),
            Operation::Min => sub_packets.iter().map(eval_packet).min().unwrap(),
            Operation::Max => sub_packets.iter().map(eval_packet).max().unwrap(),
            Operation::Gt => {
                let a = eval_packet(&sub_packets[0]);
                let b = eval_packet(&sub_packets[1]);

                if a > b { 1 } else { 0 }
            }
            Operation::Lt => {
                let a = eval_packet(&sub_packets[0]);
                let b = eval_packet(&sub_packets[1]);

                if a < b { 1 } else { 0 }
            }
            Operation::Eq => {
                let a = eval_packet(&sub_packets[0]);
                let b = eval_packet(&sub_packets[1]);

                if a == b { 1 } else { 0 }
            }
        },
    }
}

pub fn part1(input: String) -> usize {
    let packet_prime = parse_input(input);
    count_versions(&packet_prime)
}

pub fn part2(input: String) -> usize {
    let packet_prime = parse_input(input);
    eval_packet(&packet_prime)
}
