use std::collections::HashMap;

#[derive(Default, Debug)]
struct Polymer {
    occurences: HashMap<[char; 2], usize>,
    instructions: HashMap<[char; 2], char>,
}

fn parse_input(input: String) -> Option<(Polymer, char)> {
    let (template, instruction_strs) = input.split_once("\n\n")?;
    let instructions = instruction_strs
        .split('\n')
        .map(|inst| inst.split_once(" -> "))
        .map(|maybe_parts| {
            maybe_parts.and_then(|(portion, c)| {
                let chars = portion.chars().collect::<Vec<_>>();
                Some(([chars[0], chars[1]], c.chars().next()?))
            })
        })
        .collect::<Option<_>>()?;

    let mut occurences: HashMap<[char; 2], usize> = HashMap::default();
    for slice in template.chars().collect::<Vec<_>>().windows(2) {
        *occurences.entry([slice[0], slice[1]]).or_default() += 1;
    }

    return Some((
        Polymer {
            occurences,
            instructions,
        },
        template.chars().last()?,
    ));
}

fn polymize(
    Polymer {
        occurences,
        instructions,
    }: Polymer,
) -> Polymer {
    let mut new_occurences = HashMap::default();

    for ([first, last], v) in occurences {
        let mid = instructions[&[first, last]];
        *new_occurences.entry([first, mid]).or_default() += v;
        *new_occurences.entry([mid, last]).or_default() += v;
    }

    return Polymer {
        occurences: new_occurences,
        instructions,
    };
}

fn count_letters(
    occurences: HashMap<[char; 2], usize>,
    last_character: char,
) -> HashMap<char, usize> {
    let mut result = HashMap::default();

    for ([first, _], count) in occurences {
        *result.entry(first).or_default() += count;
    }

    *result.entry(last_character).or_default() += 1;

    return result;
}

fn count_min_max((mut polymer, last_character): (Polymer, char), iterations: usize) -> usize {
    for _ in 0..iterations {
        polymer = polymize(polymer);
    }

    let letter_counts = count_letters(polymer.occurences, last_character);

    let (_, &min) = letter_counts
        .iter()
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap();
    let (_, &max) = letter_counts
        .iter()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap();

    return max - min;
}

pub fn part1(input: String) -> usize {
    let polymer = parse_input(input).expect("Parse failure");

    return count_min_max(polymer, 10);
}

pub fn part2(input: String) -> usize {
    let polymer = parse_input(input).expect("Parse failure");

    return count_min_max(polymer, 40);
}
