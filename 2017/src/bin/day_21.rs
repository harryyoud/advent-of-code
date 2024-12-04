use std::collections::HashMap;

use aoc_2017::get_input;
use itertools::Itertools;

fn main() {
    let input = get_input(21);
    let rules = parse_input(&input);
    let starting_grid = vec![
        vec![false, true, false],
        vec![false, false, true],
        vec![true, true, true],
    ];

    let part_1: usize = run(&starting_grid, &rules, 5);
    dbg!(part_1);
    let part_2 = run(&starting_grid, &rules, 18);
    dbg!(part_2);
}

fn run(starting_grid: &Vec<Vec<bool>>, rules: &Rules, iterations: usize) -> usize {
    let mut grid = starting_grid.clone();

    for _ in 0..iterations {
        grid = rejoin_split_square(
            &split_into_n(&grid, if grid.len() % 2 == 0 { 2 } else { 3 })
                .into_iter()
                .map(|x| get_rule_result(rules, &x))
                .collect_vec(),
        );
    }

    grid.into_iter()
        .map(|x| x.into_iter().filter(|a| *a).count())
        .sum::<usize>()
}

fn flip_horizontal<T: Copy>(grid: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    (0..grid.len())
        .map(|x| grid[x].iter().copied().rev().collect_vec())
        .collect_vec()
}

fn flip_vertical<T: Copy>(grid: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    grid.iter().cloned().rev().collect_vec()
}

fn get_rule_result(rules: &Rules, grid: &Grid) -> Grid {
    match rules.get(grid) {
        Some(x) => return x.clone(),
        None => {}
    };
    get_rule_result_recursive(rules, grid, 0, 5).unwrap()
}

fn get_rule_result_recursive(
    rules: &Rules,
    grid: &Grid,
    attempt: usize,
    max_attempts: usize,
) -> Option<Grid> {
    if attempt >= max_attempts {
        return None;
    }

    match rules.get(grid) {
        Some(x) => return Some(x.clone()),
        None => {}
    };

    for new_grid in [
        flip_horizontal(grid),
        flip_vertical(grid),
        rotate_grid(grid),
    ] {
        match get_rule_result_recursive(rules, &new_grid, attempt + 1, max_attempts) {
            Some(x) => return Some(x),
            None => continue,
        }
    }

    None
}

fn parse_input(input: &str) -> HashMap<Grid, Grid> {
    input
        .lines()
        .map(|line| {
            line.split(" => ")
                .map(|grid| {
                    grid.split('/')
                        .map(|row| row.chars().map(|cell| cell == '#').collect_vec())
                        .collect_vec()
                })
                .collect_tuple()
                .unwrap()
        })
        .collect::<HashMap<_, _>>()
}

type Grid = Vec<Vec<bool>>;
type Rules = HashMap<Grid, Grid>;

// 0 1 2         2 5 8
// 3 4 5    =>   1 4 7
// 6 7 8         0 3 6
fn rotate_grid<T: Copy>(grid: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let length = grid.len();
    (1..=length)
        .map(|a| (0..length).map(|b| grid[b][length - a]).collect_vec())
        .collect_vec()
        .try_into()
        .unwrap()
}

// 0 1 2 3            square 0: [0, 1, 4, 5]
// 4 5 6 7    n = 4   square 1: [2, 3, 6, 7]
// 8 9 a b     =>     square 2: [8, 9, c, d]
// c d e f            square 3: [a, b, e, f]
fn split_into_n<T: Copy>(grid: &Vec<Vec<T>>, n: usize) -> Vec<Vec<Vec<T>>> {
    assert!(grid.len() % n == 0);
    let new_len = grid.len() / n;

    let get_square = |row: usize, col: usize| -> Vec<Vec<T>> {
        let from_row = row * n;
        let from_col = col * n;
        let to_row = from_row + n;
        let to_col = from_col + n;

        (from_col..to_col)
            .map(|col_num| {
                (from_row..to_row)
                    .map(|row_num| grid[col_num][row_num])
                    .collect_vec()
            })
            .collect_vec()
    };

    (0..new_len)
        .cartesian_product(0..new_len)
        .map(|(col, row)| get_square(row, col))
        .collect_vec()
}

