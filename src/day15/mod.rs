use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

type Co = (isize, isize);
type Board = HashMap<Co, usize>;

#[derive(Eq)]
struct Node {
    coords: Co,
    priority: usize,
}

impl Node {
    pub fn new(((x, y), priority): (Co, usize)) -> Self {
        return Node {
            coords: (x, y),
            priority,
        };
    }

    pub fn neighbors(&self) -> Vec<Co> {
        let Node { coords: (x, y), .. } = self;
        return vec![(x - 1, *y), (x + 1, *y), (*x, y - 1), (*x, y + 1)];
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.priority.cmp(&other.priority);
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        return self.priority == other.priority;
    }
}

fn parse_input(input: String, macro_board_size: usize) -> Board {
    let mut board = HashMap::default();
    let max_y = input.lines().count();
    let max_x = input.lines().next().unwrap().len();
    for (y, row) in input.split('\n').enumerate() {
        for (x, digit) in row.chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
            let d = digit as usize;
            *board.entry((x as isize, y as isize)).or_default() = d;
            for x_offset in 0..macro_board_size {
                for y_offset in 0..macro_board_size {
                    *board
                        .entry((
                            (x + (x_offset * max_x)) as isize,
                            (y + (y_offset * max_y)) as isize,
                        ))
                        .or_default() = (d + x_offset + y_offset - 1) % 9 + 1
                }
            }
        }
    }

    return board;
}

fn find_maximum_coord(board: &Board) -> Option<Co> {
    let (max_x, _) = board.keys().max_by_key(|(x, _)| x)?;
    let (_, max_y) = board.keys().max_by_key(|(_, y)| y)?;

    return Some((*max_x, *max_y));
}

fn search_heuristic((x_a, y_a): Co, (x_b, y_b): Co) -> usize {
    return ((x_a - x_b).abs() + (y_a - y_b).abs()) as usize;
}

fn find_path(board: Board) -> Option<usize> {
    let goal = find_maximum_coord(&board)?;
    let mut came_from: HashMap<Co, Co> = HashMap::default();
    let mut to_visit: BinaryHeap<Node> = BinaryHeap::default();
    let mut cost_so_far: Board = HashMap::default();
    let mut visited_so_far: HashSet<Co> = HashSet::default();

    to_visit.push(Node::new(((0, 0), board[&(0, 0)])));
    cost_so_far.entry((0, 0)).or_insert(0);

    while to_visit.len() > 0 {
        let current = to_visit.pop()?;
        // println!("Visiting {:?} towards {:?}", current.coords, goal);
        // if current.coords == goal {
        //     break;
        // }

        visited_so_far.insert(current.coords);

        for next_coords in current.neighbors() {
            if !board.contains_key(&next_coords) {
                continue;
            }
            let new_cost = cost_so_far[&(current.coords)] + board[&(next_coords)];
            let next_cost_so_far = cost_so_far.entry(next_coords).or_insert(usize::MAX);

            // println!(
            //     "Looking at {:?}, the current cost is {}, the next cost is {}",
            //     next_coords, next_cost_so_far, new_cost
            // );

            if *next_cost_so_far > new_cost {
                *next_cost_so_far = new_cost;
                let next_priority =
                    (9 - board[&(next_coords)]) + search_heuristic(goal, next_coords);
                to_visit.push(Node::new((next_coords, next_priority)));
                came_from.insert(next_coords, current.coords);
            }
        }
    }

    return Some(cost_so_far[&goal]);
}

pub fn part1(input: String) -> usize {
    let board = parse_input(input, 1);

    return find_path(board).expect("Path not found");
}
pub fn part2(input: String) -> usize {
    let board = parse_input(input, 5);

    return find_path(board).expect("Path not found");
}
