use std::fmt::{self};
use std::collections::{HashMap, HashSet};

use aoc_2023::get_input;
use itertools::Itertools;

#[derive(Clone)]
enum Tile {
    Empty,
    BottomLeftToTopRightMirror,
    BottomRightToTopLeftMirror,
    HorizontalSplitter,
    VerticalSplitter,
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
struct Grid {
    tiles: HashMap<(isize, isize), Tile>,
    energised: HashSet<((isize, isize), Direction)>,
    x_len: isize,
    y_len: isize,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.y_len {
            for x in 0..self.x_len {
                if self.is_energised(x, y) {
                    write!(f, "#")?;
                    continue;
                }
                write!(f, "{}", match self.tiles.get(&(x, y)).unwrap() {
                    Tile::Empty => '.',
                    Tile::BottomLeftToTopRightMirror => '/',
                    Tile::BottomRightToTopLeftMirror => '\\',
                    Tile::HorizontalSplitter => '-',
                    Tile::VerticalSplitter => '|',
                })?
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    fn is_energised(&self, x: isize, y: isize) -> bool {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right
        ].into_iter().map(|dir|
            self.energised.contains(&((x, y), dir))
        ).any(|x| x)
    }
}

trait Move {
    fn move_dir(&self, direction: &Direction) -> Self;
}

impl Move for (isize, isize) {
    fn move_dir(&self, direction: &Direction) -> Self {
        match direction {
            Direction::Down => (self.0, self.1 + 1),
            Direction::Up => (self.0, self.1 - 1),
            Direction::Left => (self.0 - 1, self.1),
            Direction::Right => (self.0 + 1, self.1),
        }
    }
}

fn main() {
    let input = get_input(16);
    let mut grid = Grid {
        tiles: HashMap::new(),
        energised: HashSet::new(),
        x_len: input.lines().next().unwrap().len() as isize,
        y_len: input.lines().count() as isize,
    };
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.tiles.insert((x as isize, y as isize), match c {
                '.' => Tile::Empty,
                '/' => Tile::BottomLeftToTopRightMirror,
                '\\' => Tile::BottomRightToTopLeftMirror,
                '-' => Tile::HorizontalSplitter,
                '|' => Tile::VerticalSplitter,
                _ => panic!("Invalid character at ({x}, {y})"),
            });
        }
    }

    let part_a = calculate_energised(grid.clone(), (0, 0), Direction::Right);
    dbg!(part_a);

    let mut max_energised = 0usize;

    for y in 0..grid.y_len {
        let energised = calculate_energised(grid.clone(), (0, y), Direction::Right);
        max_energised = max_energised.max(energised);
        let energised = calculate_energised(grid.clone(), (grid.x_len - 1, y), Direction::Left);
        max_energised = max_energised.max(energised);
    }
    for x in 0..grid.x_len {
        let energised = calculate_energised(grid.clone(), (x, 0), Direction::Down);
        max_energised = max_energised.max(energised);
        let energised = calculate_energised(grid.clone(), (x, grid.y_len - 1), Direction::Up);
        max_energised = max_energised.max(energised);
    }

    let part_b = max_energised;
    dbg!(part_b);
}

fn calculate_energised(mut grid: Grid, start_point: (isize, isize), start_direction: Direction) -> usize {
    let mut next_visit: HashSet<((isize, isize), Direction)> = HashSet::new();
    next_visit.insert((start_point, start_direction));

    loop {
        let this_visit = std::mem::take(&mut next_visit)
            .into_iter()
            .filter(|((x, y), _dir)| {
                x >= &0 && x < &grid.x_len && y >= &0 && y < &grid.y_len
            })
            .collect::<Vec<_>>()
        ;

        for ((x, y), direction) in this_visit.clone().into_iter() {
            if ! grid.energised.insert(((x, y), direction.clone())) {
                continue;
            };
            match grid.tiles.get(&(x, y)).unwrap() {
                Tile::Empty => {
                    next_visit.insert(((x, y).move_dir(&direction), direction));
                },
                Tile::BottomLeftToTopRightMirror => {
                    match direction {
                        Direction::Up => next_visit.insert(((x, y).move_dir(&Direction::Right), Direction::Right)),
                        Direction::Down => next_visit.insert(((x, y).move_dir(&Direction::Left), Direction::Left)),
                        Direction::Left => next_visit.insert(((x, y).move_dir(&Direction::Down), Direction::Down)),
                        Direction::Right => next_visit.insert(((x, y).move_dir(&Direction::Up), Direction::Up)),
                    };
                },
                Tile::BottomRightToTopLeftMirror => {
                    match direction {
                        Direction::Up => next_visit.insert(((x, y).move_dir(&Direction::Left), Direction::Left)),
                        Direction::Down => next_visit.insert(((x, y).move_dir(&Direction::Right), Direction::Right)),
                        Direction::Left => next_visit.insert(((x, y).move_dir(&Direction::Up), Direction::Up)),
                        Direction::Right => next_visit.insert(((x, y).move_dir(&Direction::Down), Direction::Down)),
                    };
                },
                Tile::HorizontalSplitter => {
                    match direction {
                        Direction::Up | Direction::Down => {
                            next_visit.insert(((x, y).move_dir(&Direction::Left), Direction::Left));
                            next_visit.insert(((x, y).move_dir(&Direction::Right), Direction::Right));
                        },
                        _ => { next_visit.insert(((x, y).move_dir(&direction), direction)); }
                    };
                },
                Tile::VerticalSplitter => {
                    match direction {
                        Direction::Left | Direction::Right => {
                            next_visit.insert(((x, y).move_dir(&Direction::Up), Direction::Up));
                            next_visit.insert(((x, y).move_dir(&Direction::Down), Direction::Down));
                        },
                        _ => { next_visit.insert(((x, y).move_dir(&direction), direction)); }
                    };
                },
            }
        }

        if next_visit.is_empty() {
            break;
        }
    }

    grid.energised.iter().map(|((x, y), _d)| (x, y)).unique().count()
}