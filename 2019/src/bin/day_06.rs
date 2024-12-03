use std::collections::HashMap;
use itertools::Itertools;
use pathfinding::prelude::dijkstra;
use aoc_2019::get_input;

const CENTRE: &str = "COM";

fn main() {
    let input = get_input(6);
    let mut orbits = HashMap::<&str, Vec<&str>>::new();
    let mut inverse_orbits = HashMap::<&str, Vec<&str>>::new();

    for line in input.lines() {
        let s = line.split_once(")").expect(&format!("Invalid input: {line}"));
        orbits.entry(s.0).or_default().push(s.1);
        inverse_orbits.entry(s.1).or_default().push(s.0);
    }

    dbg!(part_1(&orbits));
    dbg!(part_2(&orbits, &inverse_orbits));
}

fn part_1(orbits: &HashMap<&str, Vec<&str>>) -> usize {
    let mut count = 0;
    recurse_count(&orbits, CENTRE, 1, &mut count);
    count
}

fn part_2(
    orbits: &HashMap<&str, Vec<&str>>,
    inverse_orbits: &HashMap<&str, Vec<&str>>
) -> usize {
    find_shortest_path("YOU", "SAN", orbits, inverse_orbits)
        .expect("Unable to find a path").1 - 2
}

fn find_shortest_path<'a>(
    start: &'a str,
    end: &str,
    orbits: &'a HashMap<&str, Vec<&str>>,
    inverse_orbits: &'a HashMap<&str, Vec<&str>>
) -> Option<(Vec<&'a str>, usize)> {
    dijkstra(
        &start,
        |x| orbits.get(x).unwrap_or(&vec![]).into_iter()
            .chain(inverse_orbits.get(x).unwrap_or(&vec![]))
            .map(|y| (*y, 1))
            .collect_vec(),
        |x| *x == end
    )
}

fn recurse_count(
    direct_orbits: &HashMap<&str, Vec<&str>>,
    current: &str,
    depth: usize,
    count: &mut usize
) {
    for orbitee in direct_orbits.get(current).unwrap_or(&vec![]) {
        *count += depth;
        recurse_count(direct_orbits, orbitee, depth + 1, count);
    }
}
