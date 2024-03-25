use std::collections::HashSet;

use aoc_2023::get_input;
use itertools::Itertools;

trait Position {
    fn up(&self) -> Option<Self> where Self: Sized;
    fn down(&self) -> Option<Self> where Self: Sized;
    fn left(&self) -> Option<Self> where Self: Sized;
    fn right(&self) -> Option<Self> where Self: Sized;
}

impl Position for (u32, u32) {
    fn up(&self) -> Option<Self> {
        Some((self.0, self.1.checked_sub(1)?))
    }

    fn down(&self) -> Option<Self> {
        Some((self.0, self.1 + 1))
    }

    fn left(&self) -> Option<Self> {
        Some((self.0.checked_sub(1)?, self.1))
    }

    fn right(&self) -> Option<Self> {
        Some((self.0 + 1, self.1))
    }
}

struct Grid {
    rocks: HashSet<(u32, u32)>,
    starting_position: (u32, u32),
    x_len: u32,
    y_len: u32,
}

impl Grid {
    fn find_viable_neighbours(&self, point: (u32, u32)) -> Vec<(u32, u32)> {
        let neighbours = vec![
            point.up(),
            point.down(),
            point.right(),
            point.left(),
        ];
        neighbours
            .into_iter()
            .filter_map(|x| x)
            .filter(|x| self.is_in_bounds(*x))
            .filter(|x| !self.rocks.contains(&x))
            .collect_vec()
    }

    fn is_in_bounds(&self, point: (u32, u32)) -> bool {
        point.0 < self.x_len && point.1 < self.y_len
    }
}

fn main() {
    let input = get_input(21);
    let grid = parse_input(&input);
    dbg!(part_a(&grid, 64));
}

fn part_a(grid: &Grid, steps: u32) -> u32 {
    let mut to_visit = HashSet::<(u32, u32)>::new();
    to_visit.insert(grid.starting_position);

    for _tick in 0..steps {
        for point in std::mem::take(&mut to_visit).into_iter() {
            for neighbour in grid.find_viable_neighbours(point) {
                to_visit.insert(neighbour);
            }
        }
    }

    to_visit.len() as u32
}

fn parse_input(input: &str) -> Grid {
    let x_len = input.lines().next().unwrap().len() as u32;
    let y_len = input.lines().count() as u32;

    dbg!(x_len, y_len);

    let mut starting_position = None;
    let mut rocks: HashSet<(u32, u32)> = HashSet::new();
    
    for (y, line) in input.lines().enumerate() {
        for (x, cha) in line.chars().enumerate() {
            match cha {
                '#' => {rocks.insert((x as u32, y as u32));},
                'S' => {starting_position = Some((x as u32, y as u32))},
                '.' => continue,
                _ => panic!("Invalid character \"{cha}\" line {x}, character {y}"),
            } 
        }
    }

    Grid {
        starting_position: starting_position.expect("Could not find starting position S"),
        rocks, x_len, y_len
    }
}