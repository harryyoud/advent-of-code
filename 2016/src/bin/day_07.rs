use std::collections::HashSet;

use aoc_2016::get_input;
use itertools::Itertools;

fn main() {
    let input = get_input(7);
    dbg!(part_1(&input));
    dbg!(part_2(&input));
}

fn part_1(input: &str) -> u32 {
    input.lines().filter(supports_tls).count() as u32
}

fn part_2(input: &str) -> u32 {
    input.lines().filter(supports_ssl).count() as u32
}

fn supports_tls(line: &&str) -> bool {
    let mut in_hypernet_seq = false;
    let mut passes: bool = false;
    for slice in line.split(&['[', ']']) {
        let contains_abba = {
            for (a, b, c, d) in slice.chars().tuple_windows() {
                if a == d && b == c && a != b {
                    return true;
                }
            }
            false
        };
        if in_hypernet_seq && contains_abba {
            return false;
        } else if contains_abba {
            passes = true;
        }
        in_hypernet_seq = !in_hypernet_seq
    }
    passes
}

fn supports_ssl(line: &&str) -> bool {
    let mut abas = HashSet::new();
    let mut babs = HashSet::new();

    let mut in_hypernet_seq = false;
    for slice in line.split(&['[', ']']) {
        for (x, y, z) in slice.chars().tuple_windows() {
            if x == z && x != y {
                if in_hypernet_seq {
                    babs.insert((y, x, y));
                } else {
                    abas.insert((x, y, x));
                }
            }
        }
        in_hypernet_seq = !in_hypernet_seq
    }

    abas.intersection(&babs).count() > 0
}
