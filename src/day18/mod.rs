use anyhow::{anyhow, Result};
use std::iter::Peekable;
use std::ops::Add;
use std::str::Chars;
use std::str::FromStr;

struct Sfn(Vec<(usize, usize)>);
impl Sfn {
  pub fn reduce(&mut self) {
    let sfn = &mut self.0;
    loop {
      // Explode
      if let Some(idx) = sfn.iter().position(|&(depth, _)| depth == 5) {
        let (_, a) = sfn.remove(idx);
        let (_, b) = sfn.remove(idx);

        sfn.insert(idx, (4, 0));

        if idx > 0 {
          if let Some((_, value)) = sfn.get_mut(idx - 1) {
            *value += a;
          }
        }

        if let Some((_, value)) = sfn.get_mut(idx + 1) {
          *value += b;
        }

        continue;
      }

      // Split
      if let Some(idx) = sfn.iter().position(|&(_, value)| value > 9) {
        let (depth, value) = sfn.remove(idx);
        let a = value / 2;
        let b = if value % 2 == 0 { a } else { a + 1 };
        sfn.insert(idx, (depth + 1, b));
        sfn.insert(idx, (depth + 1, a));
        continue;
      }

      return;
    }
  }

  pub fn magnitude(mut self) -> usize {
    let sfn = &mut self.0;
    for d in (1..=4).rev() {
      while let Some(idx) = sfn.iter().position(|&(depth, _)| depth == d) {
        let (_, left) = sfn.remove(idx);
        let (_, right) = sfn.remove(idx);
        sfn.insert(idx, (d - 1, 3 * left + 2 * right));
      }
    }
    sfn[0].1
  }
}

impl<'a> Add for &'a Sfn {
  type Output = Sfn;
  fn add(self, rhs: Self) -> Sfn {
    let mut sfn_vec = Vec::new();

    sfn_vec.extend(self.0.iter().map(|&(depth, value)| (depth + 1, value)));
    sfn_vec.extend(rhs.0.iter().map(|&(depth, value)| (depth + 1, value)));

    let mut new_sfn = Sfn(sfn_vec);
    new_sfn.reduce();
    new_sfn
  }
}

impl FromStr for Sfn {
  type Err = anyhow::Error;
  fn from_str(input: &str) -> Result<Self> {
    let mut parser = SfnParser::new(input);
    parser.parse_pair()?;
    Ok(Sfn(parser.sfn))
  }
}

struct SfnParser<'input> {
  input: Peekable<Chars<'input>>,
  depth: usize,
  sfn: Vec<(usize, usize)>,
}

impl<'input> SfnParser<'input> {
  pub fn new(input: &'input str) -> Self {
    SfnParser {
      input: input.chars().peekable(),
      depth: 0,
      sfn: Vec::new(),
    }
  }

  pub fn parse_pair(&mut self) -> Result<()> {
    self.expect('[')?;
    self.depth += 1;
    self.parse_element()?;
    self.expect(',')?;
    self.parse_element()?;
    self.expect(']')?;
    self.depth -= 1;

    Ok(())
  }

  fn expect(&mut self, t: char) -> Result<()> {
    match self.input.next() {
      Some(c) if c == t => Ok(()),
      Some(c) => Err(anyhow!("Expected {} but got {}", t, c)),
      None => Err(anyhow!("Unexpected end of input")),
    }
  }

  fn parse_element(&mut self) -> Result<()> {
    if matches!(self.input.peek(), Some('[')) {
      self.parse_pair()
    } else {
      let n = self
        .input
        .next()
        .ok_or(anyhow!("Unexpected end of input"))?
        .to_digit(10)
        .ok_or(anyhow!("Unable to convert to digit"))?;

      self.sfn.push((self.depth, n as usize));

      Ok(())
    }
  }
}

pub fn part1(input: String) -> usize {
  let sfns = input
    .lines()
    .map(Sfn::from_str)
    .collect::<Result<Vec<_>>>()
    .expect("Parse error");

  let final_sfn = sfns.into_iter().reduce(|a, b| (&a + &b)).unwrap();

  final_sfn.magnitude()
}

pub fn part2(input: String) -> usize {
  let sfns = input
    .lines()
    .map(Sfn::from_str)
    .collect::<Result<Vec<_>>>()
    .expect("Parse error");

  sfns
    .iter()
    .map(|a| {
      sfns
        .iter()
        .map(|b| (a.clone() + b.clone()).magnitude())
        .max()
    })
    .max()
    .unwrap()
    .unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn magnitude() {
    let sfn = "[[1,2],[[3,4],5]]".parse::<Sfn>().unwrap();

    assert_eq!(sfn.magnitude(), 143);
  }
}