// square 0: [0, 1, 4, 5]          0 1 2 3
// square 1: [2, 3, 6, 7]          4 5 6 7
// square 2: [8, 9, c, d]    =>    8 9 a b
// square 3: [a, b, e, f]          c d e f
fn rejoin_split_square<T: Copy + std::fmt::Debug>(squares: &Vec<Vec<Vec<T>>>) -> Vec<Vec<T>> {
    if squares.len() == 1 {
        return squares[0].clone();
    }

    let inner_square_len = squares[0][0].len();
    let squares_per_side = (squares.len() as f64).sqrt() as usize;
    let new_len = inner_square_len * squares_per_side;

    let get_row = |row: usize| -> Vec<T> {
        let start = (row / inner_square_len) * squares_per_side;
        let outer_range = start..(start + squares_per_side);
        outer_range
            .flat_map(|x| squares[x][row % inner_square_len].iter().copied())
            .collect_vec()
    };

    (0..new_len).map(get_row).collect_vec()
}

#[test]
fn test_rotate() {
    let start = vec![
        vec![0, 1, 2, 3],
        vec![4, 5, 6, 7],
        vec![8, 9, 10, 11],
        vec![12, 13, 14, 15],
    ];

    let rotate90 = rotate_grid(&start);
    assert_eq!(
        rotate90,
        [[3, 7, 11, 15], [2, 6, 10, 14], [1, 5, 9, 13], [0, 4, 8, 12],]
    );

    let rotate180 = rotate_grid(&rotate90);
    assert_eq!(
        rotate180,
        [[15, 14, 13, 12], [11, 10, 9, 8], [7, 6, 5, 4], [3, 2, 1, 0],]
    );

    let rotate270 = rotate_grid(&rotate180);
    assert_eq!(
        rotate270,
        [[12, 8, 4, 0], [13, 9, 5, 1], [14, 10, 6, 2], [15, 11, 7, 3],]
    );

    let rotate360 = rotate_grid(&rotate270);
    assert_eq!(rotate360, start);
}

#[test]
fn test_flips() {
    let start = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    assert_eq!(flip_vertical(&start), [[7, 8, 9], [4, 5, 6], [1, 2, 3],]);

    assert_eq!(flip_horizontal(&start), [[3, 2, 1], [6, 5, 4], [9, 8, 7],]);
}

#[test]
fn test_split() {
    let start: Vec<Vec<usize>> = vec![
        vec![1, 2, 3, 4, 5, 6],
        vec![7, 8, 9, 10, 11, 12],
        vec![13, 14, 15, 16, 17, 18],
        vec![19, 20, 21, 22, 23, 24],
        vec![25, 26, 27, 28, 29, 30],
        vec![30, 31, 32, 33, 34, 35],
    ];

    let x;

    assert_eq!(
        {
            x = split_into_n(&start, 2);
            &x
        },
        &[
            [[1, 2], [7, 8]],
            [[3, 4], [9, 10]],
            [[5, 6], [11, 12]],
            [[13, 14], [19, 20]],
            [[15, 16], [21, 22]],
            [[17, 18], [23, 24]],
            [[25, 26], [30, 31]],
            [[27, 28], [32, 33]],
            [[29, 30], [34, 35]],
        ]
    );

    assert_eq!(rejoin_split_square(&x), start,);

    let x;

    assert_eq!(
        {
            x = split_into_n(&start, 3);
            &x
        },
        &[
            [[1, 2, 3], [7, 8, 9], [13, 14, 15]],
            [[4, 5, 6], [10, 11, 12], [16, 17, 18]],
            [[19, 20, 21], [25, 26, 27], [30, 31, 32]],
            [[22, 23, 24], [28, 29, 30], [33, 34, 35]]
        ]
    );

    assert_eq!(rejoin_split_square(&x), start,);
}
