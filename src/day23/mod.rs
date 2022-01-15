use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt::Display;
use std::fmt::Formatter;

type Cost = usize;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Amphipod {
    fn cost(&self) -> Cost {
        match self {
            Self::Amber => 1,
            Self::Bronze => 10,
            Self::Copper => 100,
            Self::Desert => 1000,
        }
    }

    fn parse(c: u8) -> Option<Self> {
        match c {
            b'A' => Some(Self::Amber),
            b'B' => Some(Self::Bronze),
            b'C' => Some(Self::Copper),
            b'D' => Some(Self::Desert),
            _ => None,
        }
    }

    fn from_index(i: usize) -> Option<Self> {
        match i {
            0 => Some(Self::Amber),
            1 => Some(Self::Bronze),
            2 => Some(Self::Copper),
            3 => Some(Self::Desert),
            _ => None,
        }
    }
}

impl Display for Amphipod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Amber => "A",
            Self::Bronze => "B",
            Self::Copper => "C",
            Self::Desert => "D",
        };
        write!(f, "{}", s)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Room<const SIZE: usize> {
    id: Amphipod,
    slots: [Option<Amphipod>; SIZE],
}

impl<const SIZE: usize> Room<SIZE> {
    fn is_done(&self) -> bool {
        self.slots.iter().all(|s| s == &Some(self.id))
    }
    fn take(&mut self) -> Option<(Cost, Amphipod)> {
        for i in 0..SIZE {
            if self.slots[i].is_some() {
                if self.slots[i] != Some(self.id) {
                    return self.slots[i].take().map(|s| (i as Cost + 1, s));
                } else if self.slots[i + 1..].iter().any(|s| s != &Some(self.id)) {
                    return self.slots[i].take().map(|s| (i as Cost + 1, s));
                } else {
                    return None;
                }
            }
        }
        None
    }

    fn accepts(&self, amphipod: Amphipod) -> bool {
        if amphipod != self.id {
            return false;
        }
        self.slots
            .iter()
            .all(|s| s.is_none() || s == &Some(self.id))
    }

