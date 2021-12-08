use std::collections::HashSet;
use std::str::FromStr;

#[derive(Clone, Debug)]
struct Display {
  wire_signals: Vec<HashSet<char>>,
  display: Vec<HashSet<char>>,
}

impl FromStr for Display {
  type Err = String;
  fn from_str(line: &str) -> Result<Self, <Self as FromStr>::Err> {
    match line.split_once(" | ") {
      None => Err(format!("Failed to parse {}", line)),
      Some((wire_signals, display)) => {
        let parsed_digits = wire_signals
          .split(' ')
          .map(|digit| digit.chars().into_iter().collect())
          .collect();
        let parsed_display = display
          .split(' ')
          .map(|digit| digit.chars().into_iter().collect())
          .collect();

        return Ok(Display {
          wire_signals: parsed_digits,
          display: parsed_display,
        });
      }
    }
  }
}

fn deduce_number(
  Display {
    wire_signals,
    display,
  }: Display,
) -> usize {
  let mut mapping: [HashSet<char>; 10] = Default::default();

  for pattern in wire_signals.clone() {
    match pattern.len() {
      2 => mapping[1] = pattern,
      3 => mapping[7] = pattern,
      4 => mapping[4] = pattern,
      7 => mapping[8] = pattern,
      _ => (),
    }
  }

  for pattern in wire_signals.clone() {
    if pattern.len() != 5 {
      continue;
    }
    if mapping[1].difference(&pattern).count() == 0 {
      mapping[3] = pattern;
    } else if pattern.difference(&mapping[4]).count() == 2 {
      mapping[5] = pattern;
    } else if pattern.difference(&mapping[4]).count() == 3 {
      mapping[2] = pattern;
    }
  }

  for pattern in wire_signals {
    if pattern.len() == 6 {
      if mapping[1].difference(&pattern).count() == 1 {
        mapping[6] = pattern;
      } else if mapping[5].difference(&pattern).count() == 0 {
        mapping[9] = pattern;
      } else {
        mapping[0] = pattern;
      }
    }
  }

  let unscrambled_number = display
    .iter()
    .rev() // I want increasing powers!
    .enumerate()
    .fold(0, |acc, (i, scrambled_digit)| {
      let digit = mapping
        .iter()
        .position(|potential_digit| potential_digit == scrambled_digit)
        .unwrap();

      return acc + digit * usize::pow(10, i as u32);
    });

  return unscrambled_number;
}

fn parse_input(input: String) -> Vec<Display> {
  return input
    .split('\n')
    .map(Display::from_str)
    .collect::<Result<_, _>>()
    .unwrap();
}

pub fn part1(input: String) -> usize {
  let displays = parse_input(input);

  return displays
    .iter()
    .map(|Display { display, .. }| {
      display
        .iter()
        .filter(|digit| [2, 3, 4, 7].contains(&digit.len()))
        .count()
    })
    .sum();
}

pub fn part2(input: String) -> usize {
  let displays = parse_input(input);

  return displays
    .iter()
    .map(|display| deduce_number(display.clone()))
    .sum();
}
