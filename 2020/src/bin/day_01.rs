use itertools::Itertools;
use std::collections::HashSet;
use aoc_2020::get_input;

fn main() {
    let input = get_input(1);
    let numbers = input.lines()
        .map(|x| x.parse().unwrap())
        .collect::<HashSet<u32>>();

    dbg!(part_1(&numbers));
    dbg!(part_2(&numbers));
}

// We could shortcut and iter for n and if (2020 - n) is present in map, return these, but we use a
// combination method for part_2, so keep it simple
fn part_1(numbers: &HashSet<u32>) -> Option<u32> {
    find_group(numbers, 2, 2020)
        .map(|x| x.iter().product())
}

fn part_2(numbers: &HashSet<u32>) -> Option<u32> {
    find_group(numbers, 3, 2020)
        .map(|x| x.iter().product())
}

fn find_group(numbers: &HashSet<u32>, amount: usize, target: u32) -> Option<Vec<u32>> {
    for set in numbers.into_iter().copied().combinations(amount) {
        if set.iter().copied().sum::<u32>() == target {
            return Some(set);
        }
    }

    None
}
