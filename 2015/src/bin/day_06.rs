use aoc_2015::get_input;
use itertools::Itertools;

type Grid = [[bool; 1000]; 1000];

fn main() {
    let input = get_input(6);

    dbg!(part_1(&input));
    dbg!(part_2(&input));
}

fn part_2(input: &str) -> u64 {
    let mut grid = [[0; 1000]; 1000];

    for line in input.lines() {
        let (start_x, start_y, end_x, end_y) = extract_numbers(line);

        match &line[0..7] {
            "toggle " => set_brightness(&mut grid, start_x, start_y, end_x, end_y, 2),
            "turn of" => set_brightness(&mut grid, start_x, start_y, end_x, end_y, -1),
            "turn on" => set_brightness(&mut grid, start_x, start_y, end_x, end_y, 1),
            _ => panic!("Unknown command"),
        }
    }

    grid.into_iter().map(|x| x.into_iter().map(|x| x as u64).sum::<u64>()).sum::<u64>()
}

fn set_brightness(grid: &mut [[u8; 1000]; 1000], start_x: usize, start_y: usize, end_x: usize, end_y: usize, delta: i8) {
    for x in start_x..=end_x {
        for y in start_y..=end_y {
            grid[x][y] = grid[x][y].saturating_add_signed(delta);
        }
    }

}

fn part_1(input: &str) -> u64 {
    let mut grid = [[false; 1000]; 1000];

    for line in input.lines() {
        let (start_x, start_y, end_x, end_y) = extract_numbers(line);

        match &line[0..7] {
            "toggle " => toggle_lights(&mut grid, start_x, start_y, end_x, end_y),
            "turn of" => set_lights(&mut grid, start_x, start_y, end_x, end_y, false),
            "turn on" => set_lights(&mut grid, start_x, start_y, end_x, end_y, true),
            _ => panic!("Unknown command"),
        }
    }

    grid.iter().flat_map(|x| x.iter()).filter(|x| **x).count() as u64
}

fn toggle_lights(grid: &mut Grid, start_x: usize, start_y: usize, end_x: usize, end_y: usize) {
    for x in start_x..=end_x {
        for y in start_y..=end_y {
            grid[x][y] = !grid[x][y];
        }
    }
}

fn set_lights(grid: &mut Grid, start_x: usize, start_y: usize, end_x: usize, end_y: usize, set_to: bool) {
    for x in start_x..=end_x {
        for y in start_y..=end_y {
            grid[x][y] = set_to;
        }
    }
}

fn extract_numbers(line: &str) -> (usize, usize, usize, usize) {
    line
        .trim_start_matches(|x: char| x.is_alphabetic() || x.is_whitespace())
        .split(" through ")
        .map(|x| x.split(",").map(|a| a.parse::<usize>().unwrap()).collect_vec())
        .flatten()
        .collect_tuple()
        .unwrap()
}