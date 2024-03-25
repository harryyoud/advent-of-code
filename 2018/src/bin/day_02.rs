use aoc_2018::get_input;
use itertools::Itertools;

fn main() {
    let input = get_input(2);
    let box_ids = input.lines().collect_vec();

    dbg!(part_1(&box_ids));
    dbg!(part_2(&box_ids));
}

fn part_1(box_ids: &[&str]) -> usize {
    let counts = box_ids.iter()
        .map(|x| x.chars().counts())
        .collect_vec();

    counts.iter().filter(|x| x.values().any(|a| *a == 3)).count() *
    counts.iter().filter(|x| x.values().any(|a| *a == 2)).count()
}

fn part_2(box_ids: &[&str]) -> String {
    for (a, b) in box_ids.iter().tuple_combinations() {
        let differences = a.chars().zip(b.chars())
            .enumerate()
            .filter(|(_pos, (a, b))| *a != *b)
            .collect_vec();

        if differences.len() != 1 {
            continue;
        }

        return a.chars()
            .enumerate()
            .filter(|(pos, _x)| *pos != differences[0].0)
            .map(|(_pos, x)| x).collect();
    }
    unreachable!("No solution found");
}