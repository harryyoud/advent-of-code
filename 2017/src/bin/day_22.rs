use std::collections::HashMap;

use aoc_2017::get_input;

fn main() {
    let input = get_input(22);
    let cluster = parse_input(&input);

    dbg!(part_1(cluster.clone()));
    dbg!(part_2(cluster.clone()));
}

fn part_1(mut cluster: Cluster) -> usize {
    simple_infect_n(&mut cluster, 10_000)
}

fn part_2(mut cluster: Cluster) -> usize {
    advanced_infect_n(&mut cluster, 10_000_000)
}

fn simple_infect_n(cluster: &mut Cluster, iterations: usize) -> usize {
    let mut virus = VirusState::new(cluster.middle);
    let mut total_infected = 0;

    for _ in 0..iterations {
        match cluster
            .nodes
            .entry(virus.position)
            .or_insert(TileType::Clean)
        {
            tile_type @ TileType::Clean => {
                virus.facing.turn_left();
                *tile_type = TileType::Infected;
                total_infected += 1;
            }
            tile_type @ TileType::Infected => {
                virus.facing.turn_right();
                *tile_type = TileType::Clean;
            }
            _ => panic!("Simple infection cannot deal with weakened or flagged nodes!"),
        }
        virus.position.move_direction(virus.facing);
    }

    total_infected
}

fn advanced_infect_n(cluster: &mut Cluster, iterations: usize) -> usize {
    let mut virus = VirusState::new(cluster.middle);
    let mut total_infected = 0;

    for _ in 0..iterations {
        let tile_type = cluster
            .nodes
            .entry(virus.position)
            .or_insert(TileType::Clean);
        match tile_type {
            TileType::Clean => {
                virus.facing.turn_left();
            }
            TileType::Weakened => {
                // will not turn
            }
            TileType::Infected => virus.facing.turn_right(),
            TileType::Flagged => virus.facing.reverse(),
        }
        tile_type.tick();
        if matches!(tile_type, TileType::Infected) {
            total_infected += 1;
        }
        virus.position.move_direction(virus.facing);
    }

    total_infected
}

struct VirusState {
    position: Point,
    facing: Direction,
}

impl VirusState {
    fn new(starting_point: Point) -> Self {
        Self {
            position: starting_point,
            facing: Direction::Up,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_left(&mut self) {
        use Direction::*;
        *self = match self {
            Up => Left,
            Down => Right,
            Left => Down,
            Right => Up,
        }
    }
    fn turn_right(&mut self) {
        use Direction::*;
        *self = match self {
            Up => Right,
            Down => Left,
            Left => Up,
            Right => Down,
        }
    }
    fn reverse(&mut self) {
        use Direction::*;
        *self = match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}

#[derive(Debug, Clone)]
struct Cluster {
    nodes: HashMap<Point, TileType>,
    middle: Point,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TileType {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

impl TileType {
    fn tick(&mut self) {
        use TileType::*;
        *self = match self {
            Clean => Weakened,
            Weakened => Infected,
            Infected => Flagged,
            Flagged => Clean,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn move_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }
}

fn parse_input(input: &str) -> Cluster {
    let infected_nodes = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                '.' => None,
                '#' => Some((
                    Point {
                        x: x as isize,
                        y: y as isize,
                    },
                    TileType::Infected,
                )),
                _ => panic!("Invalid character in input (line {x}, char {y}): {c}"),
            })
        })
        .collect::<HashMap<_, _>>();

    Cluster {
        nodes: infected_nodes,
        middle: Point {
            x: (input.lines().next().unwrap().len() / 2) as isize,
            y: (input.lines().count() / 2) as isize,
        },
    }
}