    fn accept(&mut self, amphipod: Amphipod) -> Cost {
        debug_assert!(self.accepts(amphipod));

        for i in (0..SIZE).rev() {
            if self.slots[i].is_none() {
                self.slots[i] = Some(amphipod);
                return i as Cost + 1;
            }
        }
        panic!("oops");
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
struct Connections([Option<Amphipod>; 11]);

impl Connections {
    fn can_access_room_from_tile(&self, from_tile: usize, to_room: usize) -> Cost {
        let end_index = 2 + to_room * 2;
        self.int_move(from_tile, end_index, false)
    }

    fn can_access_tile_from_room(&self, room: usize, tile: usize) -> Cost {
        let room_index = 2 + room * 2;
        self.int_move(room_index, tile, true)
    }

    fn int_move(&self, from_tile: usize, to_tile: usize, target_is_tile: bool) -> Cost {
        if target_is_tile && to_tile >= 2 && to_tile <= self.0.len() - 2 && to_tile % 2 == 0 {
            // to_tile is not accessible (directly in front of a room)
            return 0;
        }
        if from_tile == to_tile {
            return 0;
        }
        let (cost, from_tile, to_tile) = if from_tile < to_tile {
            (to_tile - from_tile, from_tile + 1, to_tile)
        } else {
            (from_tile - to_tile, to_tile, from_tile - 1)
        };

        for s in &self.0[from_tile..=to_tile] {
            if s.is_some() {
                return 0;
            }
        }
        cost as Cost
    }
}

impl Display for Connections {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for c in self.0 {
            match c {
                Some(amp) => write!(f, "{}", amp)?,
                None => write!(f, ".")?,
            }
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Cave<const ROOM_SIZE: usize> {
    rooms: [Room<ROOM_SIZE>; 4],
    connections: Connections,
}

impl<const ROOM_SIZE: usize> Cave<ROOM_SIZE> {
    fn is_done(&self) -> bool {
        self.rooms.iter().all(|room| room.is_done())
    }

    fn generate_moves(&self) -> Option<Vec<(Cost, Self)>> {
        if self.is_done() {
            return None;
        }

        let mut moves = vec![];
        for i_room in 0..self.rooms.len() {
            let mut cave = *self;

            // see if anything can move from a path to a room
            for i_conn in 0..cave.connections.0.len() {
                let amphipod = if let Some(amphipod) = cave.connections.0[i_conn] {
                    amphipod
                } else {
                    continue;
                };

                let access_cost = self.connections.can_access_room_from_tile(i_conn, i_room);
                if access_cost > 0 && cave.rooms[i_room].accepts(amphipod) {
                    let mut move_ = cave;
                    move_.connections.0[i_conn] = None;
                    let move_in_cost = move_.rooms[i_room].accept(amphipod);
                    moves.push(((access_cost + move_in_cost) * amphipod.cost(), move_));
                }
            }

            // All things that require to move something out of a room
            let (move_out_cost, amphipod) =
                if let Some((move_out_cost, amphipod)) = cave.rooms[i_room].take() {
                    (move_out_cost, amphipod)
                } else {
                    continue;
                };

            // see if anything can move out of a room into a free spot
            for i_conn in 0..cave.connections.0.len() {
                let access_cost = self.connections.can_access_tile_from_room(i_room, i_conn);
                if access_cost > 0 {
                    let mut move_ = cave;
                    move_.connections.0[i_conn] = Some(amphipod);
                    moves.push(((move_out_cost + access_cost) * amphipod.cost(), move_));
                }
            }
        }

        Some(moves)
    }
}

impl<const ROOM_SIZE: usize> Display for Cave<ROOM_SIZE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "#############")?;
        writeln!(f, "#{}#", self.connections)?;

        for i in 0..ROOM_SIZE {
            write!(f, "###")?;
            for room in &self.rooms {
                match room.slots[i] {
                    Some(amp) => write!(f, "{}", amp)?,
                    None => write!(f, ".")?,
                }
                write!(f, "#")?;
            }
            writeln!(f, "##")?;
        }

        write!(f, "  #########  ")
    }
}

fn parse<const ROOM_SIZE: usize>(input: String) -> Option<Cave<ROOM_SIZE>> {
    let mut field = input.lines().skip(1).map(|line| line.as_bytes());

    let mut connections = Connections::default();
    for (i, amp) in field
        .next()?
        .into_iter()
        .filter(|&x| (b'A'..=b'D').contains(x) || *x == b'.')
        .enumerate()
    {
        connections.0[i] = Amphipod::parse(*amp);
    }

    let room_lines: Vec<_> = field.take(ROOM_SIZE).collect();

    let rooms = (0..4)
        .into_iter()
        .map(|i| {
            Some(Room {
                id: Amphipod::from_index(i)?,
                slots: room_lines
                    .iter()
                    .map(|line| Amphipod::parse(line[3 + 2 * i]))
                    .collect::<Vec<_>>()
                    .try_into()
                    .ok()?,
            })
        })
        .collect::<Option<Vec<_>>>()?
        .as_slice()
        .try_into()
        .ok()?;

    Some(Cave { rooms, connections })
}

#[derive(Debug, PartialEq, Eq)]
struct Element<T: Eq> {
    cost: Cost,
    item: T,
}

impl<T: Eq> Element<T> {
    fn new(cost: Cost, item: T) -> Self {
        Self { cost, item }
    }
}

impl<T: Eq> PartialOrd for Element<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Eq> Ord for Element<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

fn solve<const ROOM_SIZE: usize>(cave: Cave<ROOM_SIZE>) -> Option<Cost> {
    let mut to_visit = BinaryHeap::new();
    to_visit.push(Element::new(0, cave));

    while let Some(element) = to_visit.pop() {
        if element.item.is_done() {
            // println!("Reached an end! {}", element.cost);
            return Some(element.cost);
        }

        if let Some(moves) = element.item.generate_moves() {
            for (cost, new_cave) in moves {
                to_visit.push(Element::new(element.cost + cost, new_cave))
            }
        }
    }

    None
}

pub fn part1(input: String) -> usize {
    let cave = parse::<2>(input).expect("Unable to parse cave");

    solve(cave).expect("Unable to solve")
}
pub fn part2(input: String) -> usize {
    let mut input_lines: Vec<_> = input.lines().collect();
    input_lines.insert(3, "  #D#B#A#C#");
    input_lines.insert(3, "  #D#C#B#A#");
    let new_input = input_lines.join("\n");
    let cave = parse::<4>(new_input).expect("Unable to parse cave");

    solve(cave).expect("Unable to solve")
}
