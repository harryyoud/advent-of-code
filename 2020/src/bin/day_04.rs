use std::collections::HashMap;
use itertools::Itertools;
use lazy_regex::{regex, regex_captures};
use aoc_2020::get_input;

const REQUIRED_KEYS: [&str; 7] = [
    "byr", // Birth Year
    "iyr", // Issue Year
    "eyr", // Expiration Year
    "hgt", // Height
    "hcl", // Hair Color
    "ecl", // Eye Color
    "pid", // Passport ID
];

const CHECKS: [(&str, fn(&str) -> bool); 7] = [
    ("byr", |x| (1920..=2002).contains(&x.parse::<u32>().unwrap_or(0))),
    ("iyr", |x| (2010..=2020).contains(&x.parse::<u32>().unwrap_or(0))),
    ("eyr", |x| (2020..=2030).contains(&x.parse::<u32>().unwrap_or(0))),
    ("hgt", |x| {
        let Some((_, num, unit)) = regex_captures!(r#"^([0-9]+)(in|cm)$"#, x) else { return false };
        match unit {
            "in" => (59..=76).contains(&num.parse().unwrap()),
            "cm" => (150..=193).contains(&num.parse().unwrap()),
            _ => false
        }

    }),
    ("hcl", |x| regex!(r#"^#[0-9a-f]{6}$"#).is_match(x)),
    ("ecl", |x| ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&x)),
    ("pid", |x| regex!(r#"^[0-9]{9}$"#).is_match(x)),
];

type Passport<'a> = HashMap<&'a str, &'a str>;

fn main() {
    let input = get_input(4);

    let passports = parse_passports(&input);
    dbg!(part_1(&passports));
    dbg!(part_2(&passports));
}

fn part_1(passports: &[Passport]) -> usize {
    passports.iter()
        .filter(|x| has_required_keys(x))
        .count()
}

fn part_2(passports: &[Passport]) -> usize {
    passports.iter()
        .filter(|x| passes_strict_check(x))
        .count()
}

fn has_required_keys(passport: &Passport) -> bool {
    REQUIRED_KEYS.into_iter().all(|x| passport.contains_key(x))
}

fn passes_strict_check(passport: &Passport) -> bool {
    CHECKS.iter().all(|(field, check)| {
        check(passport.get(field).unwrap_or(&""))
    })
}

fn parse_passports(input: &str) -> Vec<Passport> {
    input.split("\n\n")
        .map(|x| parse_passport(&x))
        .collect_vec()
}

fn parse_passport(input: &str) -> Passport {
    let r = regex!(r#"([a-z]{3}:#?[0-9a-z]+)"#);

    r.find_iter(input)
        .map(|x| x.as_str().split_once(':').unwrap())
        .collect()
}