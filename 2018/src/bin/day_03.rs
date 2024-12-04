use std::{collections::HashMap, ops::Range};

use aoc_2018::get_input;
use itertools::Itertools;
use lazy_regex::regex;

fn main() {
    let input = get_input(3);
    let squares = input.lines().map(parse_line).collect_vec();
    let grid = build_grid(&squares);

    dbg!(part_1(&grid));
    dbg!(part_2(&squares, &grid));
}

fn part_1(grid: &HashMap<(usize, usize), Vec<usize>>) -> usize {
    grid.iter()
        .filter(|(_coord, squares)| squares.len() > 1)
        .count()
}

fn part_2(squares: &[Square], grid: &HashMap<(usize, usize), Vec<usize>>) -> usize {
    squares
        .iter()
        .find(|square| square.points().all(|coord| grid[&coord].len() == 1))
        .unwrap()
        .num
}

fn build_grid(squares: &[Square]) -> HashMap<(usize, usize), Vec<usize>> {
    let mut grid = HashMap::new();
    for square in squares {
        for coord in square.points() {
            grid.entry(coord).or_insert(vec![]).push(square.num);
        }
    }
    grid
}

fn parse_line(line: &str) -> Square {
    let re =
        regex!(r#"^#(?<num>\d+) @ (?<x_start>\d+),(?<y_start>\d+): (?<x_len>\d+)x(?<y_len>\d+)$"#);
    let captures = re.captures(line).unwrap();
    let x_start = captures["x_start"].parse().unwrap();
    let y_start = captures["y_start"].parse().unwrap();
    Square {
        num: captures["num"].parse().unwrap(),
        x_range: (x_start)..(x_start + captures["x_len"].parse::<usize>().unwrap()),
        y_range: (y_start)..(y_start + captures["y_len"].parse::<usize>().unwrap()),
    }
}

struct Square {
    num: usize,
    x_range: Range<usize>,
    y_range: Range<usize>,
}

impl Square {
    fn points(&self) -> impl Iterator<Item = (usize, usize)> {
        self.x_range.clone().cartesian_product(self.y_range.clone())
    }
}
