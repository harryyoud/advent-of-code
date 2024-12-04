use aoc_2024::get_input;
use itertools::Itertools;

fn main() {
    let input = get_input(1);

    let (mut left_list, mut right_list): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(left_id, right_id)| {
            (
                left_id.parse::<usize>().unwrap(),
                right_id.parse::<usize>().unwrap(),
            )
        })
        .unzip();

    left_list.sort();
    right_list.sort();

    dbg!(part_1(&left_list, &right_list));
    dbg!(part_2(&left_list, &right_list));
}

// Assumes lists are sorted
fn part_1(left_list: &[usize], right_list: &[usize]) -> usize {
    left_list
        .into_iter()
        .zip(right_list)
        .map(|(left, right)| left.abs_diff(*right))
        .sum::<usize>()
}

fn part_2(left_list: &[usize], right_list: &[usize]) -> usize {
    let counts = right_list.into_iter().cloned().counts();
    left_list
        .into_iter()
        .map(|x| x * counts.get(x).unwrap_or(&0))
        .sum::<usize>()
}
