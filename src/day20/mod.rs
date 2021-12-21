use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

struct Game {
  bitmap: [u16; 512],
  outside_value: u16,
  image: HashMap<(isize, isize), u16>,
}

impl Game {
  fn get_value(&self, (x, y): (isize, isize)) -> usize {
    let mut res = 0;
    for cy in y - 1..=y + 1 {
      for cx in x - 1..=x + 1 {
        res <<= 1;
        res += self.image.get(&(cx, cy)).unwrap_or(&self.outside_value)
      }
    }
    res.into()
  }

  pub fn tick(&self) -> Self {
    let mut new_image = HashMap::new();
    let next_outside_value = if self.outside_value == 1 {
      self.bitmap[9]
    } else {
      self.bitmap[0]
    };

    let (min_x, _) = self.image.keys().min_by_key(|(x, _)| x).unwrap();
    let (max_x, _) = self.image.keys().max_by_key(|(x, _)| x).unwrap();
    let (_, min_y) = self.image.keys().min_by_key(|(_, y)| y).unwrap();
    let (_, max_y) = self.image.keys().max_by_key(|(_, y)| y).unwrap();

    for x in min_x - 1..=max_x + 1 {
      for y in min_y - 1..=max_y + 1 {
        let value = self.get_value((x, y));
        new_image.insert((x, y), self.bitmap[value]);
      }
    }

    Self {
      image: new_image,
      outside_value: next_outside_value,
      ..*self
    }
  }
  pub fn count_lit(&self) -> usize {
    self.image.values().filter(|&&v| v == 1).count()
  }
}

impl FromStr for Game {
  type Err = anyhow::Error;
  fn from_str(input: &str) -> Result<Game> {
    let (bitmap_str, image_str) = input
      .split_once("\n\n")
      .ok_or(anyhow!("First split failure"))?;
    debug_assert_eq!(bitmap_str.len(), 512);
    let mut bitmap = [0; 512];
    for (i, c) in bitmap_str.char_indices() {
      if c == '#' {
        bitmap[i] = 1;
      }
    }

    let mut image = HashMap::new();
    for (y, row) in image_str.lines().enumerate() {
      for (x, c) in row.char_indices() {
        let v = match c {
          '#' => Ok(1),
          '.' => Ok(0),
          _ => Err(anyhow!("Unfamiliar character {}", c)),
        }?;

        image.insert((x as isize, y as isize), v);
      }
    }

    Ok(Game {
      bitmap,
      image,
      outside_value: 0,
    })
  }
}

impl Display for Game {
  fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    let (min_x, _) = self.image.keys().min_by_key(|(x, _)| x).unwrap();
    let (max_x, _) = self.image.keys().max_by_key(|(x, _)| x).unwrap();
    let (_, min_y) = self.image.keys().min_by_key(|(_, y)| y).unwrap();
    let (_, max_y) = self.image.keys().max_by_key(|(_, y)| y).unwrap();

    Ok(for y in *min_y..=*max_y {
      for x in *min_x..=*max_x {
        write!(fmt, "{}", if self.image[&(x, y)] == 1 { '#' } else { '.' })?;
      }
      write!(fmt, "\n")?;
    })
  }
}

pub fn part1(input: String) -> usize {
  let game: Game = input.parse().unwrap();

  game.tick().tick().count_lit()
}

pub fn part2(input: String) -> usize {
  let game: Game = input.parse().unwrap();

  (0..50)
    .into_iter()
    .fold(game, |acc, _| acc.tick())
    .count_lit()
}
