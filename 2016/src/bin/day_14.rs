use std::{collections::HashMap, sync::{Arc, RwLock}};

use aoc_2016::get_input;
use itertools::{iterate, Itertools};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() {
    let input = get_input(14);
    let input = input.trim();

    dbg!(part_1(input));
    dbg!(part_2(input));
}

fn part_1(input: &str) -> u64 {
    solve(input, 1)
}

fn part_2(input: &str) -> u64 {
    solve(input, 2017)
}

fn solve(input: &str, recursions: usize) -> u64 {
    let mut out = vec![];

    let cache = Arc::new(RwLock::new(HashMap::new()));

    for i in 0.. {
        let hash = calculate_hash_recursive(&format!("{input}{i}"), recursions, cache.clone());
        if let Some(c) = first_pass(&hash) {
            if ((i + 1)..=(i + 1000)).into_par_iter().find_map_first(|x| {
                if second_pass(&calculate_hash_recursive(&format!("{input}{x}"), recursions, cache.clone()), c) {
                    Some(())
                } else {
                    None
                }
            }).is_some() {
                out.push(i);
            }
        }
        if out.len() == 64 {
            break;
        }
    }
    *out.last().unwrap()
}

fn calculate_hash_recursive(plaintext: &str, number_of_recursions: usize, cache: Arc<RwLock<HashMap<String, md5::Digest>>>) -> String {
    let plaintext = plaintext.to_owned();
    if let Some(s) = cache.read().unwrap().get(&plaintext) {
        return format!("{:x}", s);
    }
    let out = iterate(
        md5::compute(plaintext.clone()),
        |x| {
            md5::compute(format!("{x:x}"))
        }
    ).nth(number_of_recursions - 1).unwrap();
    cache.write().unwrap().insert(plaintext, out);
    format!("{out:x}")
}

fn first_pass(hash: &str) -> Option<char> {
    hash
        .chars()
        .tuple_windows()
        .filter(|(a, b, c)| a == b && b == c)
        .map(|(a, _b, _c)| a)
        .next()
}

fn second_pass(hash: &str, c: char) -> bool {
    hash
        .chars()
        .tuple_windows::<(_, _, _, _, _)>()
        .any(|x| x == (c, c, c, c, c))
}