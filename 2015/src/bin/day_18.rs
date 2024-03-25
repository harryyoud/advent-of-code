use aoc_2015::get_input;
use grid::{Grid, X_LENGTH, Y_LENGTH};


mod grid {
    pub const X_LENGTH: u32 = 100;
    pub const Y_LENGTH: u32 = 100;

    #[derive(Clone, Copy, Debug)]
    pub struct Point(u32, u32);

    impl From<(u32, u32)> for Point {
        fn from((x, y): (u32, u32)) -> Self {
            Self(x, y)
        }
    }
    
    #[derive(Clone)]
    pub struct Grid {
        inner: [[bool; X_LENGTH as usize]; Y_LENGTH as usize],
    }
    
    impl Grid {
        pub fn new(inner: [[bool; X_LENGTH as usize]; Y_LENGTH as usize]) -> Self {
            Self { inner }
        }

        pub fn get(&self, point: impl Into<Point>) -> bool {
            let point = point.into();
            self.inner[point.0 as usize][point.1 as usize]
        }

        pub fn set(&mut self, point: impl Into<Point>, value: bool) {
            let point = point.into();
            self.inner[point.0 as usize][point.1 as usize] = value;
        }

        pub fn neighbours(&self, point: impl Into<Point>) -> Vec<(Point, bool)> {
            let point = point.into();
            let min_x = point.0.saturating_sub(1);
            let max_x = (point.0 + 1).min(99);
            let min_y = point.1.saturating_sub(1);
            let max_y = (point.1 + 1).min(99);

            let mut out = vec![];
    
            for x in min_x..=max_x {
                for y in min_y..=max_y {
                    if x == point.0 && y == point.1 {
                        continue;
                    }
                    let z: Point = (x, y).into();
                    out.push((z, self.get(z)));
                }
            }

            out
        }

        pub fn count_on(&self) -> u64 {
            self.inner.iter().map(|x| x.iter().filter(|y| **y).count() as u64).sum()
        }
    }
}

fn main() {
    let input = get_input(18);
    let grid = parse_input(&input);

    dbg!(part_1(&grid));
    dbg!(part_2(&grid));
}

fn part_1(grid: &Grid) -> u64 {
    let mut grid = grid.clone();
    for _ in 0..100 {
        grid = tick(&grid);
    }
    grid.count_on()
}

fn part_2(grid: &Grid) -> u64 {
    let mut grid = grid.clone();
    for _ in 0..100 {
        grid = tick(&grid);
        grid.set((0, 0), true);
        grid.set((99,0), true);
        grid.set((0, 99), true);
        grid.set((99, 99), true);
    }
    grid.count_on()
}

fn tick(grid: &Grid) -> Grid {
    let mut next_grid = grid.clone();
    for x in 0..X_LENGTH {
        for y in 0..Y_LENGTH {
            let neighbours_on = grid.neighbours((x, y)).iter()
                .filter(|(_point, value)| *value)
                .count();
            let current = grid.get((x, y));
            if current {
                next_grid.set((x, y), [2, 3].contains(&neighbours_on));
            } else {
                next_grid.set((x, y), neighbours_on == 3);
            }
        }
    } next_grid
}

fn parse_input(input: &str) -> Grid {
    let mut grid = [[false; 100]; 100];

    for (x, line) in input.lines().enumerate() {
        for (y, val) in line.chars().enumerate() {
            grid[x][y] = match val {
                '.' => false,
                '#' => true,
                _ => panic!("Invalid character in input: line {x}, character {y}"),
            };
        }
    }

    Grid::new(grid)
}