use std::collections::{HashMap, VecDeque};

use aoc_2015::get_input;
use itertools::Itertools;

const TOTAL_LITRES: i64 = 150;
// const TOTAL_LITRES: i64 = 25;

fn main() {
    let input = get_input(17);

    let containers = input
        .lines()
        .map(|s| s.parse::<i64>().unwrap())
        .collect_vec();

    let results = solve_iterative(&containers);

    dbg!(part_1(&results));
    dbg!(part_2(&results));
}

fn part_1(results: &HashMap<u64, u64>) -> u64 {
    results.values().sum()
}

fn part_2(results: &HashMap<u64, u64>) -> u64 {
    results.iter()
        .min_by_key(|(k, _v)| **k)
        .map(|(_k, v)| *v)
        .unwrap()
}

fn solve_iterative(containers: &[i64]) -> HashMap<u64, u64> {
    assert!(!containers.is_empty());

    let mut out = HashMap::new();
    let mut to_visit: VecDeque<(Vec<usize>, i64)> = VecDeque::new();

    to_visit.push_back((vec![], TOTAL_LITRES));

    while let Some((stack, remaining)) = to_visit.pop_front() {
        let candidates = (0..containers.len())
            .filter(|x| x >= stack.last().unwrap_or(&0))
            .filter(|x| !stack.contains(x));

        for candidate in candidates {
            let next = remaining - containers[candidate];
            if next == 0 {
                *out.entry(stack.len() as u64 + 1).or_default() += 1;
                continue;
            }
            if next < 0 {
                continue;
            }

            let mut new_stack = stack.clone();
            new_stack.push(candidate);
            new_stack.sort();
            to_visit.push_back((new_stack, next));
        }
    }

    out
}