use aoc_2019::get_input;
use itertools::Itertools;
use std::collections::HashSet;

const ORIGIN: Point = Point { x: 0, y: 0 };

fn main() {
    let lines = parse_lines_from_input(&get_input(3));

    let intersections = intersections(lines.clone());
    dbg!(closest_intersection_manhattan(intersections.clone()).manhattan(ORIGIN));
    dbg!(closest_intersection_path(intersections, lines).1);
}

fn parse_lines_from_input(input: &str) -> Vec<Vec<Point>> {
    input
        .lines()
        .map(|line| {
            points_on_line(
                &line
                    .split(",")
                    .map(|vector| parse_move(vector))
                    .collect_vec(),
            )
        })
        .collect_vec()
}

fn parse_move(input: &str) -> (Direction, u32) {
    let mut chars = input.chars();
    (
        Direction::parse(chars.next().expect("Ran out of characters")),
        chars
            .collect::<String>()
            .parse::<u32>()
            .expect("Ran out of characters"),
    )
}

fn intersections(lines: Vec<Vec<Point>>) -> Vec<Point> {
    lines
        .into_iter()
        .map(|a| HashSet::<Point>::from_iter(a.into_iter()))
        .reduce(|a, b| a.intersection(&b).copied().collect())
        .expect("Need more than 2 or more lines to calculate intersections")
        .into_iter()
        .collect_vec()
}

fn closest_intersection_manhattan(intersections: Vec<Point>) -> Point {
    intersections
        .into_iter()
        .min_by_key(|a| a.manhattan(ORIGIN))
        .unwrap()
}

fn closest_intersection_path(intersections: Vec<Point>, lines: Vec<Vec<Point>>) -> (Point, usize) {
    intersections
        .into_iter()
        .map(|intersection| {
            (
                intersection,
                lines
                    .iter()
                    .map(|line| {
                        line.iter()
                            .position(|point| *point == intersection)
                            .expect("Could not find position of intersection in line")
                            + 1
                    })
                    .sum::<usize>(),
            )
        })
        .min_by_key(|(_point, combined_path_distance)| *combined_path_distance)
        .expect("Unable to find intersection with shortest path")
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn travel(&self, dir: Direction, amount: u32) -> Point {
        match dir {
            Direction::Right => Self {
                x: self.x + amount as i32,
                ..*self
            },
            Direction::Left => Self {
                x: self.x - amount as i32,
                ..*self
            },
            Direction::Up => Self {
                y: self.y + amount as i32,
                ..*self
            },
            Direction::Down => Self {
                y: self.y - amount as i32,
                ..*self
            },
        }
    }

    fn manhattan(&self, other: Self) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

fn points_on_line(moves: &[(Direction, u32)]) -> Vec<Point> {
    let mut current = Point { x: 0, y: 0 };
    let mut points = vec![];
    for m in moves {
        for _ in 0..m.1 {
            current = current.travel(m.0, 1);
            points.push(current);
        }
    }
    points
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    fn parse(input: char) -> Self {
        use Direction::*;
        match input {
            'R' => Right,
            'L' => Left,
            'U' => Up,
            'D' => Down,
            _ => panic!("Invalid direction"),
        }
    }
}
