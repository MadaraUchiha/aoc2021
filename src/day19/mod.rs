use anyhow::{anyhow, Result};
use aoc2021::Vec3;
use core::str::FromStr;
use std::collections::HashSet;

const HEADER_OFFSET: usize = 12; // length of "--- scanner "

#[derive(Debug, Eq, Hash, Clone)]
struct Scanner {
    id: usize,
    visible: Vec<Beacon>,
    location: Option<Vec3>,
}

impl PartialEq for Scanner {
    fn eq(&self, other: &Scanner) -> bool {
        self.id == other.id
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Beacon {
    /// Relative position
    pos: Vec3,
    neighbors: Vec<Vec3>,
}

impl FromStr for Scanner {
    type Err = anyhow::Error;
    fn from_str(input: &str) -> Result<Self> {
        let mut lines = input.lines();
        let header = lines.next().ok_or(anyhow!("Missing header"))?;
        let id = header[HEADER_OFFSET..]
            .chars()
            .take_while(|c| c.to_digit(10).is_some())
            .collect::<String>()
            .parse()?;

        let visible_locations = lines.map(|v| v.parse()).collect::<Result<Vec<Vec3>>>()?;

        let visible = visible_locations
            .clone()
            .iter()
            .map(|vec| {
                let mut neighbors = visible_locations
                    .iter()
                    .filter(|oth| *oth != vec)
                    .map(|other| other - vec)
                    .collect::<Vec<_>>();

                neighbors.sort_by_key(|neighbor| neighbor.square_distance(vec));

                Beacon {
                    pos: vec.clone(),
                    neighbors,
                }
            })
            .collect();

        let location = if id == 0 { Some(Vec3(0, 0, 0)) } else { None };

        Ok(Scanner {
            id,
            visible,
            location,
        })
    }
}

fn parse_input(input: String) -> Result<Vec<Scanner>> {
    input.split("\n\n").map(|block| block.parse()).collect()
}

fn find_beacon_locations(scanners: Vec<Scanner>) -> HashSet<Vec3> {
    let result = HashSet::new();

    while scanners.iter().any(|s| s.location.is_none()) {
        for scanner1 in scanners.iter() {
            if let None = scanner1.location {
                continue;
            }
            for scanner2 in scanners.iter() {
                if scanner1 != scanner2 && scanner2.location.is_none() {
                    todo!()
                }
            }
        }
    }

    result
}

pub fn part1(input: String) -> usize {
    let scanners = parse_input(input).expect("Input parse failed");

    println!("{:?}", scanners[0]);

    0
}
pub fn part2(_input: String) -> usize {
    0
}
