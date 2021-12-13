use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug)]
enum Fold {
  X(usize),
  Y(usize),
}

#[derive(Debug)]
struct Board {
  points: HashMap<(usize, usize), usize>,
  folds: VecDeque<Fold>,
}

fn parse_input(input: String) -> Board {
  let (points_str, folds_str) = input.split_once("\n\n").unwrap();

  let points = points_str
    .split('\n')
    .map(|coords| coords.split_once(',').unwrap())
    .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
    .map(|coords| (coords, 1))
    .collect();

  let folds = folds_str
    .split('\n')
    .map(|fold| fold.split_once('=').unwrap())
    .map(|split| match split {
      ("fold along x", n) => Fold::X(n.parse().unwrap()),
      ("fold along y", n) => Fold::Y(n.parse().unwrap()),
      _ => panic!("Parse error"),
    })
    .collect();

  return Board { points, folds };
}

fn fold_once(board: &mut Board) {
  let fold = board.folds.pop_front().unwrap();
  match fold {
    Fold::X(fold_x) => {
      for ((x, y), value) in board.points.clone() {
        if x < fold_x {
          continue;
        }
        let d = x - fold_x;
        board.points.insert(
          (x - (2 * d), y),
          board.points.get(&(x - (2 * d), y)).unwrap_or(&0) + value,
        );
        board.points.remove(&(x, y));
      }
    }
    Fold::Y(fold_y) => {
      for ((x, y), value) in board.points.clone() {
        if y < fold_y {
          continue;
        }
        let d = y - fold_y;
        board.points.insert(
          (x, y - (2 * d)),
          board.points.get(&(x, y - (2 * d))).unwrap_or(&0) + value,
        );
        board.points.remove(&(x, y));
      }
    }
  }
}

fn render_board(Board { points, .. }: &Board) {
  let &max_x = points.keys().map(|(x, _)| x).max().unwrap();
  let &max_y = points.keys().map(|(_, y)| y).max().unwrap();

  for y in 0..=max_y {
    for x in 0..=max_x {
      let c = match points.get(&(x, y)) {
        Some(_) => '#',
        None => ' ',
      };

      print!("{}", c);
    }
    print!("\n")
  }
  print!("\n");
}

pub fn part1(input: String) -> usize {
  let mut board = parse_input(input);

  fold_once(&mut board);

  return board.points.len();
}

pub fn part2(input: String) -> usize {
  let mut board = parse_input(input);

  while board.folds.len() > 0 {
    fold_once(&mut board);
  }

  render_board(&board);

  return 42;
}
