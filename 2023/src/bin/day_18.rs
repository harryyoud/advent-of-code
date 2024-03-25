#![feature(int_roundings)]

use aoc_2023::get_input;
use itertools::Itertools;

type Point = (i64, i64);

trait PointExt {
    fn move_dir(&self, direction: &str, distance: i64) -> Self;
}

impl PointExt for Point {
    fn move_dir(&self, direction: &str, distance: i64) -> Self {
        match direction {
            "L" => (self.0 - distance, self.1),
            "R" => (self.0 + distance, self.1),
            "U" => (self.0, self.1 + distance),
            "D" => (self.0, self.1 - distance),
            _ => panic!("Invalid direction: {direction}"),
        }
    }
}

fn main() {
    let input = get_input(18);

    let (corners, edge_length) = parse_input(&input, false);
    let part_a = get_area(&corners, edge_length);
    dbg!(part_a);

    let (corners, edge_length) = parse_input(&input, true);
    let part_b = get_area(&corners, edge_length);
    dbg!(part_b);
}

fn parse_input(input: &str, part_b: bool) -> (Vec<Point>, i64) {
    let mut corners: Vec<Point> = vec![(0, 0)];
    let mut edge_length = 1;

    for line in input.lines() {
        let (direction, distance) = if part_b { parse_line_b(line) } else { parse_line_a(line) };
        let point = corners.last().unwrap().move_dir(direction, distance);
        edge_length += distance;
        corners.push(point);
    }

    (corners, edge_length)
}

fn parse_line_a(line: &str) -> (&str, i64) {
    let (direction, distance, _) = line.split_whitespace().collect_tuple().unwrap();
    (direction, distance.parse().unwrap())
}

fn parse_line_b(line: &str) -> (&str, i64) {
    let (_, _, colour) = line.split_whitespace().collect_tuple().unwrap();
    (   match colour.chars().nth(7).unwrap() {
            '0' => "R",
            '1' => "D",
            '2' => "L",
            '3' => "U",
            x => panic!("Invalid direction: {x}"),
        },
        i64::from_str_radix(&colour[2..7], 16).unwrap()
    )
}


fn get_area(corners: &[Point], edge_length: i64) -> i64 {
    let mut total_area = 0;
    for (point_a, point_b) in corners.into_iter().circular_tuple_windows() {
        // matrix determinant
        total_area += (point_a.0 * point_b.1) - (point_a.1 * point_b.0);
    }
    total_area = total_area.abs() + edge_length;
    total_area.div_ceil(2)
}