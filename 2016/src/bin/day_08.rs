use core::fmt;
use std::{collections::VecDeque, iter};

use aoc_2016::get_input;
use itertools::Itertools;

fn main() {
    let input = get_input(8);
    let instructions = parse_input(&input);
    let mut grid = Grid::new(50, 6);
    for instruction in instructions {
        grid.apply(instruction);
    }

    let part_1 = grid.count_on();
    dbg!(part_1);
    println!("{grid}");
}

struct Grid {
    inner: VecDeque<VecDeque<bool>>,
}

impl Grid {
    fn new(rows: usize, columns: usize) -> Self {
        Grid {
            inner: iter::repeat(
                iter::repeat(false).take(rows).collect::<VecDeque<bool>>()
            ).take(columns).collect::<VecDeque<VecDeque<bool>>>(),
        }
    }

    fn apply(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Rectangle(columns, rows) => {
                self.inner.make_contiguous()[0..rows].iter_mut().for_each(|row| {
                    row.make_contiguous()[0..columns].iter_mut().for_each(|x| *x = true);
                });
            },
            Instruction::RotateRow(row, amount) => {
                self.inner.get_mut(row).unwrap().rotate_right(amount);
            },
            Instruction::RotateColumn(column, amount) => {
                let mut new_column = VecDeque::new();
                for row in self.inner.iter() {
                    new_column.push_back(row[column]);
                }
                new_column.rotate_right(amount);
                for (row_idx, row) in self.inner.iter_mut().enumerate() {
                    row[column] = new_column[row_idx];
                }
            },
        }
    }

    fn count_on(&self) -> usize {
        self.inner.iter().map(|row| row.iter().filter(|cell| **cell).count()).sum::<usize>()
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.inner.iter() {
            writeln!(f, "{}", row.iter().map(|x| if *x { '#' } else { '.' }).collect::<String>())?
        }
        Ok(())
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|x| {
            let split = x.splitn(3, ' ').collect_vec();

            match (split[0], split[1]) {
                ("rect", s) => {
                    let (l, r) = s.split_once('x').unwrap();
                    Instruction::Rectangle(l.parse().unwrap(), r.parse().unwrap())
                },
                ("rotate", "row") => {
                    let (l, r) = split[2].split(" by ").map(|s| s.trim_matches(['x', 'y', '='])).collect_tuple().unwrap();
                    Instruction::RotateRow(l.parse().unwrap(), r.parse().unwrap())
                },
                ("rotate", "column") => {
                    let (l, r) = split[2].split(" by ").map(|s| s.trim_matches(['x', 'y', '='])).collect_tuple().unwrap();
                    Instruction::RotateColumn(l.parse().unwrap(), r.parse().unwrap())
                },
                _ => panic!("Invalid data"),
            }
        })
        .collect_vec()
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Rectangle(usize, usize),
    RotateRow(usize, usize),
    RotateColumn(usize, usize),
}