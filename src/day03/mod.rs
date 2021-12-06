fn parse_input(input: String) -> Vec<String> {
    input.split_whitespace().map(|x| x.to_string()).collect()
}

fn count_ones(numbers: Vec<String>) -> Vec<usize> {
    let indices = (0..numbers[0].len()).collect::<Vec<_>>();
    return indices
        .iter()
        .copied()
        .map(|i| {
            numbers
                .iter()
                .map(|n| n.chars().nth(i))
                .filter(|n| n.unwrap_or('0') == '1')
                .count()
        })
        .collect();
}

fn inverse_binary_string(number: String) -> String {
    return number
        .chars()
        .map(|c| if c == '1' { '0' } else { '1' })
        .collect::<String>();
}

pub fn part1(input: String) -> usize {
    let numbers = parse_input(input);
    let majority = numbers.len() / 2;

    let ones = count_ones(numbers);
    let gamma_rate_binary = ones
        .iter()
        .cloned()
        .map(|n| if n >= majority { '1' } else { '0' })
        .collect::<String>();
    let epislon_rate_binary = inverse_binary_string(gamma_rate_binary.clone());

    let gamma_rate = usize::from_str_radix(&gamma_rate_binary, 2).unwrap();
    let epislon_rate = usize::from_str_radix(&epislon_rate_binary, 2).unwrap();

    return gamma_rate * epislon_rate;
}

enum BitCriteria {
    OxygenGeneratorRating,
    CO2ScrubberRating,
}

fn find_with_criteria(numbers: Vec<String>, index: usize, criteria: BitCriteria) -> String {
    if numbers.len() == 1 {
        return numbers[0].clone();
    }

    let total = numbers.len();
    let ones = count_ones(numbers.clone())[index];
    let zeros = total - ones;

    let selected_char = match criteria {
        BitCriteria::OxygenGeneratorRating => if ones >= zeros { '1' } else { '0' }
        BitCriteria::CO2ScrubberRating => if ones >= zeros { '0' } else { '1' }
    };

    let remaining_numbers: Vec<String> = numbers
        .iter()
        .cloned()
        .filter(|n| n.chars().nth(index).unwrap() == selected_char)
        .collect();

    return find_with_criteria(remaining_numbers, index + 1, criteria);
}

pub fn part2(input: String) -> usize {
    let numbers = parse_input(input);

    let oxygen_generator_rating_binary =
        find_with_criteria(numbers.clone(), 0, BitCriteria::OxygenGeneratorRating);
    let co2_scrubber_rating_binary = find_with_criteria(numbers, 0, BitCriteria::CO2ScrubberRating);

    let oxygen_generator_rating = usize::from_str_radix(&oxygen_generator_rating_binary, 2).unwrap();
    let co2_scrubber_rating = usize::from_str_radix(&co2_scrubber_rating_binary, 2).unwrap();

    return oxygen_generator_rating * co2_scrubber_rating;
}
