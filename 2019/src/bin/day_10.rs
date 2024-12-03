use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use aoc_2019::{gcd, get_input};
use types::*;

fn main() {
    let asteroids = parse_input(&get_input(10));

    let (best_location, count) = find_best_location(&asteroids);
    dbg!(count);
    let (point_200th, _angle) = &run_laser(asteroids, best_location)[199];
    dbg!(point_200th.x * 100 + point_200th.y);
}

fn find_best_location(asteroids: &HashSet<Point>) -> (Point, u32) {
    let mut seen = HashMap::<Point, u32>::new();

    for asteroid in asteroids.iter() {
        for other in asteroids.iter() {
            if asteroid == other {
                continue;
            }
            if can_see(&asteroids, *asteroid, *other) {
                *seen.entry(*asteroid).or_default() += 1;
            }
        }
    }

    seen.into_iter().max_by_key(|(_point, count)| *count).unwrap()
}

fn run_laser(mut asteroids: HashSet<Point>, laser: Point) -> Vec<(Point, f64)> {
    asteroids.remove(&laser);

    let mut asteroid_angles = HashMap::new();
    for asteroid in asteroids.into_iter() {
        let angle = calculate_bearing(laser, asteroid);
        asteroid_angles.insert(asteroid, angle);
    }

    let mut removed_order = vec![];

    while !asteroid_angles.is_empty() {
        let candidates = asteroid_angles
            .iter()
            .sorted_by(|(_point_a, angle_a), (_point_b, angle_b)| {
                angle_a.partial_cmp(angle_b).unwrap()
            })
            .map(|(point, _angle)| *point)
            .collect_vec();
        let ch = candidates.iter().copied().collect::<HashSet<Point>>();

        for point in candidates.iter().filter(|x| can_see(&ch, laser, **x)) {
            removed_order.push((*point, *asteroid_angles.get(point).unwrap()));
        }

        for remove in removed_order.iter() {
            asteroid_angles.remove(&remove.0);
        }
    }

    removed_order
}

fn parse_input(input: &str) -> HashSet<Point> {
    let mut asteroids = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => { asteroids.insert((x, y).into()); },
                '.' => {},
                _ => panic!("Invalid character '{c}', line {y}, character {x}"),
            }
        }
    }
    asteroids
}

fn calculate_line(point_a: Point, point_b: Point) -> Line {
    let x_diff = point_b.x as i32 - point_a.x as i32;
    let y_diff = point_b.y as i32 - point_a.y as i32;

    let gcd = gcd(x_diff.unsigned_abs() as usize, y_diff.unsigned_abs() as usize) as u32;

    Line {
        x_step: x_diff / gcd as i32,
        y_step: y_diff / gcd as i32,
    }
}

fn calculate_bearing(point_a: Point, point_b: Point) -> f64 {
    let x_diff = point_b.x as i32 - point_a.x as i32;
    let y_diff = point_b.y as i32 - point_a.y as i32;

    // take angle difference between x=0 in -y direction (upwards)
    (((y_diff as f64).atan2(x_diff as f64) - (-1.0_f64.atan2(0.0_f64))).to_degrees() + 720.0) % 360.0
}

fn can_see(asteroids: &HashSet<Point>, from: Point, to: Point) -> bool {
    LineIterator {
        line: calculate_line(from, to),
        current: from,
        limit: to,
    }.all(|x| !asteroids.contains(&x))
}

mod types {
    #[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
    pub struct Point {
        pub x: u32,
        pub y: u32,
    }

    #[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
    pub struct Line {
        pub x_step: i32,
        pub y_step: i32,
    }

    pub struct LineIterator {
        pub line: Line,
        pub current: Point,
        pub limit: Point,
    }

    impl Iterator for LineIterator {
        type Item = Point;

        fn next(&mut self) -> Option<Self::Item> {
            self.current = Point {
                x: self.current.x.saturating_add_signed(self.line.x_step),
                y: self.current.y.saturating_add_signed(self.line.y_step),
            };

            if self.line.x_step > 0 && self.current.x >= self.limit.x {
                return None;
            }
            if self.line.x_step < 0 && self.current.x <= self.limit.x {
                return None;
            }
            if self.line.y_step > 0 && self.current.y >= self.limit.y {
                return None;
            }
            if self.line.y_step < 0 && self.current.y <= self.limit.y {
                return None;
            }

            Some(self.current)
        }
    }

    impl From<(u32, u32)> for Point {
        fn from((x, y): (u32, u32)) -> Self {
            Point {
                x, y
            }
        }
    }
    impl From<(usize, usize)> for Point {
        fn from((x, y): (usize, usize)) -> Self {
            Point {
                x: x as u32,
                y: y as u32,
            }
        }
    }
}
