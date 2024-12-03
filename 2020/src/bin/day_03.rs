use std::collections::HashSet;
use aoc_2020::{get_input, Vec2d};

fn main() {
    let input = get_input(3);
    let grid = build_grid(&input);

    dbg!(part_1(&grid));
    dbg!(part_2(&grid));
}

fn part_1(grid: &Grid) -> usize {
    Sled::new(&grid, (3, 1))
        .filter(|x| x.1.is_tree()).count()
}

fn part_2(grid: &Grid) -> usize {
    let slopes = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];

    slopes.into_iter()
        .map(|slope| {
            Sled::new(grid, slope).filter(|x| x.1.is_tree()).count()
        })
        .product()
}

struct Grid {
    trees: HashSet<Vec2d>,
    height: usize,
    width: usize,
}

impl Grid {
    fn get(&self, position: Vec2d) -> Tile {
        match self.trees.contains(&position) {
            false => Tile::Empty,
            true  => Tile::Tree,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Tile {
    Tree,
    Empty,
}

impl Tile {
    fn is_tree(&self) -> bool {
        matches!(self, Self::Tree)
    }
}

struct Sled<'a> {
    grid: &'a Grid,
    position: Vec2d,
    slope: (usize, usize),
}

impl<'a> Sled<'a> {
    fn new(grid: &Grid, slope: (usize, usize)) -> Sled {
        Sled { grid, slope, position: (0isize, 0isize).into() }
    }
}

impl Iterator for Sled<'_> {
    type Item = (Vec2d, Tile);

    fn next(&mut self) -> Option<Self::Item> {
        if self.position.y > self.grid.height as isize {
            return None;
        }

        let result = (self.position, self.grid.get(self.position));

        self.position = (
            (self.position.x.saturating_add_unsigned(self.slope.0)) % self.grid.width as isize,
            self.position.y.saturating_add_unsigned(self.slope.1),
        ).into();

        Some(result)
    }
}

fn build_grid(input: &str) -> Grid {
    let mut trees = HashSet::new();
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {},
                '#' => { trees.insert((x, y).into()); },
                _ => panic!("Invalid character {c} at line {y}, column {x}")
            }
        }
    }

    Grid {
        trees,
        height,
        width,
    }
}
