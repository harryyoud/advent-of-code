use std::collections::{HashMap, HashSet};

use aoc_2023::get_input;
use regex::Regex;

struct Schematic {
    inner: Vec<Vec<Square>>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum Square {
    Digit(u8),
    Symbol(char),
    Dot,
    Asterisk,
}

impl Schematic {

    // returns true if any of neighbours (direct or diagonal are a symbol (i.e. not a digit or period))
    fn has_symbol_neighbour(&self, y: usize, x: usize) -> bool {
        self.get_neighbours(y, x).iter().map(|(y, x)| &self.inner[*y][*x]).any(|x| {
            matches!(x, Square::Symbol(_)) || matches!(x, Square::Asterisk)
        })
    }

    fn get_gear_neighbour(&self, y: usize, x: usize) -> Vec<(usize, usize)> {
        self.get_neighbours(y, x).iter().filter_map(|(y, x)| {
            if matches!(self.inner[*y][*x], Square::Asterisk) {
                return Some((*y, *x));
            }
            None
        }).collect()
    }

    fn get_neighbours(&self, y: usize, x: usize) -> Vec<(usize, usize)> {
        let mut coords_to_check: Vec<(isize, isize)> = vec![
            (0, -1),  // left
            (1, -1),  // down left
            (1, 0),   // down
            (1, 1),   // down right
            (0, 1),   // right
            (-1, 1),  // up right
            (-1, 0),  // up
            (-1, -1), // up left
        ];
        if x == 0 {
            coords_to_check.retain(|(_, x)| *x != -1)
        }
        if x == self.inner[0].len() - 1 {
            coords_to_check.retain(|(_, x)| *x != 1)
        }
        if y == 0 {
            coords_to_check.retain(|(y, _)| *y != -1)
        }
        if y == self.inner.len() - 1 {
            coords_to_check.retain(|(y, _)| *y != 1)
        }

        let mut neighbours = vec![];
        for (y_diff, x_diff) in coords_to_check {
            neighbours.push((
                (y as isize + y_diff) as usize,
                (x as isize + x_diff) as usize)
            );
        }

        neighbours
    }
}

fn main() {
    let input = get_input(3);

    let mut schematic: Vec<Vec<Square>> = vec![];
    for line in input.lines() {
        schematic.push(line.chars().map(|cha| {
            match cha {
                x @ '0'..='9' => Square::Digit(x.to_digit(10).unwrap() as u8),
                '.' => Square::Dot,
                '*' => Square::Asterisk,
                x => Square::Symbol(x),
            }
        }).collect());
    }
    let schematic = Schematic { inner: schematic };

    let mut part_number_total = 0u32;
    let mut gear_ratio_total = 0u32;
    let mut gear_list: HashMap<(usize, usize), Vec<u32>> = HashMap::new();


    let re = Regex::new(r"\d+").unwrap();
    for (y, line) in input.lines().enumerate() {
        for caps in re.captures_iter(line) {
            let mat = caps.get(0).unwrap();
            if (mat.start()..mat.end()).any(|x| schematic.has_symbol_neighbour(y, x)) {
                part_number_total += mat.as_str().parse::<u32>().unwrap();
            }
            let mut our_gears = HashSet::new();
            for gear_neighbour in (mat.start()..mat.end()).flat_map(|x| schematic.get_gear_neighbour(y, x)) {
                our_gears.insert(gear_neighbour);
            }
            for gear in our_gears {
                gear_list.entry(gear).or_default().push(mat.as_str().parse::<u32>().unwrap());
            }
        }
    }
    for (_, ratios) in gear_list {
        if ratios.len() > 1 {
            gear_ratio_total += ratios.iter().product::<u32>();
        }
    }

    println!("{part_number_total}");
    println!("{gear_ratio_total}");
}