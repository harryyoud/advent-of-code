use std::collections::HashSet;

use aoc_2015::get_input;
use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Point(isize, isize);

impl Point {
    fn up(&self) -> Self {
        Self(self.0, self.1 + 1)
    }
    fn down(&self) -> Self {
        Self(self.0, self.1 - 1)
    }
    fn right(&self) -> Self {
        Self(self.0 + 1, self.1)
    }
    fn left(&self) -> Self {
        Self(self.0 - 1, self.1)
    }
}

fn main() {
    let input = get_input(3);

    dbg!(part_1(&input));
    dbg!(part_2(&input));
}

fn part_1(input: &str) -> u64 {
    let mut points_visited: HashSet<Point> = HashSet::new();
    points_visited.insert(Point(0, 0));

    let mut current_point = Point(0, 0);
    
    for c in input.chars() {
        current_point = move_point(current_point, c);
        points_visited.insert(current_point);
    }

    points_visited.len() as u64
}

fn part_2(input: &str) -> u64 {
    let mut points_visited: HashSet<Point> = HashSet::new();
    points_visited.insert(Point(0, 0));

    let mut current_point_a = Point(0, 0);
    let mut current_point_b = Point(0, 0);
    
    for (a, b) in input.chars().tuples() {
        current_point_a = move_point(current_point_a, a);
        points_visited.insert(current_point_a);

        current_point_b = move_point(current_point_b, b);
        points_visited.insert(current_point_b);
    }

    points_visited.len() as u64
}

fn move_point(point: Point, c: char) -> Point {
    match c {
        '^' => point.up(),
        '>' => point.right(),
        '<' => point.left(),
        'v' => point.down(),
        _ => panic!("Invalid character: {c}"),
    }
}