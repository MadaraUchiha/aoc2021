use aoc2021::Vec2;

#[derive(Clone)]
struct Line {
    a: Vec2,
    b: Vec2,
}
impl Line {
    fn is_90_deg(&self) -> bool {
        let Vec2(x, y) = self.iter_direction();
        return x * y == 0;
    }
    fn iter_direction(&self) -> Vec2 {
        let Vec2(x, y) = &self.b - &self.a;
        Vec2(x.signum(), y.signum())
    }
}
impl IntoIterator for Line {
    type Item = Vec2;
    type IntoIter = LineIterator;
    fn into_iter(self) -> <Self as std::iter::IntoIterator>::IntoIter {
        return LineIterator::new(self);
    }
}
struct LineIterator {
    line: Line,
    iter_state: Vec2,
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
    type Item = Vec2;
    fn next(&mut self) -> Option<<Self as IntoIterator>::Item> {
        let advance_vec = self.line.iter_direction();
        let curr_state = self.iter_state.clone();
        let next = &curr_state + &advance_vec;
        let last = &self.line.b + &advance_vec;

        if curr_state == last {
            return None;
        }

        self.iter_state = next;
        return Some(curr_state);
    }
}

fn parse_input(input: String) -> Vec<Line> {
    return input
        .split('\n')
        .map(|text_line| text_line.split(" -> ").collect())
        .map(|points: Vec<_>| Line {
            a: points[0].parse().unwrap(),
            b: points[1].parse().unwrap(),
        })
        .collect();
}

fn count_intersections(lines: Vec<Line>) -> usize {
    let mut board = vec![vec![0i16; 1000]; 1000];
    let mut count = 0;

    for line in lines {
        for Vec2(x, y) in line {
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

pub fn part1(input: String) -> usize {
    let lines = parse_input(input);

    let straight_lines = lines
        .iter()
        .filter(|line| line.is_90_deg())
        .cloned()
        .collect();

    return count_intersections(straight_lines);
}

pub fn part2(input: String) -> usize {
    let lines = parse_input(input);

    return count_intersections(lines);
}
