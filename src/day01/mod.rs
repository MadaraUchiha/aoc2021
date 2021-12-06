pub fn part1(input: String) -> usize {
  let parsed_input: Vec<i32> = parse_input(input);
  return parsed_input
    .windows(2)
    .filter(|window| window[1] > window[0])
    .count()
    .try_into()
    .unwrap();
}

pub fn part2(input: String) -> usize {
  let parsed_input: Vec<i32> = parse_input(input);

  return parsed_input
    .windows(3)
    .map(|window| window.iter().sum::<i32>())
    .collect::<Vec<i32>>()
    .windows(2)
    .filter(|window| window[1] > window[0])
    .count()
    .try_into()
    .unwrap();
}

fn parse_input(input: String) -> Vec<i32> {
  return input
    .split_whitespace()
    .map(|n| n.parse().expect("Not a number"))
    .collect();
}
