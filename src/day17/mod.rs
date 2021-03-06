use aoc2021::Vec2;
use std::collections::HashSet;
use std::ops::RangeInclusive;
use std::str::FromStr;

struct Probe {
  velocity: Vec2,
  position: Vec2,
}

impl Probe {
  pub fn new(initial_velocity: Vec2) -> Self {
    Probe {
      velocity: initial_velocity,
      position: Vec2(0, 0),
    }
  }

  pub fn tick(&self) -> Self {
    let mut new_velocity = self.velocity.clone() + Vec2(-1, -1);
    if new_velocity.0 < 0 {
      new_velocity.0 = 0;
    }
    Probe {
      position: self.position.clone() + self.velocity.clone(),
      velocity: new_velocity,
    }
  }

  pub fn missed(&self, zone: &LandingZone) -> bool {
    self.position.0 > *zone.x.end() || self.position.1 < *zone.y.start()
  }

  pub fn landed(&self, zone: &LandingZone) -> bool {
    zone.x.contains(&self.position.0) && zone.y.contains(&self.position.1)
  }
}

struct LandingZone {
  x: RangeInclusive<isize>,
  y: RangeInclusive<isize>,
}

impl FromStr for LandingZone {
  type Err = &'static str;
  fn from_str(input: &str) -> Result<Self, <Self as FromStr>::Err> {
    let p = |n: &str| n.parse().or(Err("Failed to parse number"));
    let (_, ranges) = input.split_once(": ").ok_or("Split error")?;
    let (x_range, y_range) = ranges.split_once(", ").ok_or("Split error")?;
    let (min_x, max_x) = x_range[2..]
      .split_once("..")
      .ok_or("Failed to parse x range")?;
    let (min_y, max_y) = y_range[2..]
      .split_once("..")
      .ok_or("Failed to parse y range")?;

    Ok(Self {
      x: (p(min_x)?..=p(max_x)?),
      y: (p(min_y)?..=p(max_y)?),
    })
  }
}

fn find_sum_of_consecutive(n: isize) -> isize {
  (n * (n + 1)) / 2
}

fn find_all_viable_trajectories(zone: LandingZone) -> HashSet<Vec2> {
  let max_vx = *zone.x.end() + 1;
  let max_vy = -*zone.y.start();

  let mut result = HashSet::default();

  for vx in 0..=max_vx {
    for vy in -max_vy..=max_vy {
      let vel = Vec2(vx, vy);
      let mut probe = Probe::new(vel.clone());
      loop {
        probe = probe.tick();
        if probe.landed(&zone) {
          result.insert(vel);
          break;
        }
        if probe.missed(&zone) {
          break;
        }
      }
    }
  }

  result
}

pub fn part1(input: String) -> usize {
  let landing_zone = LandingZone::from_str(&input).unwrap();
  let &min_y = landing_zone.y.start();

  find_sum_of_consecutive(-1 - min_y).try_into().unwrap()
}

pub fn part2(input: String) -> usize {
  let landing_zone = LandingZone::from_str(&input).unwrap();

  let initial_velocities = find_all_viable_trajectories(landing_zone);

  initial_velocities.len()
}
