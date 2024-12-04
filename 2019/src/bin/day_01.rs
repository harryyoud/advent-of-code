use aoc_2019::get_input;

fn main() {
    let input = get_input(1);

    let module_masses: Vec<u32> = input
        .lines()
        .map(|line| line.parse::<u32>().expect("Valid number in input"))
        .collect();

    dbg!(part_1(&module_masses));
    dbg!(part_2(&module_masses));
}

fn part_1(module_masses: &[u32]) -> u32 {
    module_masses.iter().copied().map(calculate_fuel).sum()
}

fn part_2(module_masses: &[u32]) -> u32 {
    module_masses
        .iter()
        .copied()
        .map(calculate_fuel_recursive)
        .sum()
}

fn calculate_fuel(mass: u32) -> u32 {
    if mass == 0 {
        return 0;
    }
    (mass / 3).saturating_sub(2)
}

fn calculate_fuel_recursive(mass: u32) -> u32 {
    let mut fuel = 0;
    let mut last_weight_added = mass;
    while last_weight_added > 0 {
        last_weight_added = calculate_fuel(last_weight_added);
        fuel += last_weight_added;
    }
    fuel
}
