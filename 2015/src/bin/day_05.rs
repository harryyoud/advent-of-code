use std::collections::HashMap;

use aho_corasick::AhoCorasick;
use aoc_2015::get_input;
use itertools::Itertools;

fn main() {
    let input = get_input(5);

    dbg!(part_1(&input));
    dbg!(part_2(&input));
}

fn is_nice_part_2(word: &str) -> bool {
    let mut found_pair = false;
    let mut pairs: HashMap<(char, char), Vec<usize>> = HashMap::new();
    for (pos, (a, b)) in word
        .chars()
        .tuple_windows()
        .enumerate() {
        pairs.entry((a, b))
            .and_modify(|x| x.push(pos))
            .or_insert(vec![pos]);
    }
    'outer: for ((_a, _b), positions) in pairs.iter().filter(|((_a, _b), positions)| positions.len() >= 2) {
        for (pos_a, pos_b) in positions.iter().tuple_combinations() {
            if pos_a.abs_diff(*pos_b) >= 2 {
                found_pair = true;
                break 'outer;
            }
        }
    }

    if !found_pair {
        return false;
    }

    let mut found_pair = false;
    for (a, _b, c) in word.chars().tuple_windows() {
        if a == c {
            found_pair = true;
            break
        }
    }

    found_pair
}

fn part_2(input: &str) -> u64 {
    input
        .lines()
        .map(is_nice_part_2)
        .filter(|x| *x)
        .count() as u64
}

fn is_nice_part_1(word: &str) -> bool {
    let bad_list = [
        "ab", "cd", "pq", "xy"
    ];
    let good_list = [
        "aa", "bb", "cc", "dd", "ee", "ff", "gg", "hh", "ii",
        "jj", "kk", "ll", "mm", "nn", "oo", "pp", "qq", "rr",
        "ss", "tt", "uu", "vv", "ww", "xx", "yy", "zz"
    ];
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let bad_list = AhoCorasick::new(bad_list).unwrap();
    let good_list = AhoCorasick::new(good_list).unwrap();

    if bad_list.find(word).is_some() {
        return false;
    };
    if good_list.find(word).is_none() {
        return false;
    }

    word.chars().filter(|x| vowels.contains(x)).count() >= 3
}

fn part_1(input: &str) -> u64 {
    input
        .lines()
        .map(is_nice_part_1)
        .filter(|x| *x)
        .count() as u64
}