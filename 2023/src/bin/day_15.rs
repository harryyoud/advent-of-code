use std::iter;

use aoc_2023::get_input;
use itertools::Itertools;

fn main() {
    let input = get_input(15);
    dbg!(part_a(&input));
    dbg!(part_b(&input));
}

fn part_a(input: &str) -> u32 {
    input.lines().next().unwrap().trim().split(',').map(get_hash_for_str).sum()
}

fn part_b(input: &str) -> u32 {
    let mut boxes: [Vec<(&str, u8)>; 256] = iter::repeat(Vec::new()).take(256).collect::<Vec<_>>().try_into().unwrap();
    for step in input.lines().next().unwrap().trim().split(',') {
        if step.contains('=') {
            let (label, focal_len) = step.split('=').collect_tuple().unwrap();
            let label_hash = get_hash_for_str(label) as usize;
            let focal_len = focal_len.parse::<u8>().unwrap();

            if let Some(position) = boxes[label_hash].iter().position(|(lab, _focal_len)| *lab == label) {
                boxes[label_hash][position] = (label, focal_len);
            } else {
                boxes[label_hash].push((label, focal_len));
            }

        } else if step.contains('-') {
            let label = step.trim_end_matches('-');
            let label_hash = get_hash_for_str(label);
            boxes[label_hash as usize].retain(|(lab, _f_l)| *lab != label);
        } else {
            panic!("not = or -");
        }
    }

    boxes.iter().enumerate().map(|(box_num, boxx)| {
        boxx.iter()
            .enumerate()
            .map(|(slot_num, (_label, focal_len))| {
                (box_num as u32 + 1) * (slot_num as u32 + 1) * (*focal_len as u32)
            })
            .sum::<u32>()
    }).sum::<u32>()
}

fn get_byte_hash(current_value: &mut u32, byte: u8) {
    *current_value += byte as u32;
    *current_value *= 17;
    *current_value %= 256;
}

fn get_hash_for_str(input: &str) -> u32 {
    let mut current_value = 0u32;
    for byte in input.bytes() {
        get_byte_hash(&mut current_value, byte);
    }
    current_value
}