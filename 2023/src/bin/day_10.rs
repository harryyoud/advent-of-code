use std::collections::HashMap;

use aoc_2023::get_input;
use itertools::Itertools;

enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Debug, Clone)]
enum Tile {
    Vertical,
    Horizontal,
    NorthToEast,
    SouthToEast,
    SouthToWest,
    NorthToWest,
    Ground,
    StartingPoint,
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NorthToEast,
            'J' => Self::NorthToWest,
            '7' => Self::SouthToWest,
            'F' => Self::SouthToEast,
            '.' => Self::Ground,
            'S' => Self::StartingPoint,
            _ => return Err(()),
        })
    }
}

impl Tile {
    fn could_connect_to(&self, direction: &Direction) -> bool {
        match (self, direction) {
            (Tile::Vertical, Direction::North | Direction::South) => true,
            (Tile::Horizontal, Direction::East | Direction::West) => true,
            (Tile::NorthToEast, Direction::North | Direction::East) => true,
            (Tile::NorthToWest, Direction::North | Direction::West) => true,
            (Tile::SouthToWest, Direction::South | Direction::West) => true,
            (Tile::SouthToEast, Direction::South | Direction::East) => true,
            (Tile::StartingPoint, _) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Coords {
    x: u32,
    y: u32,
}

impl From<(u32, u32)> for Coords {
    fn from(value: (u32, u32)) -> Self {
        Coords {
            x: value.0,
            y: value.1,
        }
    }
}

impl Coords {
    fn north(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }
    fn south(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn west(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }
    fn east(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }
}

#[derive(Debug)]
struct Grid {
    inner: Vec<Vec<Tile>>,
    start: Coords,
}

impl Grid {
    fn get_neighbours(&self, coords: &Coords) -> Vec<(Direction, Coords, Tile)> {
        let mut out = vec![];
        if coords.y < self.inner.len() as u32 {
            out.push((
                Direction::South,
                coords.south(),
                self.get_tile_at(&coords.south()),
            ));
        }
        if coords.y > 0 {
            out.push((
                Direction::North,
                coords.north(),
                self.get_tile_at(&coords.north()),
            ));
        }
        if coords.x < self.inner[0].len() as u32 {
            out.push((
                Direction::East,
                coords.east(),
                self.get_tile_at(&coords.east()),
            ));
        }
        if coords.x > 0 {
            out.push((
                Direction::West,
                coords.west(),
                self.get_tile_at(&coords.west()),
            ));
        }
        out
    }

    fn get_tile_at(&self, coords: &Coords) -> Tile {
        self.inner[coords.y as usize][coords.x as usize].clone()
    }

    fn get_connected_neighbours(&self, coords: &Coords) -> Vec<(Direction, Coords, Tile)> {
        let neighbours = self.get_neighbours(coords);
        let current_tile = self.get_tile_at(coords);

        neighbours
            .into_iter()
            .filter(|(direction, _neighbour_coord, neighbour_tile)| {
                neighbour_tile.could_connect_to(&direction.opposite())
                    && current_tile.could_connect_to(direction)
            })
            .collect_vec()
    }
}

fn main() {
    let input = get_input(10);
    let tiles = input
        .lines()
        .map(|s| s.chars().map(|c| Tile::try_from(c).unwrap()).collect_vec())
        .collect_vec();

    let mut start_position = None;
    for (row_idx, row) in tiles.iter().enumerate() {
        for (col_idx, tile) in row.iter().enumerate() {
            if matches!(tile, Tile::StartingPoint) {
                start_position = Some((row_idx as u32, col_idx as u32));
                break;
            }
        }
    }

    let grid = Grid {
        inner: tiles,
        start: Coords {
            x: start_position.unwrap().0,
            y: start_position.unwrap().1,
        },
    };

    let mut visited: HashMap<Coords, u32> = HashMap::new();
    let mut to_visit: Vec<Coords> = vec![grid.start.clone()];
    let mut part_a = 0u32;

    loop {
        let mut next_visits: Vec<Coords> = vec![];
        for coords in to_visit {
            visited.insert(coords.clone(), part_a);
            for (_direction, neighbour_coords, _tile) in grid.get_connected_neighbours(&coords) {
                if visited.contains_key(&neighbour_coords) {
                    continue;
                }
                next_visits.push(neighbour_coords);
            }
        }
        if next_visits.is_empty() {
            break;
        }
        to_visit = next_visits;
        part_a += 1;
    }

    dbg!(part_a);
}
