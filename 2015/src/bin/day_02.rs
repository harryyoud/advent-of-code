use aoc_2015::get_input;
use itertools::Itertools;

fn main() {
    let input = get_input(2);

    dbg!(part_1(&input));
    dbg!(part_2(&input));
}

fn parse_input(line: &str) -> [u64; 3] {
    line
        .split("x")
        .map(|x| x.parse::<u64>().unwrap())
        .collect_vec()
        .try_into()
        .unwrap()
}

fn part_1(input: &str) -> u64 {
    let mut total = 0_u64;

    for line in input.lines() {
        let areas = parse_input(line).iter()
            .tuple_combinations()
            .map(|(a, b)| a * b)
            .collect_vec();

        total += areas.iter().min().unwrap();
        total += areas.iter().sum::<u64>() * 2;
    }

    total
}

fn part_2(input: &str) -> u64 {
    let mut total = 0_u64;

    for line in input.lines() {
        let mut lengths = parse_input(line);
        lengths.sort();

        total += &lengths[0..2].iter().sum::<u64>() * 2;
        total += lengths.iter().product::<u64>();
    }

    total
}