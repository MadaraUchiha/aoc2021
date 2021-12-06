fn parse_input(input: String) -> Vec<usize> {
    return input
        .split(',')
        .map(|n| n.parse())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
}

fn count_fish(fish: Vec<usize>, days: usize) -> usize {
    let mut school = [0; 9];
    for i in fish {
        school[i] += 1;
    }

    for i in (0..9).cycle().take(days) {
        school[(i + 7) % 9] += school[i];
    }

    return school.iter().sum();
}

pub fn part1(input: String) -> usize {
    let fish = parse_input(input);

    return count_fish(fish, 80);
}
pub fn part2(input: String) -> usize {
    let fish = parse_input(input);

    return count_fish(fish, 256);
}
