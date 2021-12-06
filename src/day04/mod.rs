enum GameState {
    Bingo(Game, usize),
    NoBingo(Game),
}
#[derive(Clone)]
struct Game {
    unmarked_numbers: Vec<u16>,
    called_number: u16,
    boards: Vec<Vec<Vec<u16>>>,
}
fn parse_input(input: String) -> Game {
    let chunks = input.split_terminator("\n\n").collect::<Vec<_>>();
    let numbers = chunks[0].split(',').map(|n| n.parse().unwrap()).collect();
    let boards = chunks[1..].iter().map(parse_board).collect::<Vec<_>>();

    return Game {
        unmarked_numbers: numbers,
        boards: boards,
        called_number: 999,
    };
}

fn parse_board(input: &&str) -> Vec<Vec<u16>> {
    let lines = input.split('\n');
    return lines
        .map(|line| {
            line.split(' ')
                .filter(|s| !s.is_empty())
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect();
}

fn play_bingo_till_first(game: Game) -> GameState {
    return match play_bingo_step(game.clone()) {
        GameState::NoBingo(Game { unmarked_numbers, .. }) => {
            let (next_called_number, next_unmarked_numbers) = unmarked_numbers.split_at(1);
            let next_game = Game {
                unmarked_numbers: next_unmarked_numbers.to_vec(),
                called_number: next_called_number[0],
                ..game.clone()
            };
            return play_bingo_till_first(next_game);
        }
        bingo => bingo,
    };
}

fn play_bingo_till_last(game: Game) -> GameState {
    return match play_bingo_till_first(game.clone()) {
        GameState::Bingo(mut game, i) => {
            if game.boards.len() > 1 {
                game.boards.remove(i);
                return play_bingo_till_last(game);
            } else {
                GameState::Bingo(game, i)
            }
        }
        not_bingo => not_bingo,
    };
}

fn play_bingo_step(game: Game) -> GameState {
    return match bingo_check(&game) {
        Some(n) => GameState::Bingo(game, n),
        None => GameState::NoBingo(game),
    };
}

fn bingo_check( Game { unmarked_numbers, boards, .. }: &Game, ) -> std::option::Option<usize> {
    return boards.iter().position(|board| {
        let has_rows = board
            .iter()
            .any(|row| row.iter().all(|n| !unmarked_numbers.contains(n)));
        let has_cols = (1..5).any(|i| board.iter().all(|row| !unmarked_numbers.contains(&row[i])));

        return has_rows || has_cols;
    });
}

fn count_score(state: GameState) -> u16 {
    match state {
        GameState::NoBingo(_) => panic!("Counted score with no bingo, shouldn't be reached"),
        GameState::Bingo(Game { unmarked_numbers, boards, called_number, }, i) => {
            let board = &boards[i];

            let unmarked_sum = board
                .iter()
                .map(|row| {
                    row.iter()
                        .filter(|n| unmarked_numbers.contains(n))
                        .sum::<u16>()
                })
                .sum::<u16>();

            return unmarked_sum * called_number;
        }
    }
}

pub fn part1(input: String) -> usize {
    let game = parse_input(input);

    let score = count_score(play_bingo_till_first(game));

    return score.into();
}

pub fn part2(input: String) -> usize {
    let game = parse_input(input);

    let score = count_score(play_bingo_till_last(game));

    return score.into();
}
