use std::collections::HashMap;
use bitvec::prelude::*;
use itertools::Itertools;
use lazy_regex::regex_captures;
use aoc_2020::get_input;

const WIDTH: usize = 36;
type U36 = BitVec::<u64, Msb0>;

fn main() {
    let input = get_input(14);

    // massage input into Vec<(mask, Vec<(address, value)>)>
    //                         &str       |-u64-|  |u64|
    let input = input.split("mask = ")
        .filter(|x| !x.is_empty())
        .map(|x| {
            let mut iter = x.lines();
            (iter.next().unwrap(), iter.map(|line| {
                let (_, address, value) = regex_captures!(r#"^mem\[(\d+)\] = (\d+)$"#, line).unwrap();
                (address.parse::<u64>().unwrap(), value.parse::<u64>().unwrap())
            }).collect_vec())
        })
        .collect_vec();

    dbg!(part_1(&input));
    dbg!(part_2(&input));
}

fn part_1(input: &[(&str, Vec<(u64, u64)>)]) -> u64 {
    let mut memory = HashMap::new();

    for (mask, instructions) in input {
        let (set_mask, unset_mask) = split_mask(mask);
        for (address, mut value) in instructions {
            value |= set_mask;
            value &= unset_mask;
            *memory.entry(address).or_default() = value;
        }
    }

    memory.values().sum::<u64>()
}

fn part_2(input: &[(&str, Vec<(u64, u64)>)]) -> u64 {
    let mut memory = HashMap::new();

    for (mask, instructions) in input {
        for (original_address, value) in instructions {
            for address in decode_address(*original_address, mask) {
                *memory.entry(address).or_default() = *value;
            }
        }
    }

    memory.values().sum::<u64>()
}

fn from_binary(input: &str) -> u64 {
    u64::from_str_radix(input, 2).unwrap()
}

fn split_mask(input: &str) -> (u64, u64) {
    assert_eq!(input.len(), WIDTH, "Mask string representation must be {WIDTH} bits wide exactly");

    let set_mask = input.replace('X', "0");
    let unset_mask = input.replace('X', "1");

    (from_binary(&set_mask), from_binary(&unset_mask))
}

fn decode_address(original_address: u64, mask: &str) -> Vec<u64> {
    mask
        .chars()
        .zip(U36::from_element(original_address)[(64 - WIDTH)..].into_iter())
        .map(|(mask_char, addr_bit)| match mask_char {
            '0' => vec![*addr_bit],     // keep as original
            '1' => vec![true],        // override
            'X' => vec![false, true], // "floating"
            _ => panic!("Invalid input"),
        })
        .multi_cartesian_product()
        .map(|x| U36::from_iter(x.into_iter()).load())
        .collect_vec()
}
