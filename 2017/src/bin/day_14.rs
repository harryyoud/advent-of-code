use std::collections::HashSet;

use aoc_2017::{count_groups, get_input, knot_hasher};
use itertools::Itertools;

const WIDTH: usize = 128;
const HEIGHT: usize = 128;

type Grid = [[bool; WIDTH]; HEIGHT];

fn main() {
    let input = get_input(14);
    let grid = build_grid(&input);

    dbg!(part_1(&grid));
    dbg!(part_2(&grid));
}

fn part_1(grid: &Grid) -> usize {
    grid.iter().flatten().filter(|x| **x).count()
}

fn part_2(grid: &Grid) -> usize {
    let f = |x: &(usize, usize)| -> Vec<(usize, usize)> { used_neighbours(grid, *x) };

    count_groups(find_used_cells(grid), f)
}

fn find_used_cells(grid: &Grid) -> HashSet<(usize, usize)> {
    HashSet::from_iter(grid.iter().enumerate().flat_map(|(y, row)| {
        row.iter()
            .enumerate()
            .filter(|(_x, cell)| **cell)
            .map(|(x, _cell)| (x, y))
            .collect_vec()
    }))
}

fn used_neighbours(grid: &Grid, point: (usize, usize)) -> Vec<(usize, usize)> {
    vec![
        (point.0.saturating_sub(1), point.1),          // left
        ((point.0 + 1).clamp(0, WIDTH - 1), point.1),  // right
        (point.0, point.1.saturating_sub(1)),          // up
        (point.0, (point.1 + 1).clamp(0, HEIGHT - 1)), // down
    ]
    .into_iter()
    .unique()
    .filter(|(x, y)| grid[*y][*x])
    .collect_vec()
}

fn build_grid(input: &str) -> Grid {
    (0..HEIGHT)
        .map(|x| {
            knot_hasher::hash(&format!("{input}-{x}"))
                .iter()
                .flat_map(|u| format!("{u:8b}").chars().map(|v| v == '1').collect_vec())
                .collect_vec()
                .try_into()
                .unwrap()
        })
        .collect_vec()
        .try_into()
        .unwrap()
}
