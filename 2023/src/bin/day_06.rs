use std::str::Lines;

use aoc_2023::get_input;
use itertools::Itertools;


fn main() {
    let input = get_input(6);
    part_a(&input);
    part_b(&input);
}
fn part_a(input: &str) {
    let records = parse_lines_a(input.lines());
    let mut part_a = 1u64;
    for record in records {
        part_a *= get_possibilities(record.0, record.1);
    }
    dbg!(part_a);
}

fn part_b(input: &str) {
    let record = parse_lines_b(input.lines());
    let part_b = get_possibilities(record.0, record.1);
    dbg!(part_b);
}

fn parse_lines_a(mut lines: Lines) -> Vec<(u64, u64)> {
    lines.next().unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .zip(
            lines.next().unwrap()
            .split_whitespace()
            .skip(1)
            .map(|s| s.parse::<u64>().unwrap())
        )
    .collect_vec()
}

fn parse_lines_b(mut lines: Lines) -> (u64, u64) {
    (
        lines.next().unwrap()
            .split(":")
            .skip(1)
            .collect::<String>()
            .replace(" ", "")
            .parse::<u64>().unwrap(),
        lines.next().unwrap()
            .split(":")
            .skip(1)
            .collect::<String>()
            .replace(" ", "")
            .parse::<u64>().unwrap(),

    )
}


fn get_possibilities(race_time: u64, distance_record: u64) -> u64 {
    let mut times_beaten = 0_u64;
    for time_holding_button in 1..race_time {
        let time_remaining = race_time - time_holding_button;
        let velocity = time_holding_button;
        if time_remaining * velocity > distance_record {
            times_beaten += 1;
        }
    }
    times_beaten
}