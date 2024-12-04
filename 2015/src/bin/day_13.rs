use std::collections::{HashMap, HashSet};

use aoc_2015::get_input;
use itertools::Itertools;

fn main() {
    let input = get_input(13);

    let (mut set, mut map) = parse_input(&input);
    dbg!(part_1(&set, &map));

    for person in set.iter() {
        map.insert(("Me", person), 0);
        map.insert((person, "Me"), 0);
    }
    set.insert("Me");

    dbg!(part_2(&set, &map));
}

fn part_1(set: &HashSet<&str>, map: &HashMap<(&str, &str), i32>) -> i32 {
    let mut maximum = i32::MIN;
    for arrangement in set.iter().copied().permutations(set.len()) {
        maximum = maximum.max(calculate_total_happiness(&arrangement, map));
    }
    maximum
}

fn part_2(set: &HashSet<&str>, map: &HashMap<(&str, &str), i32>) -> i32 {
    part_1(set, map)
}

fn parse_input(input: &str) -> (HashSet<&str>, HashMap<(&str, &str), i32>) {
    let mut map = HashMap::new();
    let mut set = HashSet::new();
    for line in input.lines() {
        let line = line.trim_end_matches('.');
        let (lhs, next_to) = line
            .split(" happiness units by sitting next to ")
            .collect_tuple()
            .unwrap();
        let (person, _would, gain_or_lose, amount) =
            lhs.split_whitespace().collect_tuple().unwrap();
        let mut amount = amount.parse::<i32>().unwrap();
        amount *= match gain_or_lose {
            "gain" => 1,
            "lose" => -1,
            _ => panic!("Invalid {gain_or_lose} at word position 3"),
        };
        map.insert((person, next_to), amount);
        set.insert(person);
        set.insert(next_to);
    }
    (set, map)
}

fn calculate_total_happiness(arrangement: &[&str], map: &HashMap<(&str, &str), i32>) -> i32 {
    let mut total = 0;
    for (a, b) in arrangement
        .iter()
        .chain(std::iter::once(&arrangement[0]))
        .tuple_windows()
    {
        total += map.get(&(a, b)).unwrap();
        total += map.get(&(b, a)).unwrap();
    }
    total
}
