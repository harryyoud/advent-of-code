#![feature(coroutines)]
#![feature(iter_from_coroutine)]

use std::collections::VecDeque;
use itertools::Itertools;
use aoc_2020::get_input;

fn main() {
    let input = get_input(9);
    let input = input.lines()
        .map(|x| x.parse::<u64>().expect("Invalid number"))
        .collect_vec();

    let part_1 = dbg!(part_1(&input));
    dbg!(part_2(part_1, &input));
}

fn part_1(input: &[u64]) -> u64 {
    let mut input = input.into_iter();
    let mut to_consider = input.by_ref().copied().take(25).collect::<VecDeque<_>>();

    while let Some(candidate) = input.next() {
        if !to_consider.iter().copied().tuple_combinations().any(|(x, y)| x + y == *candidate) {
            return *candidate
        }
        to_consider.pop_front();
        to_consider.push_back(*candidate);
    }

    unreachable!("No solution found")
}

// Strategy: step through input in increasing window size (> 2) until we find a set that sums to
//           value from part_1
fn part_2(part_1: u64, input: &[u64]) -> u64 {
    for n in 2..input.len() {
        for x in input.windows(n) {
            if x.iter().any(|y| *y > part_1) {
                continue;
            }
            if x.iter().sum::<u64>() == part_1 {
                return x.iter().min().unwrap() + x.iter().max().unwrap();
            }
        }
    }

    unreachable!("No solution found")
}

