use aoc_2018::get_input;
use itertools::Itertools;

fn main() {
    let input = get_input(1);
    let operations = parse_input(&input);

    dbg!(part_1(&operations));
    dbg!(part_2(&operations));
}

fn part_1(operations: &[isize]) -> isize {
    operations.iter().sum()
}

fn part_2(operations: &[isize]) -> isize {
    let mut i = 0;
    operations.iter()
        .cycle()
        .map(|x| {
            i += *x;
            i
        })
        .duplicates()
        .next()
        .unwrap()
}

fn parse_input(input: &str) -> Vec<isize> {
    input.lines()
        .map(|line| line.trim_start_matches('+'))
        .map(|line| line.parse().unwrap())
        .collect_vec()
}
