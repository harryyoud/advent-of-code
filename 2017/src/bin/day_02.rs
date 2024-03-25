use aoc_2017::get_input;
use itertools::Itertools;

fn main() {
    let input = get_input(2);

    dbg!(part_1(&input));
    dbg!(part_2(&input));
}

fn part_1(input: &str) -> u32 {
    input.lines()
        .map(parse_line)
        .map(min_max_diff)
        .sum()
}

fn part_2(input: &str) -> u32 {
    input.lines()
        .map(parse_line)
        .map(evenly_divisible_pair)
        .sum()
}

fn parse_line(line: &str) -> Vec<u32> {
    line.split_whitespace().map(|x| x.parse().unwrap()).collect_vec()
}

fn min_max_diff(row: Vec<u32>) -> u32 {
    row.iter().max().unwrap() - row.iter().min().unwrap()
}

fn evenly_divisible_pair(row: Vec<u32>) -> u32 {
    for (a, b) in row.iter().tuple_combinations() {
        if a % b == 0 {
            return a / b;
        } else if b % a == 0 {
            return b / a;
        }
    }
    panic!("No solution found");
}