use std::collections::HashMap;

use aoc_2017::get_input;
use itertools::Itertools;

const BANK_COUNT: usize = 16;

fn main() {
    let input = get_input(6);
    let allocator = parse_input(&input);

    let (part_1, part_2) = solve(&allocator);
    dbg!(part_1, part_2);
}

fn solve(allocator: &Allocator) -> (usize, usize) {
    let mut allocator = allocator.clone();
    let mut seen = HashMap::<Allocator, usize>::new();

    for i in 0.. {
        if let Some(x) = seen.get(&allocator) {
            return (i, i - x);
        }
        seen.insert(allocator.clone(), i);
        let max = allocator.get_max();
        allocator.reallocate(max);
    }

    unreachable!("No solutions found");
}

fn parse_input(input: &str) -> Allocator {
    let banks: [Bank; BANK_COUNT] = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .map(|x| Bank { blocks: x })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    Allocator { banks }
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Allocator {
    banks: [Bank; BANK_COUNT],
}

impl Allocator {
    fn get_max(&self) -> usize {
        self.banks
            .iter()
            .enumerate()
            .max_set_by_key(|(_num, bank)| bank.blocks)
            .into_iter()
            .min_by_key(|(num, _bank)| *num)
            .unwrap()
            .0
    }

    fn reallocate(&mut self, bank: usize) {
        let mut blocks_to_reallocate = self.banks[bank].blocks;
        self.banks[bank].blocks = 0;

        let mut bank_num = (bank + 1) % self.banks.len();
        while blocks_to_reallocate > 0 {
            self.banks[bank_num].blocks += 1;
            blocks_to_reallocate -= 1;
            bank_num = (bank_num + 1) % self.banks.len();
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Bank {
    blocks: usize,
}