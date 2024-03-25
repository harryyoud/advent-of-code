use std::collections::HashMap;

use aho_corasick::AhoCorasick;
use aoc_2023::get_input;
use lazy_static::lazy_static;

lazy_static! {
    static ref MAP: HashMap<String, u32> = {
        let mut h = HashMap::new();
        for i in 0..=9 {
            h.insert(i.to_string(), i);
        }
        for (l, n) in [
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9)
        ] {
            h.insert(l.to_string(), n);
        }
        h
    };
    static ref AC: AhoCorasick = AhoCorasick::new(MAP.keys()).unwrap();
}

fn main() {
    let input = get_input(1);
    let numerics = input.lines().fold(0u32, |acc, line| acc + extract_calibration_value_numerical(line));
    let alphanumerics = input.lines().fold(0u32, |acc, line| acc + extract_calibration_value_alphanumeric(line));
    println!("Interpreting digits only: {}", numerics);
    println!("Interpreting digits and words: {}", alphanumerics);
}

fn extract_calibration_value_numerical(line: &str) -> u32 {
    let numbers: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
    (numbers.first().unwrap() * 10) + numbers.last().unwrap()
}

fn extract_calibration_value_alphanumeric(line: &str) -> u32 {
    let numbers: Vec<u32> = AC.find_overlapping_iter(line).map(|mat|
        *MAP.get(&line[mat.start()..mat.end()]).unwrap()
    ).collect();
    (numbers.first().unwrap() * 10) + numbers.last().unwrap()
}



#[test]
fn test_extraction1() {
    let tests = [
        ("1abc2", 12),
        ("pqr3stu8vwx", 38),
        ("a1b2c3d4e5f", 15),
        ("treb7uchet", 77),
    ];

    let mut total = 0;

    for test in tests {
        let val = extract_calibration_value_numerical(test.0);
        total += val;
        assert_eq!(val, test.1, "extracted from \"{}\"", test.0);
    }

    assert_eq!(total, 142);
}

#[test]
fn test_extraction2() {
    let tests = [
        ("two1nine", 29),
        ("eightwothree", 83),
        ("abcone2threexyz", 13),
        ("xtwone3four", 24),
        ("4nineeightseven2", 42),
        ("zoneight234", 14),
        ("7pqrstsixteen", 76),
    ];

    let mut total = 0;

    for test in tests {
        let val = extract_calibration_value_alphanumeric(test.0);
        total += val;
        assert_eq!(val, test.1, "extracted from \"{}\"", test.0);
    }

    assert_eq!(total, 281);
}
