use anyhow::Result;
use aoc2021::Vec2;
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Herd {
    S,
    E,
}

impl Herd {
    fn direction(&self) -> Vec2 {
        match self {
            Self::S => Vec2(0, 1),
            Self::E => Vec2(1, 0),
        }
    }
}

fn char_to_tile(c: char) -> Option<Herd> {
    match c {
        'v' => Some(Herd::S),
        '>' => Some(Herd::E),
        _ => None,
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Field(HashMap<Vec2, Option<Herd>>);

impl FromStr for Field {
    type Err = anyhow::Error;
    fn from_str(input: &str) -> Result<Self> {
        let mut map = HashMap::new();

        for (y, row) in input.lines().enumerate() {
            for (x, c) in row.chars().enumerate() {
                let vec = Vec2(x.try_into()?, y.try_into()?);
                map.insert(vec, char_to_tile(c));
            }
        }

        Ok(Self(map))
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let max_x = self.0.keys().max_by_key(|k| k.0).unwrap().0;
        let max_y = self.0.keys().max_by_key(|k| k.1).unwrap().1;

        println!("{}, {}", max_x, max_y);

        for y in 0..=max_y {
            for x in 0..=max_x {
                let c = match self.0.get(&Vec2(x, y)) {
                    None => Err(std::fmt::Error),
                    Some(None) => Ok("."),
                    Some(Some(Herd::S)) => Ok("v"),
                    Some(Some(Herd::E)) => Ok(">"),
                };
                write!(f, "{}", c?)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Field {
    fn new() -> Self {
        Self(HashMap::default())
    }
    fn step(&self) -> Self {
        self.move_herd(Herd::E).move_herd(Herd::S)
    }

    fn move_herd(&self, herd: Herd) -> Self {
        let will_move = self.0.clone().into_iter().filter(|(key, value)| {
            if *value != Some(herd.clone()) {
                return false;
            }
            if let Some(herd) = value {
                let mut next_position = key + &herd.direction();
                if !self.0.contains_key(&next_position) {
                    next_position = match herd {
                        Herd::E => Vec2(0, next_position.1),
                        Herd::S => Vec2(next_position.0, 0),
                    }
                }
                if !self.0.contains_key(&next_position) {
                    println!("{:?}", next_position);
                }
                return self.0[&next_position].is_none();
            }
            false
        });

        let mut next_field = self.clone();

        for (pos, herd) in will_move {
            let herd = herd.unwrap();
            let mut next_position = &pos + &herd.direction();
            if !self.0.contains_key(&next_position) {
                next_position = match herd {
                    Herd::E => Vec2(0, next_position.1),
                    Herd::S => Vec2(next_position.0, 0),
                }
            }
            next_field.0.insert(next_position, Some(herd));
            next_field.0.insert(pos, None);
        }

        next_field
    }
}

pub fn part1(input: String) -> usize {
    let mut field = input.parse::<Field>().expect("Parse failed");
    let mut prev_field = Field::new();
    let mut steps = 0;

    while field != prev_field {
        steps += 1;
        prev_field = field.clone();
        field = field.step();
    }

    steps
}
pub fn part2(_: String) -> usize {
    42
}
