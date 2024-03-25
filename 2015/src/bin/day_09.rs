use std::{collections::{HashMap, HashSet}, hash::Hash};

use aoc_2015::get_input;
use itertools::Itertools;

struct Connection<'a>(&'a str, &'a str);

impl Hash for Connection<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.0.min(self.1)).hash(state);
        (self.0.max(self.1)).hash(state);
    }
}

impl PartialEq for Connection<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.0.min(self.1) == other.0.min(other.1) &&
        self.0.max(self.1) == other.0.max(other.1)
    }
}

impl Eq for Connection<'_> {}

impl<'a> From<(&'a str, &'a str)> for Connection<'a> {
    fn from((a, b): (&'a str, &'a str)) -> Self {
        Self(a, b)
    }
}

fn main() {
    let input = get_input(9);
    let (set, map) = parse_input(&input);

    let (part_1, part_2) = calculate_routes(&set, &map);
    dbg!(part_1);
    dbg!(part_2);
}

fn calculate_routes(set: &HashSet<&str>, map: &HashMap<Connection, u32>) -> (u32, u32) {
    let mut minimum = u32::MAX;
    let mut maximum = u32::MIN;
    for path in set.iter().permutations(set.len()) {
        let cost = path.iter().tuple_windows().map(|(a, b)| {
            map.get(&(**a, **b).into()).unwrap()
        }).sum();
        minimum = minimum.min(cost);
        maximum = maximum.max(cost);
    }
    (minimum, maximum)
}

fn parse_input<'a>(input: &'a str) -> (HashSet<&'a str>, HashMap<Connection<'a>, u32>) {
    let mut map = HashMap::new();
    let mut set = HashSet::new();
    input.lines().map(parse_line).for_each(|(a, b, dist)| {
        set.insert(a);
        set.insert(b);
        map.insert((a, b).into(), dist);
    });
    (set, map)
}

fn parse_line<'a>(line: &'a str) -> (&'a str, &'a str, u32) {
    let (dests, distance) = line.split(" = ").collect_tuple().unwrap();
    let (from, to) = dests.split(" to ").collect_tuple().unwrap();
    (from, to, distance.parse().unwrap())
}