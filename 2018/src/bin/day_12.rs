#![feature(linked_list_cursors)]

use std::collections::{HashMap, HashSet};

use aoc_2018::get_input;
use itertools::Itertools;

fn main() {
    let input = get_input(12);
    let (plants, transformations) = parse_input(&input);

    dbg!(part_1(&plants, &transformations));
    dbg!(part_2(&plants, &transformations));
}

fn part_1(plants: &HashSet<isize>, transformations: &HashMap<[bool; 5], bool>) -> isize {
    simulate(plants.clone(), transformations, 20)
        .into_iter()
        .sum::<isize>()
}

fn part_2(plants: &HashSet<isize>, transformations: &HashMap<[bool; 5], bool>) -> isize {
    // at iterations of 5 x 10^x, answer is 21...428, where ... is (x - 2) number of 0
    // therefore 5 x 10^10, answer is 2_100_000_000_428
    let x = simulate(plants.clone(), transformations, 5_000)
        .into_iter()
        .sum::<isize>();
    let (a, b): (usize, usize) = format!("{x}")
        .split_once('0')
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .unwrap();
    format!("{a}00000000{b}").parse().unwrap()
}

fn parse_input(input: &str) -> (HashSet<isize>, HashMap<[bool; 5], bool>) {
    let mut lines = input.lines();

    (
        lines
            .next()
            .unwrap()
            .trim_start_matches("initial state: ")
            .chars()
            .enumerate()
            .filter(|(_i, c)| *c == '#')
            .map(|(i, _c)| i as isize)
            .collect(),
        lines
            .skip(1)
            .map(|x| x.split_once(" => ").unwrap())
            .map(|(from, to)| {
                (
                    from.chars()
                        .map(|x| x == '#')
                        .collect_vec()
                        .try_into()
                        .unwrap(),
                    to == "#",
                )
            })
            .collect(),
    )
}

fn simulate(
    mut plants: HashSet<isize>,
    transformations: &HashMap<[bool; 5], bool>,
    iterations: usize,
) -> HashSet<isize> {
    let mut next_plants = HashSet::new();

    for i in 0..iterations {
        if i % 100_000 == 0 {
            dbg!(i);
        }
        for index in (plants.iter().min().unwrap() - 2)..(plants.iter().max().unwrap() + 2) {
            let window = ((index - 2)..=(index + 2))
                .map(|x| plants.contains(&x))
                .collect_vec()
                .try_into()
                .unwrap();
            if let Some(x) = transformations.get::<[bool; 5]>(&window) {
                if *x {
                    next_plants.insert(index);
                } else {
                    next_plants.remove(&index);
                }
            }
        }
        plants.clone_from(&next_plants);
        next_plants = HashSet::new();
    }

    plants
}
