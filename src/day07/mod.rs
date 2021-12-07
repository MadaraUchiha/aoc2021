fn parse_input(input: String) -> Vec<isize> {
    return input.split(',').map(|n| n.parse().unwrap()).collect();
}

fn sum_of_first_n_numbers(n: isize) -> isize {
    // return n * (n + 1) / 2;
    return (1..=n).sum();
}

fn find_minimal_fuel_consumption<F: Fn(isize) -> isize>(
    crabs: Vec<isize>,
    fuel_cost_getter: F,
) -> isize {
    let &max = crabs.iter().max().unwrap();
    let &min = crabs.iter().min().unwrap();

    return (min..=max)
        .map(|target| {
            crabs
                .iter()
                .map(|crab| fuel_cost_getter((target - crab).abs()))
                .sum()
        })
        .min()
        .unwrap();
}

pub fn part1(input: String) -> usize {
    let crabs = parse_input(input);

    return find_minimal_fuel_consumption(crabs, std::convert::identity)
        .try_into()
        .unwrap();
}

pub fn part2(input: String) -> usize {
    let crabs = parse_input(input);

    return find_minimal_fuel_consumption(crabs, sum_of_first_n_numbers)
        .try_into()
        .unwrap();
}
