use std::collections::HashMap;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Dirac {
  // (pos, score)
  p1: (usize, usize),
  p2: (usize, usize),
}

impl Dirac {
  pub fn play_till<TDie: Die>(&mut self, target: usize, die: &mut TDie) -> usize {
    let mut turn = 0;
    while self.p1.1 < target && self.p2.1 < target {
      let advance = die.roll() + die.roll() + die.roll();
      let p = if turn % 2 == 0 {
        &mut self.p1
      } else {
        &mut self.p2
      };
      let new_pos = (p.0 + advance) % 10;
      let new_score = p.1 + new_pos + 1; // positions are 0 idx, scores 1 idx

      p.0 = new_pos;
      p.1 = new_score;
      turn += 1;
    }
    if self.p1.1 >= target {
      self.p2.1
    } else {
      self.p1.1
    }
  }

  pub fn quantum_play_till<TDie: QuantumDie>(self, target: usize, die: &TDie) -> QuantumWinState {
    let mut cache = HashMap::new();

    let mut roll_sums = HashMap::new();
    for roll1 in die.roll() {
      for roll2 in die.roll() {
        for roll3 in die.roll() {
          *roll_sums.entry(roll1 + roll2 + roll3).or_default() += 1;
        }
      }
    }
    let roll_sums: Vec<_> = roll_sums.into_iter().collect();

    fn quantum_play_rec(
      game: Dirac,
      roll_sums: &[(usize, usize)],
      target: usize,
      cache: &mut HashMap<Dirac, QuantumWinState>,
    ) -> QuantumWinState {
      if let Some(wins) = cache.get(&game) {
        return wins.clone();
      }
      let mut p1_wins = 0;
      let mut p2_wins = 0;

      for (roll, roll_counts) in roll_sums {
        let mut after_p1 = game.clone();
        after_p1.p1.0 = (after_p1.p1.0 + roll) % 10;
        after_p1.p1.1 += after_p1.p1.0 + 1;

        if after_p1.p1.1 >= target {
          p1_wins += roll_counts;
          continue;
        }

        for (roll, roll2_counts) in roll_sums {
          let cumulative_roll_count = roll_counts * roll2_counts;
          let mut after_p2 = after_p1.clone();
          after_p2.p2.0 = (after_p2.p2.0 + roll) % 10;
          after_p2.p2.1 += after_p2.p2.0 + 1;

          if after_p2.p2.1 >= target {
            p2_wins += cumulative_roll_count;
            continue;
          }

          let rest = quantum_play_rec(after_p2, roll_sums, target, cache);
          p1_wins += rest.p1_wins * cumulative_roll_count;
          p2_wins += rest.p2_wins * cumulative_roll_count;
        }
      }

      let r = QuantumWinState { p1_wins, p2_wins };

      cache.insert(game, r.clone());

      r
    }

    quantum_play_rec(self, roll_sums.as_slice(), target, &mut cache)
  }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct QuantumWinState {
  p1_wins: usize,
  p2_wins: usize,
}

trait Die {
  fn roll(&mut self) -> usize;
  fn count_rolls(&self) -> usize;
}

trait QuantumDie {
  fn roll(&self) -> &[usize];
}

struct DeterministicDie(usize);

impl DeterministicDie {
  pub fn new() -> Self {
    DeterministicDie(0)
  }
}

impl Die for DeterministicDie {
  fn roll(&mut self) -> usize {
    self.0 += 1;
    self.0
  }
  fn count_rolls(&self) -> usize {
    self.0
  }
}

struct QuantumDiracDie;

impl QuantumDie for QuantumDiracDie {
  fn roll(&self) -> &[usize] {
    &[1, 2, 3]
  }
}

fn parse_input(input: String) -> Option<Dirac> {
  let (p1_str, p2_str) = input.split_once('\n')?;
  let p1_1idx: usize = p1_str.chars().last()?.to_digit(10)?.try_into().ok()?;
  let p2_1idx: usize = p2_str.chars().last()?.to_digit(10)?.try_into().ok()?;

  Some(Dirac {
    p1: (p1_1idx - 1, 0),
    p2: (p2_1idx - 1, 0),
  })
}

pub fn part1(input: String) -> usize {
  let mut game = parse_input(input).unwrap();
  let mut die = DeterministicDie::new();

  let loser_score = game.play_till(1000, &mut die);

  loser_score * die.count_rolls()
}

pub fn part2(input: String) -> usize {
  let game = parse_input(input).unwrap();
  let die = QuantumDiracDie;

  let res = game.quantum_play_till(21, &die);
  usize::max(res.p1_wins, res.p2_wins)
}
