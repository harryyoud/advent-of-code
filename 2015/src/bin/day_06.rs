#![feature(coroutines, iter_from_coroutine)]

use aoc_2015::get_input;
use itertools::Itertools;

const GRID_SIZE: usize = 1000;

fn main() {
    let input = get_input(6);

    dbg!(part_1(&input));
    dbg!(part_2(&input));
}

fn part_1(input: &str) -> u64 {
    let mut grid = Grid::new();

    for line in input.lines() {
        let rectangle = parse_line_to_rectangle(line);

        match &line[0..7] {
            "toggle " => grid.toggle(rectangle),
            "turn of" => grid.set(rectangle, false.into()),
            "turn on" => grid.set(rectangle, true.into()),
            _ => panic!("Unknown command"),
        }
    }

    grid.iterate_all().filter(|cell| *cell == 1).count() as u64
}

fn part_2(input: &str) -> u64 {
    let mut grid = Grid::new();

    for line in input.lines() {
        let rectangle = parse_line_to_rectangle(line);

        match &line[0..7] {
            "toggle " => grid.add(rectangle, 2),
            "turn of" => grid.add(rectangle, -1),
            "turn on" => grid.add(rectangle, 1),
            _ => panic!("Unknown command"),
        }
    }

    grid.iterate_all().map(|cell| cell as u64).sum::<u64>()
}


fn parse_line_to_rectangle(line: &str) -> Rectangle {
    let (start_x, start_y, end_x, end_y) = line
        .trim_start_matches(|x: char| x.is_alphabetic() || x.is_whitespace())
        .split(" through ")
        .flat_map(|x| x.split(',').map(|a| a.parse::<usize>().unwrap()).collect_vec())
        .collect_tuple()
        .unwrap();
    Rectangle { start_x, end_x, start_y, end_y }
}

#[derive(Copy, Clone)]
struct Rectangle {
    start_x: usize,
    end_x: usize,
    start_y: usize,
    end_y: usize,
}

struct Grid {
    grid: [[u8; GRID_SIZE]; GRID_SIZE]
}

impl Grid {
    fn new() -> Self {
        Grid {
            grid: [[0; GRID_SIZE]; GRID_SIZE]
        }
    }

    fn iterate_all(&self) -> impl Iterator<Item = u8> + '_ {
        self.iterate(Rectangle {
            start_x: 0,
            end_x: GRID_SIZE - 1,
            start_y: 0,
            end_y: GRID_SIZE - 1,
        })
    }

    fn iterate(&self, rectangle: Rectangle) -> impl Iterator<Item = u8> + '_ {
        std::iter::from_coroutine(move || {
            for y in rectangle.start_y..=rectangle.end_y {
                for x in rectangle.start_x..=rectangle.end_x {
                    yield self.grid[y][x]
                }
            }
        })
    }

    fn iterate_mut(&mut self, rectangle: Rectangle) -> impl Iterator<Item = &mut u8> {
        std::iter::from_coroutine(move || {
            for row in self.grid.iter_mut().take(rectangle.end_y + 1).skip(rectangle.start_y) {
                for cell in row.iter_mut().take(rectangle.end_x + 1).skip(rectangle.start_x) {
                    yield cell;
                }
            }
        })
    }


    fn add(&mut self, rectangle: Rectangle, delta: i8) {
        self
            .iterate_mut(rectangle)
            .for_each(|cell| *cell = cell.saturating_add_signed(delta));
    }

    fn toggle(&mut self, rectangle: Rectangle) {
        self
            .iterate_mut(rectangle)
            .for_each(|cell| {
                *cell = match cell {
                    0 => 1,
                    _ => 0,
                }
            });
    }

    fn set(&mut self, rectangle: Rectangle, value: u8) {
        self
            .iterate_mut(rectangle)
            .for_each(|cell| *cell = value);
    }
}

