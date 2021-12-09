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

fn find_basin(
    board: &Vec<Vec<usize>>,
    known_basin: Vec<(usize, usize)>,
    checked_positions: Vec<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let mut new_basin = known_basin.clone();
    let mut checked_positions_copy = checked_positions.clone();

    for &(x, y) in &known_basin {
        let mut around = vec![(x, y + 1), (x + 1, y)];

        if x != 0 {
            around.push((x - 1, y));
        }
        if y != 0 {
            around.push((x, y - 1));
        }

        for (x_around, y_around) in around {
            if !checked_positions_copy.contains(&(x_around, y_around))
                && *board.get(y_around).and_then(|row| row.get(x_around)).unwrap_or(&usize::MAX) < 9
            {
                new_basin.push((x_around, y_around));
                checked_positions_copy.push((x_around, y_around));
            }
        }
    }

    if new_basin.len() == known_basin.len() {
        return new_basin;
    }
    return find_basin(board, new_basin, checked_positions_copy);
    
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
        .map(|&(x, y)| find_basin(&board, vec![(x, y)], vec![(x, y)]))
        .map(|basin| basin.len())
        .collect::<Vec<_>>();

    basins.sort_by(|a, b| b.cmp(a));

    return basins.iter().take(3).product();
}
