use aho_corasick::AhoCorasick;
use aoc_2018::get_input;
use itertools::Itertools;
use rayon::prelude::*;

fn main() {
    let input = get_input(5);

    dbg!(part_1(&input));
    dbg!(part_2(&input));
}

fn part_1(input: &str) -> usize {
    replace_all(input).len()
}

fn part_2(input: &str) -> usize {
    ('a'..='z')
        .zip('A'..='Z')
        .par_bridge()
        .into_par_iter()
        .map(|x| input.replace([x.0, x.1], ""))
        .map(|x| replace_all(&x).len())
        .min()
        .unwrap()
}

fn replace_all(input: &str) -> String {
    let replacements = ('a'..='z')
        .zip('A'..='Z')
        .chain(('A'..='Z').zip('a'..='z'))
        .map(|x| format!("{}{}", x.0, x.1))
        .collect_vec();
    let ac = AhoCorasick::new(replacements).unwrap();

    let mut last = input.trim().to_string();
    loop {
        let mut next = String::with_capacity(last.len() / 2);
        ac.replace_all_with(&last, &mut next, |_, _, _| true);
        if next.len() == last.len() {
            break;
        }
        last = next;
    }

    last
}

#[test]
fn test_replace() {
    let inputs = ["aA", "abBA", "abAB", "aabAAB", "dabAcCaCBAcCcaDA"];
    let outputs = ["", "", "abAB", "aabAAB", "dabCBAcaDA"];

    for (input, output) in inputs.into_iter().zip(outputs) {
        assert_eq!(replace_all(input), output);
    }
}
