use std::collections::HashMap;
use std::collections::HashSet;

type Board = HashMap<(isize, isize), usize>;

fn parse_input(input: String) -> Board {
  let mut map = HashMap::default();
  for (y, row) in input.split('\n').enumerate() {
    for (x, c) in row.chars().enumerate() {
      map.insert(
        (x.try_into().unwrap(), y.try_into().unwrap()),
        c.to_digit(10).unwrap().try_into().unwrap(),
      );
    }
  }
  return map;
}

fn get_adjacent((x, y): (isize, isize)) -> Vec<(isize, isize)> {
  return vec![
    (x - 1, y - 1),
    (x + 0, y - 1),
    (x + 1, y - 1),
    (x - 1, y + 0),
    (x + 1, y + 0),
    (x - 1, y + 1),
    (x + 0, y + 1),
    (x + 1, y + 1),
  ];
}

fn simulate_step(board: &mut Board) -> usize {
  let mut flashed: HashSet<(isize, isize)> = HashSet::default();
  // Gather energy
  for x in 0..10 {
    for y in 0..10 {
      if let Some(value) = board.get_mut(&(x, y)) {
        *value += 1;
      }
    }
  }
  // Flash cascade!
  while board.values().any(|&v| v > 9) {
    for x in 0..10 {
      for y in 0..10 {
        if let Some(value) = board.get_mut(&(x, y)) {
          if *value > 9 {
            *value = 0;
            flashed.insert((x, y));
            for (adj_x, adj_y) in get_adjacent((x, y)) {
              if flashed.contains(&(adj_x, adj_y)) {
                continue;
              }
              if let Some(val) = board.get_mut(&(adj_x, adj_y)) {
                *val += 1;
              }
            }
          }
        }
      }
    }
  }
  return flashed.len();
}

pub fn part1(input: String) -> usize {
  let mut board = parse_input(input);
  let mut flashed = 0;
  for _ in 0..100 {
    let step_flashed = simulate_step(&mut board);
    flashed += step_flashed;
  }

  return flashed;
}

pub fn part2(input: String) -> usize {
  let mut board = parse_input(input);
  let mut i = 0;
  loop {
    i += 1;
    let step_flashed = simulate_step(&mut board);
    if step_flashed == 100 {
      return i;
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_test() {
    let sample_input = String::from(
      "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526",
    );
    assert_eq!(part1(sample_input), 1656);
  }

  #[test]
  fn part2_test() {
    let sample_input = String::from(
      "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526",
    );
    assert_eq!(part2(sample_input), 195);
  }
}
