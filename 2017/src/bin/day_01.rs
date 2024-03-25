use std::iter;

use aoc_2017::get_input;
use itertools::Itertools;

fn main() {
    let input = get_input(1);
    let input = input.trim();

    dbg!(part_1(input));
    dbg!(part_2(input));
}

fn part_1(input: &str) -> u32 {
    let mut out: u32 = 0;

    for (a, b) in input.chars()
        .chain(iter::once(input.chars().next().unwrap()))
        .tuple_windows()
    {
        if a == b {
            out += a.to_digit(10).unwrap();
        }
    }
    out
}

fn part_2(input: &str) -> u32 {
    assert!(input.len() % 2 == 0);
    let left = input.chars();
    let right = input.chars()
        .cycle()
        .skip(input.len() / 2);

    let mut out: u32 = 0;

    for (a, b) in left.zip(right) {
        if a == b {
            out += a.to_digit(10).unwrap();
        }
    }
    out
}