use aoc_2016::get_input;
use itertools::{iterate, Itertools};

fn main() {
    let input = get_input(16);
    let input = input.trim();

    dbg!(part_1(input));
    dbg!(part_2(input));
}

fn part_1(input: &str) -> String {
    calculate_checksum(&fill_until(input, 272))
}

fn part_2(input: &str) -> String {
    calculate_checksum(&fill_until(input, 35651584))
}

fn fill_until(input: &str, disk_len: usize) -> String {
    let mut out = iterate(
            input.to_owned(),
            |x| build_string(x)
        )
        .skip_while(|x: &String| x.len() < disk_len)
        .next()
        .unwrap();
    out.truncate(disk_len);
    out
}

fn build_string(input: &str) -> String {
    let a = input;
    let b = input
        .chars()
        .rev()
        .map(|c| {
            if c == '0' { '1' } else if c == '1' { '0' } else { panic!("char not 0 or 1") }
        })
        .collect::<String>();
    format!("{a}0{b}")
}

fn calculate_checksum(input: &str) -> String {
    let mut input = input.to_owned();
    loop {
        let mut out = String::with_capacity(input.len() / 2);
        for (a, b) in input.chars().tuples() {
            if a == b {
                out.push('1');
            } else {
                out.push('0')
            }
        }
        input = out;
        if input.len() % 2 == 1 {
            return input;
        }
    }
}
