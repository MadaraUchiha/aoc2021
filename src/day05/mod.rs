use std::str::FromStr;

#[derive(Clone)]
struct Vector {
    x: i32,
    y: i32,
}

impl FromStr for Vector {
    type Err = &'static str;
    fn from_str(input: &str) -> Result<Self, <Self as FromStr>::Err> {
        match input.split_once(',') {
            None => Err("Split failed"),
            Some((x, y)) => match (x.parse(), y.parse()) {
                (Ok(x), Ok(y)) => Ok(Vector { x, y }),
                _ => Err("Parse failed"),
            },
        }
    }
}

#[derive(Clone)]
struct Line {
    a: Vector,
    b: Vector,
}
impl Line {
    fn is_90_deg(&self) -> bool {
        let Vector { x, y } = self.iter_direction();
        return x * y == 0;
    }
    fn iter_direction(&self) -> Vector {
        return Vector {
            x: (self.b.x - self.a.x).signum(),
            y: (self.b.y - self.a.y).signum(),
        };
    }
}
impl IntoIterator for Line {
    type Item = Vector;
    type IntoIter = LineIterator;
    fn into_iter(self) -> <Self as std::iter::IntoIterator>::IntoIter {
        return LineIterator::new(self);
    }
}
struct LineIterator {
    line: Line,
    iter_state: Vector,
}
impl LineIterator {
    fn new(line: Line) -> LineIterator {
        let initial_state = line.a.clone();
        return LineIterator {
            line,
            iter_state: initial_state,
        };
    }
}

impl Iterator for LineIterator {
    type Item = Vector;
    fn next(&mut self) -> Option<<Self as IntoIterator>::Item> {
        let Vector {
            x: x_advance,
            y: y_advance,
        } = self.line.iter_direction();
        let Vector { x, y } = self.iter_state.clone();
        let next = Vector {
            x: x + x_advance,
            y: y + y_advance,
        };
        let Vector {
            x: last_x,
            y: last_y,
        } = Vector {
            x: self.line.b.x + x_advance,
            y: self.line.b.y + y_advance,
        };

        if x == last_x && y == last_y {
            return None;
        }

        self.iter_state = next;
        return Some(Vector { x, y });
    }
}

fn parse_input(input: String) -> Vec<Line> {
    return input
        .split('\n')
        .map(|text_line| text_line.split(" -> ").collect())
        .map(|points: Vec<_>| Line {
            a: Vector::from_str(points[0]).unwrap(),
            b: Vector::from_str(points[1]).unwrap(),
        })
        .collect();
}

fn count_intersections(lines: Vec<Line>) -> i32 {
    let mut board = vec![vec![0i16; 1000]; 1000];
    let mut count = 0;

    for line in lines {
        for Vector { x, y } in line {
            board[y as usize][x as usize] += 1;
        }
    }

    for row in board {
        for val in row {
            if val > 1 {
                count += 1;
            }
        }
    }

    return count;
}

pub fn part1(input: String) -> i32 {
    let lines = parse_input(input);

    let straight_lines = lines
        .iter()
        .filter(|line| line.is_90_deg())
        .cloned()
        .collect();

    return count_intersections(straight_lines);
}

pub fn part2(input: String) -> i32 {
    let lines = parse_input(input);

    return count_intersections(lines);
}
