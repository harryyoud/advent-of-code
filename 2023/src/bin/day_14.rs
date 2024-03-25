use std::{collections::HashMap, fmt::{self, Write}};

use aoc_2023::get_input;

#[derive(Clone, PartialEq)]
enum Tile {
    CubeRock,
    RoundRock,
    Empty,
}

enum Direction {
    North,
    East,
    South,
    West,
}

struct Grid {
    tiles: HashMap<(usize, usize), Tile>,
    x_len: usize,
    y_len: usize,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.y_len {
            for x in 0..self.x_len {
                f.write_char(match self.tiles.get(&(x, y)).unwrap() {
                    Tile::CubeRock => '#',
                    Tile::RoundRock => 'O',
                    Tile::Empty => '.',
                })?
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

fn main() {
    let input = get_input(14);

    let mut grid = Grid {
        tiles: HashMap::new(),
        x_len: input.lines().next().unwrap().chars().count(),
        y_len: input.lines().count(),
    };

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.tiles.insert((x, y), match c {
                '#' => Tile::CubeRock,
                'O' => Tile::RoundRock,
                '.' => Tile::Empty,
                _ => panic!("Unknown character"),
            });
        }
    }

    let mut seen = HashMap::new();
    let mut remaining_iterations = 0;
    for i in 0usize..1_000_000_000 {
        let mut stuff = String::new();

        tilt_grid(&mut grid, Direction::North);
        tilt_grid(&mut grid, Direction::West);
        tilt_grid(&mut grid, Direction::South);
        tilt_grid(&mut grid, Direction::East);

        write!(stuff, "{grid}").unwrap();
        if let Some(prev_i) = seen.get(&stuff) {
            remaining_iterations = (1_000_000_000 - 1 - prev_i) % (i - prev_i);
            break;
        }
        seen.insert(stuff, i);
    };

    for _ in 0..remaining_iterations {
        tilt_grid(&mut grid, Direction::North);
        tilt_grid(&mut grid, Direction::West);
        tilt_grid(&mut grid, Direction::South);
        tilt_grid(&mut grid, Direction::East);
    }

    dbg!(calculate_weight_north(&grid));
}

fn tilt_grid(grid: &mut Grid, direction: Direction) -> bool {
    let mut changed = false;
    for left in 0..get_max_outer_iter(grid, &direction) {
        let mut swaps: Vec<(usize, usize)> = vec![];
        let mut next_empty_space = 0usize;
        for right in 0..get_max_inner_iter(grid, &direction) {
            match grid.tiles.get(&fix_tile_coords(grid, &direction, left, right)).unwrap() {
                Tile::CubeRock => {
                    next_empty_space = right + 1;
                },
                Tile::RoundRock => {
                    if right == next_empty_space {
                        next_empty_space += 1;
                        continue;
                    }
                    swaps.push((right, next_empty_space));
                    next_empty_space += 1;
                },
                Tile::Empty => {},
            }
        }
        if swaps.len() > 0 {
            changed = true;
        }
        for (right1, right2) in swaps {
            let right1t = grid.tiles.get(&fix_tile_coords(grid, &direction, left, right1)).unwrap().clone();
            let right2t = grid.tiles.get(&fix_tile_coords(grid, &direction, left, right2)).unwrap().clone();
            grid.tiles.insert(fix_tile_coords(grid, &direction, left, right1), right2t);
            grid.tiles.insert(fix_tile_coords(grid, &direction, left, right2), right1t);
        }
    }
    changed
}

fn fix_tile_coords(grid: &Grid, direction: &Direction, l_idx: usize, r_idx: usize) -> (usize, usize) {
    match direction {
        Direction::North => (l_idx, r_idx),
        Direction::East => (grid.x_len - r_idx - 1, l_idx),
        Direction::South => (l_idx, grid.y_len - r_idx - 1),
        Direction::West => (r_idx, l_idx),
    }
}

fn get_max_outer_iter(grid: &Grid, direction: &Direction) -> usize {
    match direction {
        Direction::North | Direction::South => grid.x_len,
        Direction::East | Direction::West => grid.y_len,
    }
}
fn get_max_inner_iter(grid: &Grid, direction: &Direction) -> usize {
    match direction {
        Direction::North | Direction::South => grid.y_len,
        Direction::East | Direction::West => grid.x_len,
    }
}

fn calculate_weight_north(grid: &Grid) -> usize {
    let mut total_weight = 0usize;

    for x in 0..grid.x_len {
        for (weight, _tile) in (0..grid.y_len)
            .map(|y| (grid.y_len - y, grid.tiles.get(&(x, y)).unwrap()))
            .filter(|(_weight, tile)| matches!(tile, Tile::RoundRock))
        {
            total_weight += weight;
        }
    }

    total_weight
}