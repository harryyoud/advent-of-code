use std::{collections::HashSet, iter};

use aoc_2017::{count_groups, get_input};
use itertools::Itertools;
use pathfinding::directed::dijkstra::dijkstra_all;

const PROGRAM_COUNT: usize = 2000;

fn main() {
    let input = get_input(12);
    let programs = parse_input(&input);

    dbg!(part_1(&programs));
    dbg!(part_2(&programs));
}

fn part_1(programs: &[Program]) -> usize {
    dijkstra_all(&0_usize, |x| {
        programs[*x].connects_to.iter().map(|a| (*a, 1))
    })
    .len()
        + 1
}

fn part_2(programs: &[Program]) -> usize {
    let to_visit = HashSet::<usize>::from_iter(0..PROGRAM_COUNT);
    let f = |program: &usize| -> Vec<usize> {
        programs[*program].connects_to.iter().copied().collect_vec()
    };

    count_groups(to_visit, f)
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Program {
    connects_to: HashSet<usize>,
}

impl Program {
    fn new() -> Self {
        Program {
            connects_to: HashSet::new(),
        }
    }
}

fn parse_input(input: &str) -> [Program; PROGRAM_COUNT] {
    assert_eq!(
        input.lines().count(),
        PROGRAM_COUNT,
        "Expected 2000 programs in input"
    );

    let mut programs: [Program; PROGRAM_COUNT] = iter::repeat(Program::new())
        .take(PROGRAM_COUNT)
        .collect_vec()
        .try_into()
        .unwrap();

    for line in input.lines() {
        let (program_num, connects_to) = line.split(" <-> ").collect_tuple().unwrap();
        let program_num = program_num.parse::<usize>().unwrap();

        let connects_to = connects_to
            .split(", ")
            .map(|x| x.parse::<usize>().unwrap())
            .collect_vec();
        for connection in connects_to.iter() {
            programs[*connection].connects_to.insert(program_num);
        }
        programs[program_num].connects_to.extend(connects_to);
    }

    programs
}
