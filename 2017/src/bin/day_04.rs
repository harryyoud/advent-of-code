use aoc_2017::get_input;
use itertools::Itertools;

fn main() {
    let input = get_input(4);

    dbg!(part_1(&input));
    dbg!(part_2(&input));
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .filter(|x| !contains_repeated_words(x))
        .count()
}

fn part_2(input: &str) -> usize {
    input
        .lines()
        .filter(|x| !contains_anagrams(x))
        .count()
}

fn contains_repeated_words(line: &str) -> bool {
    line
        .split_whitespace()
        .counts()
        .iter()
        .any(|(_, count)| *count > 1)
}

fn contains_anagrams(line: &str) -> bool {
    line
        .split_whitespace()
        .map(|word| word.chars().sorted().collect_vec())
        .counts()
        .iter()
        .any(|(_, count)| *count > 1)
}