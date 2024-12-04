use std::{collections::HashSet, iter};

use aoc_2023::get_input;
use itertools::Itertools;

#[derive(Clone)]
enum Square {
    Empty,
    Galaxy,
}

fn main() {
    let input = get_input(11);
    let (grid, vertical_empties, horizontal_empties) = parse_and_calculate_grid(&input);

    let squares: Vec<((usize, usize), Square)> = grid
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .map(|(col_idx, square)| ((col_idx, row_idx), square.clone()))
                .collect_vec()
        })
        .collect_vec();
    let mut part_a = 0_usize;
    let mut part_b = 0_usize;
    for (galaxy_a, galaxy_b) in squares
        .iter()
        .filter(|((_x, _y), square)| matches!(square, Square::Galaxy))
        .tuple_combinations()
    {
        part_a += (galaxy_a.0 .0 as isize - galaxy_b.0 .0 as isize).unsigned_abs()
            + (galaxy_a.0 .1 as isize - galaxy_b.0 .1 as isize).unsigned_abs();
        part_a +=
            HashSet::from_iter(galaxy_a.0 .0.min(galaxy_b.0 .0)..=galaxy_b.0 .0.max(galaxy_a.0 .0))
                .intersection(&vertical_empties)
                .count();
        part_a +=
            HashSet::from_iter(galaxy_a.0 .1.min(galaxy_b.0 .1)..=galaxy_b.0 .1.max(galaxy_a.0 .1))
                .intersection(&horizontal_empties)
                .count();

        part_b += (galaxy_a.0 .0 as isize - galaxy_b.0 .0 as isize).unsigned_abs()
            + (galaxy_a.0 .1 as isize - galaxy_b.0 .1 as isize).unsigned_abs();
        part_b +=
            HashSet::from_iter(galaxy_a.0 .0.min(galaxy_b.0 .0)..=galaxy_b.0 .0.max(galaxy_a.0 .0))
                .intersection(&vertical_empties)
                .count()
                * (1_000_000 - 1);
        part_b +=
            HashSet::from_iter(galaxy_a.0 .1.min(galaxy_b.0 .1)..=galaxy_b.0 .1.max(galaxy_a.0 .1))
                .intersection(&horizontal_empties)
                .count()
                * (1_000_000 - 1);
    }
    dbg!(part_a, part_b,);

    for (row_idx, row) in grid.iter().enumerate() {
        if horizontal_empties.contains(&row_idx) {
            println!("{}", "/".repeat(grid[0].len() + vertical_empties.len()));
        }
        for (col_idx, square) in row.iter().enumerate() {
            if vertical_empties.contains(&col_idx) {
                print!("/");
            }
            print!(
                "{}",
                if matches!(square, Square::Empty) {
                    "."
                } else {
                    "#"
                }
            )
        }
        println!();
    }
}

fn parse_and_calculate_grid(input: &str) -> (Vec<Vec<Square>>, HashSet<usize>, HashSet<usize>) {
    let mut horizontal_empties = HashSet::new();
    let squares: Vec<Vec<Square>> = input
        .lines()
        .enumerate()
        .flat_map(|(line_idx, line)| {
            let row = vec![line
                .chars()
                .map(|cha| match cha {
                    '.' => Square::Empty,
                    '#' => Square::Galaxy,
                    _ => panic!("Invalid character: {}", cha),
                })
                .collect_vec()];
            if row[0].iter().all(|square| matches!(square, Square::Empty)) {
                horizontal_empties.insert(line_idx);
            }
            row
        })
        .collect_vec();

    let mut empty_columns: Vec<bool> = iter::repeat(true).take(squares[0].len()).collect_vec();
    for row in squares.iter() {
        empty_columns
            .iter_mut()
            .zip(row.iter())
            .map(|(empty, square)| *empty = *empty && matches!(square, Square::Empty))
            .count();
    }

    let vertical_empties = empty_columns
        .iter()
        .enumerate()
        .filter(|(_idx, is_empty)| **is_empty)
        .map(|(idx, _is_empty)| idx)
        .collect();

    (squares, vertical_empties, horizontal_empties)
}
