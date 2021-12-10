fn parse_input(input: String) -> Vec<String> {
  return input.split('\n').map(|s| s.to_string()).collect();
}

enum ChunkState {
  Valid,
  Invalid(char),
  Incomplete(Vec<char>),
}

fn syntax_check(chunk: &String) -> ChunkState {
  let mut stack: Vec<char> = Vec::default();
  for c in chunk.chars() {
    match c {
      '(' | '[' | '{' | '<' => stack.push(c),
      ')' | ']' | '}' | '>' => {
        let open_c_option = stack.pop();
        if open_c_option.is_none() {
          return ChunkState::Invalid(c);
        }
        let open_c = open_c_option.unwrap();
        if (open_c == '(' && c != ')')
          || (open_c == '[' && c != ']')
          || (open_c == '{' && c != '}')
          || (open_c == '<' && c != '>')
        {
          return ChunkState::Invalid(c);
        }
      }
      _ => return ChunkState::Invalid(c),
    }
  }
  if stack.len() != 0 {
    return ChunkState::Incomplete(stack);
  }
  return ChunkState::Valid;
}

fn score(c: char) -> usize {
  match c {
    ')' => 3,
    ']' => 57,
    '}' => 1197,
    '>' => 25137,
    _ => panic!("Invalid character in scoring"),
  }
}

fn score_missing(mut incomplete_stack: Vec<char>) -> usize {
  incomplete_stack.reverse();
  let mut score = 0;

  for c in incomplete_stack {
    score *= 5;
    score += match c {
      '(' => 1,
      '[' => 2,
      '{' => 3,
      '<' => 4,
      _ => panic!("Invalid character in scoring"),
    }
  }

  return score;
}

pub fn part1(input: String) -> usize {
  let chunks = parse_input(input);

  return chunks
    .iter()
    .map(syntax_check)
    .filter_map(|result| match result {
      ChunkState::Invalid(c) => Some(c),
      _ => None,
    })
    .map(score)
    .sum();
}

pub fn part2(input: String) -> usize {
  let chunks = parse_input(input);

  let mut scores = chunks
    .iter()
    .map(syntax_check)
    .filter_map(|result| match result {
      ChunkState::Incomplete(remaining) => Some(remaining),
      _ => None,
    })
    .map(score_missing)
    .collect::<Vec<_>>();

  scores.sort();

  return scores[scores.len() / 2];
}
