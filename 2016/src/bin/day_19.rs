use aoc_2016::get_input;
use itertools::Itertools;

fn main() {
    let input = get_input(19);
    let number_of_elves = input.parse::<usize>().unwrap();
    let elves = (0..number_of_elves)
        .map(|idx| Elf {
            next_elf: (idx + 1) % number_of_elves,
        })
        .collect_vec();

    dbg!(part_1(&elves));
    dbg!(part_2(&elves));
}

fn part_1(elves: &[Elf]) -> usize {
    solve(Strategy::FromLeft, elves)
}

fn part_2(elves: &[Elf]) -> usize {
    solve(Strategy::FromOpposite, elves)
}

fn solve(strategy: Strategy, elves: &[Elf]) -> usize {
    let mut elves = elves.to_owned();

    let mut current_elf = 0usize;
    let mut victim_parent = if strategy.is_from_opposite() {
        (elves.len() / 2) - 1
    } else {
        0usize
    };
    let mut remaining_elves = elves.len();

    loop {
        if remaining_elves == 1 {
            break;
        }

        let victim = elves[victim_parent].next_elf;
        elves[victim_parent].next_elf = elves[victim].next_elf;

        if strategy.is_from_left() || remaining_elves % 2 == 1 {
            victim_parent = elves[victim_parent].next_elf
        }
        current_elf = elves[current_elf].next_elf;
        remaining_elves -= 1;
    }

    current_elf + 1
}

enum Strategy {
    FromOpposite,
    FromLeft,
}

impl Strategy {
    fn is_from_opposite(&self) -> bool {
        matches!(self, Strategy::FromOpposite)
    }
    fn is_from_left(&self) -> bool {
        matches!(self, Strategy::FromLeft)
    }
}

#[derive(Clone)]
struct Elf {
    next_elf: usize,
}
