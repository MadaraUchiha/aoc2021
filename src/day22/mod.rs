use anyhow::{anyhow, Result};
use std::str::FromStr;
use aoc2021::Vec3;

#[derive(Clone)]
struct Cuboid {
  from: Vec3,
  to: Vec3,
}

impl Cuboid {
  pub fn new(from: Vec3, to: Vec3) -> Self {
    Self { from, to }
  }

  pub fn volume(&self) -> isize {
    (self.to.0 - self.from.0 + 1) * (self.to.1 - self.from.1 + 1) * (self.to.2 - self.from.2 + 1)
  }
}

struct Instruction {
  cuboid: Cuboid,
  on: bool,
}

impl FromStr for Instruction {
  type Err = anyhow::Error;
  fn from_str(input: &str) -> Result<Self> {
    fn parse_range<'a, T>(iter: &mut T) -> Result<(isize, isize)>
    where
      T: Iterator<Item = &'a str>,
    {
      let (min, max) = iter
        .next()
        .ok_or(anyhow!("Unexpected end of iteration"))?
        .split_once("..")
        .ok_or(anyhow!(".. split error"))?;

      Ok((
        isize::from_str_radix(min, 10)?,
        isize::from_str_radix(max, 10)?,
      ))
    }
    let (state, ranges) = input.split_once(' ').ok_or(anyhow!("First split error"))?;
    let mut ranges_iter = ranges.split(',').map(|part| &part[2..]);
    let (min_x, max_x) = parse_range(&mut ranges_iter)?;
    let (min_y, max_y) = parse_range(&mut ranges_iter)?;
    let (min_z, max_z) = parse_range(&mut ranges_iter)?;

    let min = Vec3(min_x, min_y, min_z);
    let max = Vec3(max_x, max_y, max_z);

    Ok(Instruction {
      cuboid: Cuboid::new(min, max),
      on: state == "on",
    })
  }
}

fn disjointed(a: &Cuboid, b: &Cuboid) -> bool {
  let any_less = |a: &Vec3, b: &Vec3| a.0 < b.0 || a.1 < b.1 || a.2 < b.2;
  any_less(&a.to, &b.from) || any_less(&b.to, &a.from)
}

fn intersect(a: &Cuboid, b: &Cuboid) -> Option<Cuboid> {
  if disjointed(a, b) {
    None
  } else {
    let cuboid = Cuboid::new(Vec3::max(&a.from, &b.from), Vec3::min(&a.to, &b.to));
    Some(cuboid)
  }
}

fn parse_input(input: String) -> Result<Vec<Instruction>> {
  input.lines().map(|l| l.parse()).collect()
}

fn get_total_lit(instructions: Vec<Instruction>) -> usize {
  let mut no_partial_overlap_instructions: Vec<Instruction> = Vec::new();

  for Instruction { cuboid, on } in instructions.iter() {
    let mut add = Vec::new();
    if *on {
      add.push(Instruction {
        cuboid: cuboid.clone(),
        on: true,
      })
    }
    for other_inst in no_partial_overlap_instructions.iter() {
      if let Some(ci) = intersect(cuboid, &other_inst.cuboid) {
        add.push(Instruction {
          cuboid: ci,
          on: !other_inst.on,
        });
      }
    }
    no_partial_overlap_instructions.extend(add);
  }

  no_partial_overlap_instructions
    .into_iter()
    .map(|Instruction { cuboid, on }| {
      let sign = if on { 1 } else { -1 };
      cuboid.volume() * sign
    })
    .sum::<isize>()
    .try_into()
    .unwrap()
}

pub fn part1(input: String) -> usize {
  let instructions = parse_input(input).unwrap();
  let min_from = Vec3(-50, -50, -50);
  let max_to = Vec3(50, 50, 50);
  let init_area = Cuboid {
    from: min_from,
    to: max_to,
  };

  let instructions_in_area = instructions
    .into_iter()
    .filter(|inst| intersect(&inst.cuboid, &init_area).is_some())
    .collect();

  get_total_lit(instructions_in_area)
}

pub fn part2(input: String) -> usize {
  let instructions = parse_input(input).unwrap();
  get_total_lit(instructions)
}
