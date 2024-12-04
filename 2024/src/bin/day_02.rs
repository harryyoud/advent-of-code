use aoc_2024::{get_input, skip_nth::AocItertools};
use itertools::Itertools;

fn main() {
    let input = get_input(2);

    let reports = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .into_iter()
                .map(|y| y.parse::<u32>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    dbg!(part_1(&reports));
    dbg!(part_2(&reports));
}

fn part_1(reports: &[Vec<u32>]) -> usize {
    reports
        .into_iter()
        .filter(|x| report_is_safe(x.iter().copied()))
        .count()
}

fn part_2(reports: &[Vec<u32>]) -> usize {
    reports
        .into_iter()
        .filter(|report| {
            report_is_safe(report.iter().copied())
                || (0..report.len())
                    .any(|x| report_is_safe(report.into_iter().copied().skip_nth(x)))
        })
        .count()
}

fn report_is_safe(input: impl Iterator<Item = u32>) -> bool {
    let mut input = input.multipeek();
    let first = *input.peek().unwrap();
    let second = *input.peek().unwrap();
    let ascending = first < second;

    input.into_iter().tuple_windows().all(|(left, right)| {
        (left < right) == ascending && (1_u32..=3).contains(&left.abs_diff(right))
    })
}
