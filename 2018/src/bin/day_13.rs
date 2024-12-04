use std::collections::HashMap;

use aoc_2018::get_input;
use itertools::Itertools;

fn main() {
    let input = get_input(13);
    let mut grid = parse_input(&input);
    grid.run_until_final_cart();

    dbg!(part_1(&grid));
    dbg!(part_2(&grid));
}

fn part_1(grid: &Grid) -> String {
    format!("({}, {})", grid.collisions[0].0, grid.collisions[0].1)
}

fn part_2(grid: &Grid) -> String {
    format!(
        "({}, {})",
        grid.carts[0].position.0, grid.carts[0].position.1
    )
}

fn parse_input(input: &str) -> Grid {
    let mut cells = HashMap::new();
    let mut carts = Vec::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if let Some(direction) = Direction::parse_from(c) {
                carts.push(Cart::new(direction, (x as isize, y as isize)));
            }
            if let Some(cell) = Cell::parse_from(c) {
                cells.insert((x as isize, y as isize), cell);
            }
        }
    }

    Grid {
        cells,
        carts,
        collisions: vec![],
    }
}

#[derive(Clone, Debug)]
struct Grid {
    cells: HashMap<(isize, isize), Cell>,
    carts: Vec<Cart>,
    collisions: Vec<(isize, isize)>,
}

impl Grid {
    fn run_until_final_cart(&mut self) {
        loop {
            // sort carts so that we go through them L to R, top to bottom
            self.carts.sort_by_key(|c| (c.position.1, c.position.0));

            for idx in 0..self.carts.len() {
                self.carts[idx].tick(&self.cells);

                let curr_position = self.carts[idx].position;
                let collisions = self
                    .carts
                    .iter_mut()
                    .filter(|c| c.position == curr_position)
                    .collect_vec();

                if collisions.len() >= 2 {
                    self.collisions.push(curr_position);
                    for collision in collisions {
                        collision.collided = true;
                    }
                    self.carts[idx].collided = true;
                }
            }

            self.carts.retain(|c| !c.collided);
            if self.carts.len() == 1 {
                break;
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Cart {
    direction: Direction,
    position: (isize, isize),
    next_junction_direction: JunctionMovement,
    collided: bool,
}

impl Cart {
    fn new(direction: Direction, position: (isize, isize)) -> Self {
        Cart {
            direction,
            position,
            next_junction_direction: JunctionMovement::Left,
            collided: false,
        }
    }

    fn tick(&mut self, map: &HashMap<(isize, isize), Cell>) {
        let cart = self;

        // move first then set up for next movement
        cart.position = match cart.direction {
            Direction::Up => (cart.position.0, cart.position.1 - 1),
            Direction::Down => (cart.position.0, cart.position.1 + 1),
            Direction::Left => (cart.position.0 - 1, cart.position.1),
            Direction::Right => (cart.position.0 + 1, cart.position.1),
        };

        match (
            cart.direction,
            map.get(&cart.position).expect("Track ends abruptly!"),
        ) {
            (_, Cell::Crossroads) => {
                match cart.next_junction_direction {
                    JunctionMovement::StraightOn => {}
                    JunctionMovement::Left => cart.direction.turn_left(),
                    JunctionMovement::Right => cart.direction.turn_right(),
                };
                cart.next_junction_direction.next();
            }
            (Direction::Up | Direction::Down, Cell::ForwardSlash) => cart.direction.turn_right(),
            (Direction::Left | Direction::Right, Cell::ForwardSlash) => cart.direction.turn_left(),
            (Direction::Up | Direction::Down, Cell::BackSlash) => cart.direction.turn_left(),
            (Direction::Left | Direction::Right, Cell::BackSlash) => cart.direction.turn_right(),
            _ => {}
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn parse_from(x: char) -> Option<Self> {
        use Direction::*;
        Some(match x {
            'v' => Down,
            '^' => Up,
            '>' => Right,
            '<' => Left,
            _ => return None,
        })
    }

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
}

#[derive(Copy, Clone, Debug)]
enum JunctionMovement {
    Left,
    StraightOn,
    Right,
}

impl JunctionMovement {
    fn next(&mut self) {
        use JunctionMovement::*;
        *self = match self {
            Left => StraightOn,
            StraightOn => Right,
            Right => Left,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Cell {
    Horizontal,
    Vertical,
    Crossroads,
    ForwardSlash,
    BackSlash,
}

impl Cell {
    fn parse_from(x: char) -> Option<Self> {
        use Cell::*;
        Some(match x {
            '-' | '>' | '<' => Horizontal,
            '|' | '^' | 'v' => Vertical,
            '+' => Crossroads,
            '/' => ForwardSlash,
            '\\' => BackSlash,
            _ => return None,
        })
    }
}
