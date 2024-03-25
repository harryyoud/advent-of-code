use aoc_2023::get_input;
use itertools::Itertools;

fn main() {
    let input = get_input(9);
    dbg!(part_a(&input));
    dbg!(part_b(&input));
}

fn calculate_forwards_sequence(sequence: &Vec<i32>) -> Vec<i32> {
    sequence.iter().tuple_windows::<(&i32, &i32)>().map(|(a, b)| b - a).collect()
}

fn calculate_increase_sequence_n(sequence: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut out = vec![sequence.clone()];
    loop {
        let next = out.last().unwrap();
        if next.iter().all(|x| x == &0) {
            break;
        }
        out.push(calculate_forwards_sequence(next));
    }
    out
}

fn get_next_in_top_sequence(sequences: Vec<Vec<i32>>) -> i32 {
    let mut last_extra = 0_i32;
    for sequence in sequences.iter().rev().skip(1) {
        last_extra += sequence.last().unwrap();
    }
    last_extra
}

fn part_a(input: &str) -> i32 {
    input.lines().map(|s| s.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect_vec()).map(|sequence| get_next_in_top_sequence(calculate_increase_sequence_n(&sequence))).sum()
}

fn part_b(input: &str) -> i32 {
    input.lines().map(|s| s.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect_vec()).map(|sequence| {
        get_next_in_top_sequence(calculate_increase_sequence_n(&sequence.into_iter().rev().collect_vec()))
    }).sum()
}
