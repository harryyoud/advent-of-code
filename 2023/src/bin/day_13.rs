
use aoc_2023::get_input;
use aoc_lib::Paragraphs;
use itertools::Itertools;

fn main() {
    let input = get_input(13);
    
    let mut part_a = 0u32;
    let mut part_b = 0u32;

    for puzzle in input.paragraphs().map(|p| p.iter().map(|l| l.chars().collect_vec()).collect_vec()) {
        if let Some(horizontal_reflection_idx) = get_reflection_a(&puzzle) {
            part_a += horizontal_reflection_idx * 100
        } else if let Some(vertical_reflection_idx) = get_reflection_a(&transpose(puzzle.clone())) {
            part_a += vertical_reflection_idx;
        } else {
            panic!("No reflection found");
        }
        if let Some(horizontal_reflection_idx) = get_reflection_b(&puzzle) {
            part_b += horizontal_reflection_idx * 100
        } else if let Some(vertical_reflection_idx) = get_reflection_b(&transpose(puzzle)) {
            part_b += vertical_reflection_idx;
        } else {
            panic!("No reflection found");
        }

    }

    dbg!(part_a);
    dbg!(part_b);
}

fn get_reflection_a(lines: &Vec<Vec<char>>) -> Option<u32> {
    for ((a_idx, a), (b_idx, b)) in lines.iter().enumerate().tuple_windows() {
        if a != b {
            continue
        }
        if lines.iter().rev().skip(lines.len() - 1 - a_idx)
            .zip(lines.iter().skip(b_idx))
            .map(|(a, b)| a == b).all(|x| x) {
            return Some(b_idx as u32);
        } else {
            continue;
        }
    }
    None
}

fn get_reflection_b(lines: &Vec<Vec<char>>) -> Option<u32> {
    for ((a_idx, a), (b_idx, b)) in lines.iter().enumerate().tuple_windows() {
        if a != b && !is_similar(a, b) {
            continue
        }
        let mut identities = lines.iter().rev().skip(lines.len() - 1 - a_idx)
            .zip(lines.iter().skip(b_idx))
            .map(|(a, b)| (a == b, is_similar(a, b)));
        if identities.clone().filter(|(_same, similar)| *similar).count() == 1 &&
            identities.all(|(similar, same)| similar || same) {
            return Some(b_idx as u32);
        } else {
            continue;
        }
    }
    None
}

fn is_similar(line_a: &Vec<char>, line_b: &Vec<char>) -> bool {
    // if only one difference, then true, otherwise false
    line_a.iter().zip(line_b).filter_map(|(a, b)| (a != b).then(|| ())).count() == 1
}

fn transpose<T: Clone>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    (0..v[0].len()).map(|i|
        v.iter().map(|inner|
            inner[i].clone()
        ).collect::<Vec<T>>()
    ).collect()
}
