use aoc_2017::{get_input, knot_hasher::{bytes_to_hex, hash, twist}};
use itertools::Itertools;

fn main() {
    let input = get_input(10);

    dbg!(part_1(&input));
    dbg!(part_2(&input));
}

fn part_1(input: &str) -> usize {
    let instructions = input
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect_vec();

    let result = twist(&instructions, 1);
    result[0] as usize * result[1] as usize
}

fn part_2(plaintext: &str) -> String {
    bytes_to_hex(&hash(plaintext))
}

