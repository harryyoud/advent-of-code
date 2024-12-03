use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use lazy_regex::regex_captures;
use pathfinding::prelude::dijkstra_reach;
use aoc_2020::get_input;

fn main() {
    let input = get_input(7);

    let (can_go_into, must_contain) = parse_input(&input);

    dbg!(part_1(&can_go_into));
    dbg!(part_2(&must_contain));
}

fn part_1(can_go_into: &HashMap<&str, HashSet<&str>>) -> usize {
    dijkstra_reach(
        &"shiny gold",
        |x, _| {
            let Some(next) = can_go_into.get(x) else { return vec![].into_iter() };
            next.iter().map(|y| (*y, 1)).collect_vec().into_iter()
        }
    ).filter(|item| item.total_cost > 0).count()
}

fn part_2(must_contain: &HashMap<&str, HashSet<(u32, &str)>>) -> u32 {
    let mut count = 0;
    recursive_search(&mut count, 1, "shiny gold", must_contain);
    count
}

fn recursive_search(count: &mut u32, multiplier: u32, current: &str, must_contain: &HashMap<&str, HashSet<(u32, &str)>>) {
    for (quantity, color) in must_contain.get(current).unwrap_or(&HashSet::new()) {
        *count += multiplier * *quantity;
        recursive_search(count, multiplier * *quantity, color, must_contain);
    }
}

fn parse_input(
    input: &str
) -> (
    HashMap<&str, HashSet<&str>>,
    HashMap<&str, HashSet<(u32, &str)>>,
) {
    let mut can_go_into: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut must_contain: HashMap<&str, HashSet<(u32, &str)>> = HashMap::new();

    for line in input.lines() {
        let (bag_color, contents) = line.trim_end_matches(".").split_once(" bags contain ")
            .expect(&format!("Unexpected format for line: {line}"));
        for (_, quantity, color) in contents
            .split(", ")
            .filter(|x| *x != "no other bags")
            .map(|x| {
                regex_captures!(r#"^(\d) (\w+ \w+) bags?$"#, x)
                    .expect(&format!("Unexpected format for contents: {x}"))
            }) {
            can_go_into.entry(color).or_default().insert(bag_color);
            must_contain.entry(bag_color).or_default().insert((quantity.parse().unwrap(), color));
        }
    }

    (can_go_into, must_contain)
}