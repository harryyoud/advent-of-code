use std::collections::HashSet;
use itertools::Itertools;
use aoc_2020::get_input;

fn main() {
    let input = get_input(6);

    // Strategy: for each group, get all the unique characters (while ignoring the newlines between
    //           people), count them, then sum all the groups
    let part_1 = input.split("\n\n")
        .map(|group| group.chars()
            .filter(|x| x.is_alphabetic())
            .unique()
            .count()
        )
        .sum::<usize>();
    dbg!(part_1);


    // Strategy: for each group, create a hashset for each person, intersect them all, then count
    //           how many are in the final intersection, then sum all the groups
    let part_2 = input.split("\n\n")
        .map(|group| group.split("\n")
            .map(|person| person.chars().collect::<HashSet<_>>())
            .reduce(|acc, x| acc.intersection(&x).copied().collect::<HashSet<_>>())
            .unwrap()
            .len()
        )
        .sum::<usize>();
    dbg!(part_2);
}