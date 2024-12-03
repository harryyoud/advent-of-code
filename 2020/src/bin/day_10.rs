use std::collections::HashMap;
use itertools::Itertools;
use aoc_2020::get_input;

fn main() {
    let input = get_input(10);
    let mut adapters = input.lines()
        .map(|x| x.parse::<u32>().unwrap())
        .collect_vec();

    adapters.push(0);
    adapters.push(adapters.iter().max().unwrap() + 3);
    adapters.sort();

    dbg!(part_1(&adapters));
    dbg!(part_2(&adapters));
}

fn part_1(adapters: &[u32]) -> u32 {
    let mut diffs = HashMap::<u32, u32>::new();
    for (left, right) in adapters.iter().tuple_windows() {
        if right - left > 3 {
            panic!("Cannot jump from jolt of {left} to {right}")
        }
        *diffs.entry(right - left).or_default() += 1;
    }

    diffs.get(&1).unwrap() * diffs.get(&3).unwrap()
}

fn part_2(adapters: &[u32]) -> u64 {
    adapters.into_iter()
        .tuple_windows()
        .collect_vec()
        .split(|(x, y)| **y - **x == 3)
        .filter(|x| x.len() > 1)
        .map(|n| 2u64.pow(n.len() as u32 - 1) - (n.len() as u64 / 4))
        .product::<u64>()
}