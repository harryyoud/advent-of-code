use std::{collections::HashSet, sync::Mutex};

use aoc_2017::get_input;
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() {
    let input = get_input(24);
    let components = parse_input(&input);

    let strongest_bridge = Mutex::new(0);
    let longest_bridge = Mutex::new((0, 0));
    build_bridge_recursive(&[], &strongest_bridge, &longest_bridge, &components);

    let part_1 = strongest_bridge.into_inner().unwrap();
    let part_2 = longest_bridge.into_inner().unwrap().0;

    dbg!(part_1, part_2);
}

fn build_bridge_recursive(
    path: &[(usize, usize)],
    strongest_bridge: &Mutex<usize>,
    longest_bridge: &Mutex<(usize, usize)>,
    remaining_components: &HashSet<(usize, usize)>,
) {
    let exposed_port = path.last().unwrap_or(&(0, 0)).1;
    let lefts = remaining_components
        .iter()
        .filter(|(a, _)| *a == exposed_port)
        .copied()
        .collect_vec();
    let rights = remaining_components
        .iter()
        .filter(|(_, b)| *b == exposed_port)
        .copied()
        .collect_vec();

    if lefts.is_empty() && rights.is_empty() {
        let current_strength = calculate_strength(path);
        let mut inner = strongest_bridge.lock().unwrap();
        *inner = current_strength.max(*inner);

        let mut inner = longest_bridge.lock().unwrap();
        if path.len() > inner.1 || (path.len() == inner.1 && current_strength > inner.0) {
            *inner = (current_strength, path.len());
        }

        return;
    }

    lefts.into_par_iter().for_each(|left| {
        let new_path = path.iter().chain(&[left]).copied().collect_vec();
        let mut remaining = remaining_components.clone();
        remaining.remove(&left);
        build_bridge_recursive(&new_path, strongest_bridge, longest_bridge, &remaining);
    });

    rights.into_par_iter().for_each(|right| {
        let new_path = path.iter().chain(&[flip(right)]).copied().collect_vec();
        let mut remaining = remaining_components.clone();
        remaining.remove(&right);
        build_bridge_recursive(&new_path, strongest_bridge, longest_bridge, &remaining);
    });
}

fn calculate_strength(path: &[(usize, usize)]) -> usize {
    path.iter().map(|(a, b)| *a + *b).sum::<usize>()
}

fn flip(x: (usize, usize)) -> (usize, usize) {
    (x.1, x.0)
}

fn parse_input(input: &str) -> HashSet<(usize, usize)> {
    input
        .lines()
        .map(|s| {
            let (a, b) = s.split_once('/').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect()
}
