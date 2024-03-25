use std::collections::{HashMap, HashSet};

use aoc_2016::get_input;
use itertools::Itertools;
use pathfinding::directed::dijkstra::dijkstra;

fn main() {
    let input = get_input(24);
    let grid = parse_input(&input);

    dbg!(part_1(&grid));
    dbg!(part_2(&grid));
}

fn part_1(grid: &Grid) -> usize {
    let pair_distances = pair_distances(&grid);
    let mut min_distance = usize::MAX;

    for order in grid.to_visit.iter().permutations(grid.to_visit.len()).filter(|x| x.first().unwrap().1 == 0) {
        let order = order.iter().map(|x| x.1).collect_vec();
        min_distance = min_distance.min(calculate_distance(&order, &pair_distances));
    }

    min_distance
}

fn part_2(grid: &Grid) -> usize {
    let pair_distances = pair_distances(&grid);
    let mut min_distance = usize::MAX;

    for order in grid.to_visit.iter().permutations(grid.to_visit.len()).filter(|x| x.first().unwrap().1 == 0) {
        let mut order = order.iter().map(|x| x.1).collect_vec();
        order.push(0);
        min_distance = min_distance.min(calculate_distance(&order, &pair_distances));
    }

    min_distance
}

fn calculate_distance(order: &[u32], pair_distances: &HashMap<(u32, u32), usize>) -> usize {
    let mut distance_travelled = 0;
    for (a, b) in order.iter().tuple_windows() {
        distance_travelled += pair_distances.get(&(*a.min(b), *a.max(b))).unwrap();
    }
    distance_travelled
}

fn pair_distances(grid: &Grid) -> HashMap<(u32, u32), usize> {
    let mut distances = HashMap::new();
    for ((a_pos, a), (b_pos, b)) in grid.to_visit.iter().tuple_combinations() {
        let Some((_path, distance)) = dijkstra(
            a_pos,
            |x| grid.neighbours(*x).into_iter().map(|y| (y, 1)).collect_vec(),
            |x| x == b_pos,
        ) else {
            panic!();
        };
        distances.insert((*a.min(b), *a.max(b)), distance);
    }
    distances
}

fn parse_input(input: &str) -> Grid {
    let mut map = HashMap::new();
    let mut to_visit = HashSet::new();
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert(Point {x, y}, match c {
                '#' => Tile::Wall,
                '0'..='9' => {
                    to_visit.insert((Point {x, y}, c.to_digit(10).unwrap()));
                    Tile::Number
                },
                '.' => Tile::Empty,
                _ => panic!("Invalid character at line {y}, character {x}: {c}"),
            });
        }
    }
    Grid {
        map, to_visit, width, height
    }
}

struct Grid {
    map: HashMap<Point, Tile>,
    to_visit: HashSet<(Point, u32)>,
    width: usize,
    height: usize,
}

impl Grid {
    fn neighbours(&self, point: Point) -> Vec<Point> {
        let mut move_to = vec![];

        if point.x > 0 && !self.map.get(&point.left()).unwrap().is_wall() {
            move_to.push(point.left());
        }
        if point.x < self.width && !self.map.get(&point.right()).unwrap().is_wall() {
            move_to.push(point.right());
        }
        if point.y > 0 && !self.map.get(&point.up()).unwrap().is_wall() {
            move_to.push(point.up());
        }
        if point.y < self.height && !self.map.get(&point.down()).unwrap().is_wall() {
            move_to.push(point.down());
        }

        move_to
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn down(&self) -> Point {
        Point {x: self.x, y: self.y - 1}
    }
    fn up(&self) -> Point {
        Point {x: self.x, y: self.y + 1}
    }
    fn right(&self) -> Point {
        Point {x: self.x + 1, y: self.y}
    }
    fn left(&self) -> Point {
        Point {x: self.x - 1, y: self.y}
    }
}

enum Tile {
    Wall,
    Number,
    Empty,
}

impl Tile {
    fn is_wall(&self) -> bool {
        matches!(self, Tile::Wall)
    }
}