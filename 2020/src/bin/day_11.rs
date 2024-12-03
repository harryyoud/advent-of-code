use std::collections::{BTreeMap, HashSet};
use std::fmt::{Display, Formatter, Write};
use itertools::Itertools;
use aoc_2020::{get_input, Vec2d};

fn main() {
    let input = get_input(11);

    let mut tiles = BTreeMap::new();
    for (y, row) in input.lines().enumerate() {
        for (x, c) in row.chars().enumerate() {
            tiles.insert((x as isize, y as isize).into(), Tile::try_from(c).unwrap());
        }
    }

    let grid = Grid::new(
        tiles.clone(),
        input.lines().count(),
        input.lines().next().unwrap().len()
    );

    dbg!(part_1(grid.clone()));

    dbg!(part_2(grid));
}

fn part_1(mut grid: Grid) -> usize {
    let mut seen = HashSet::new();

    while !seen.contains(&grid) {
        seen.insert(grid.clone());
        grid.tick_v1();
    }

    grid.tiles.values().filter(|x| x.is_occupied()).count()
}

fn part_2(mut grid: Grid) -> usize {
    let mut seen = HashSet::new();

    while !seen.contains(&grid) {
        seen.insert(grid.clone());
        grid.tick_v2();
    }

    grid.tiles.values().filter(|x| x.is_occupied()).count()
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Grid {
    tiles: BTreeMap<Vec2d, Tile>,
    height: usize,
    width: usize,
}

impl Grid {
    fn get_at(&self, pos: Vec2d) -> Option<Tile> {
        self.tiles.get(&pos).copied()
    }

    fn direct_neighbours(&self, pos: Vec2d) -> Vec<Tile> {
        pos.neighbours()
            .into_iter()
            .filter_map(|x| self.get_at(x))
            .collect_vec()
    }

    fn visible_neighbours(&self, pos: Vec2d) -> Vec<Tile> {
        let movements = vec![
            |x: Vec2d| x.up().left(),
            |x: Vec2d| x.up(),
            |x: Vec2d| x.up().right(),
            |x: Vec2d| x.right(),
            |x: Vec2d| x.down().right(),
            |x: Vec2d| x.down(),
            |x: Vec2d| x.down().left(),
            |x: Vec2d| x.left(),
        ];

        let mut visible = vec![];

        for f in movements {
            let mut cursor = f(pos);
            while let Some(Tile::Floor) = self.get_at(cursor) {
                cursor = f(cursor);
            }
            if let Some(x) = self.get_at(cursor) {
                visible.push(x);
            }
        }

        visible
    }

    fn new(tiles: BTreeMap<Vec2d, Tile>, height: usize, width: usize) -> Self {
        Self {
            tiles, height, width,
        }
    }

    fn tick_v1(&mut self) {
        let mut new = self.clone();
        for pos in self.tiles.keys() {
            new.tiles.insert(*pos, self.next_state(*pos, &self.direct_neighbours(*pos), 4).unwrap());
        }
        *self = new;
    }

    fn tick_v2(&mut self) {
        let mut new = self.clone();
        for pos in self.tiles.keys() {
            new.tiles.insert(*pos, self.next_state(*pos, &self.visible_neighbours(*pos), 5).unwrap());
        }
        *self = new;
    }

    fn next_state(&self, pos: Vec2d, neighbours: &[Tile], x: usize) -> Option<Tile> {
        use Tile::*;
        let state = self.get_at(pos)?;
        let occupied_neighbours = neighbours.into_iter().filter(|x| x.is_occupied()).count();

        match (state, occupied_neighbours) {
            (EmptySeat, 0) => Some(OccupiedSeat),
            (OccupiedSeat, _) => if occupied_neighbours >= x { Some(EmptySeat) } else { Some(state) },
            _ => Some(state),
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                f.write_char(match self.tiles.get(&(x, y).into()) {
                    None => ' ',
                    Some(Tile::Floor) => '.',
                    Some(Tile::EmptySeat) => 'L',
                    Some(Tile::OccupiedSeat) => '#',
                })?;
            }
            f.write_char('\n')?;
        }
        f.write_char('\n')
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
enum Tile {
    EmptySeat,
    OccupiedSeat,
    Floor,
}

impl Tile {
    fn is_occupied(&self) -> bool {
        match self {
            Tile::EmptySeat => false,
            Tile::OccupiedSeat => true,
            Tile::Floor => false,
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Tile::*;
        match value {
            '#' => Ok(OccupiedSeat),
            'L' => Ok(EmptySeat),
            '.' => Ok(Floor),
            _ => Err(())
        }
    }
}