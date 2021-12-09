use std::collections::HashSet;

fn parse_input(input: String) -> Vec<Vec<usize>> {
    return input
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                .collect()
        })
        .collect();
}

fn find_local_minima(board: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    let mut result = Vec::default();
    for (y, row) in board.iter().enumerate() {
        for (x, &val) in row.iter().enumerate() {
            let &above = if y == 0 {
                &usize::MAX
            } else {
                board.get(y - 1).map(|row| &row[x]).unwrap_or(&usize::MAX)
            };
            let &below = board.get(y + 1).map(|row| &row[x]).unwrap_or(&usize::MAX);
            let &right = board[y].get(x + 1).unwrap_or(&usize::MAX);
            let &left = if x == 0 {
                &usize::MAX
            } else {
                board[y].get(x - 1).unwrap_or(&usize::MAX)
            };

            let is_match = val < above && val < below && val < right && val < left;

            if is_match {
                result.push((x, y))
            }
        }
    }
    return result;
}

fn find_basin_size(board: &Vec<Vec<usize>>, local_minimum: (usize, usize)) -> usize {
    let mut found: HashSet<(usize, usize)> = HashSet::from([local_minimum]);
    let mut to_visit: Vec<(usize, usize)> = vec![local_minimum];
    let mut size = 0;

    let mut i = 0;
    while i < to_visit.len() {
        size += 1;

        let (x, y) = to_visit[i];
        let mut around = vec![(x, y + 1), (x + 1, y)];

        if x != 0 {
            around.push((x - 1, y));
        }
        if y != 0 {
            around.push((x, y - 1));
        }

        for (x_around, y_around) in around {
            if found.contains(&(x_around, y_around)) {
                continue;
            }
            let &value = board
                .get(y_around)
                .and_then(|row| row.get(x_around))
                .unwrap_or(&9);
            if value == 9 {
                continue;
            }

            found.insert((x_around, y_around));
            to_visit.push((x_around, y_around));
        }

        i += 1;
    }

    return size;
}

pub fn part1(input: String) -> usize {
    let board = parse_input(input);

    return find_local_minima(&board)
        .iter()
        .map(|(x, y)| &board[*y][*x] + 1)
        .sum();
}

pub fn part2(input: String) -> usize {
    let board = parse_input(input);

    let mut basins = find_local_minima(&board)
        .iter()
        .map(|&(x, y)| find_basin_size(&board, (x, y)))
        .collect::<Vec<_>>();

    basins.sort_by(|a, b| b.cmp(a));

    return basins.iter().take(3).product();
}
