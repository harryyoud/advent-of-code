use std::collections::HashMap;

use aoc_2018::get_input;
use itertools::Itertools;
use rayon::prelude::*;

const GRID_WIDTH: usize = 300;

fn main() {
    let input = get_input(11).trim().parse::<usize>().unwrap();
    let grid = Grid::new(input);

    dbg!(part_1(&grid));
    dbg!(part_2(&grid));
}

fn part_1(grid: &Grid) -> (usize, usize) {
    find_square_size_with_highest_power(grid, 3).0
}

fn part_2(grid: &Grid) -> (usize, usize, usize) {
    find_square_with_highest_power(grid)
}

fn find_square_with_highest_power(grid: &Grid) -> (usize, usize, usize) {
    (1usize..=GRID_WIDTH)
        .into_par_iter()
        .map(|size| {
            dbg!(size);
            (find_square_size_with_highest_power(grid, size), size)
        })
        .max_by_key(|((_pos, power), _size)| *power)
        .map(|((pos, _power), size)| (pos.0, pos.1, size))
        .unwrap()
}

fn find_square_size_with_highest_power(
    grid: &Grid,
    square_width: usize,
) -> ((usize, usize), isize) {
    let upper_limit = GRID_WIDTH - (square_width - 1);

    (1..=upper_limit)
        .cartesian_product(1..=upper_limit)
        .par_bridge()
        .into_par_iter()
        .map(|(start_x, start_y)| {
            (
                (start_x, start_y),
                grid.square_power(square_width, start_x, start_y),
            )
        })
        .max_by_key(|(_pos, power)| *power)
        .unwrap()
}

struct Grid {
    power_levels: HashMap<(usize, usize), isize>,
}

impl Grid {
    fn new(serial_number: usize) -> Self {
        let mut power_levels = HashMap::new();
        for (x, y) in (1..=GRID_WIDTH).cartesian_product(1..=GRID_WIDTH) {
            power_levels.insert((x, y), Grid::calculate_power_level(serial_number, x, y));
        }
        Grid { power_levels }
    }

    fn calculate_power_level(serial_number: usize, x: usize, y: usize) -> isize {
        let rack_id = x + 10;
        let mut power_level = rack_id * y;
        power_level += serial_number;
        power_level *= rack_id;
        power_level /= 100;
        power_level %= 10;
        power_level as isize - 5
    }

    fn get_power_level(&self, x: usize, y: usize) -> isize {
        *self.power_levels.get(&(x, y)).unwrap()
    }

    fn square_power(&self, size: usize, top_left_x: usize, top_left_y: usize) -> isize {
        let mut power = 0;
        for x in top_left_x..top_left_x + size {
            for y in top_left_y..top_left_y + size {
                power += self.get_power_level(x, y);
            }
        }
        power
    }
}
