use aoc_2015::get_input;
use itertools::Itertools;

fn main() {
    let input = get_input(24);
    let packages = {
        let mut p = input.lines().map(|x| x.parse::<u128>().unwrap()).collect_vec();
        p.sort();
        p.into_iter().rev().collect_vec()
    };

    dbg!(part_1(&packages));
    dbg!(part_2(&packages));
}

fn part_1(packages: &[u128]) -> u128 {
    let target = packages.iter().sum::<u128>() / 3;
    let mut minimum_quantum_entanglement = u128::MAX;
    find_smallest_fit_recursive(packages, 0, 0, 1, target, &mut minimum_quantum_entanglement);
    minimum_quantum_entanglement
}

fn part_2(packages: &[u128]) -> u128 {
    let target = packages.iter().sum::<u128>() / 4;
    let mut minimum_quantum_entanglement = u128::MAX;
    find_smallest_fit_recursive(packages, 0, 0, 1, target, &mut minimum_quantum_entanglement);
    minimum_quantum_entanglement
}

fn find_smallest_fit_recursive(
    packages: &[u128],
    idx: usize,
    sum: u128,
    quantum_entanglement: u128,
    target: u128,
    minimum_quantum_entanglement: &mut u128
) {
    if sum == target {
        *minimum_quantum_entanglement = (*minimum_quantum_entanglement).min(quantum_entanglement);
        return;
    }
    if sum < target && quantum_entanglement < *minimum_quantum_entanglement && idx < packages.len() {
        find_smallest_fit_recursive(packages, idx + 1, sum, quantum_entanglement, target, minimum_quantum_entanglement);
        find_smallest_fit_recursive(packages, idx + 1, sum + packages[idx], quantum_entanglement * packages[idx], target, minimum_quantum_entanglement);
    }
}