use aoc_2023::get_input;
use aoc_lib::grid::Grid;
use aoc_lib::vector::Vector;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};

struct Schematic {
    inner: Grid<Square>,
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
    fn has_symbol_neighbour(&self, point: Vector<2>) -> bool {
        point
            .neighbours_diagonals()
            .filter_map(|point| self.inner.get(point))
            .any(|x| matches!(x, Square::Symbol(_)) || matches!(x, Square::Asterisk))
    }

    fn get_gear_neighbour(&self, point: Vector<2>) -> impl Iterator<Item = Vector<2>> + use<'_> {
        point
            .neighbours_diagonals()
            .filter(|point| matches!(self.inner.get(*point), Some(Square::Asterisk)))
    }
}

fn main() {
    let input = get_input(3);

    let mut schematic = Schematic {
        inner: Grid::parse(&input, |c| match c {
            x @ '0'..='9' => Square::Digit(x.to_digit(10).unwrap() as u8),
            '.' => Square::Dot,
            '*' => Square::Asterisk,
            x => Square::Symbol(x),
        }),
    };

    let mut part_number_total = 0u32;
    let mut gear_ratio_total = 0u32;
    let mut gear_list: HashMap<Vector<2>, Vec<u32>> = HashMap::new();

    let re = Regex::new(r"\d+").unwrap();
    for (y, line) in input.lines().enumerate() {
        for caps in re.captures_iter(line) {
            let mat = caps.get(0).unwrap();
            if (mat.start()..mat.end())
                .any(|x| schematic.has_symbol_neighbour(Vector::new([x as i32, y as i32])))
            {
                part_number_total += mat.as_str().parse::<u32>().unwrap();
            }
            let mut our_gears = HashSet::new();
            for gear_neighbour in (mat.start()..mat.end())
                .flat_map(|x| schematic.get_gear_neighbour(Vector::new([x as i32, y as i32])))
            {
                our_gears.insert(gear_neighbour);
            }
            for gear in our_gears {
                gear_list
                    .entry(gear)
                    .or_default()
                    .push(mat.as_str().parse::<u32>().unwrap());
            }
        }
    }
    for (_, ratios) in gear_list {
        if ratios.len() > 1 {
            gear_ratio_total += ratios.iter().product::<u32>();
        }
    }

    dbg!(part_number_total);
    dbg!(gear_ratio_total);
}
