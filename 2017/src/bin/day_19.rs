use std::collections::HashMap;

use aoc_2017::get_input;
use itertools::Itertools;
use pathfinding::directed::dijkstra::dijkstra_all;

fn main() {
    let input = get_input(19);
    let (grid, start) = parse_input(&input);

    dbg!(part_1(&grid, start));
    dbg!(part_2(&grid, start));
}

fn part_1(grid: &Grid, start: (Point, Tile, Axis)) -> String {
    solve(grid, start).into_iter().map(|(letter, _cost)| letter).collect()
}
fn part_2(grid: &Grid, start: (Point, Tile, Axis)) -> usize {
    // offset by one, as start point counts as 1
    solve(grid, start).last().unwrap().1 + 1
}

fn solve(grid: &Grid, start: (Point, Tile, Axis)) -> Vec<(char, usize)> {
    let path = dijkstra_all(
        &start,
        |x| neighbours(grid, *x).into_iter().map(|a| (a, 1))
    );
    
    path
        .into_iter()
        .filter_map(|((_point, tile, _axis), (_optimal_parent, cost))| {
            if let Tile::PointOfInterest(letter) = tile {
                return Some((letter, cost));
            }
            None
        })
        .sorted_by_key(|(_letter, cost)| *cost)
        .collect_vec()
}

type Grid = HashMap<Point, Tile>;
type Point = (isize, isize);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Tile {
    Corner,
    Horizontal,
    Vertical,
    PointOfInterest(char),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Axis {
    Vertical,
    Horizontal,
}

impl Axis {
 fn opposite(self) -> Self {
    use Axis::*;
    match self {
        Vertical => Horizontal,
        Horizontal => Vertical,
    }
 }
}

fn neighbours(grid: &Grid, (point, tile, mut axis): (Point, Tile, Axis)) -> Vec<(Point, Tile, Axis)> {
    if matches!(tile, Tile::Corner) {
        axis = axis.opposite();
    }
    movements_for_axis(point, axis)
        .into_iter()
        .filter_map(|point| {
            let tile = grid.get(&point)?;
            Some((point, *tile, axis))
        })
        .collect_vec()
}

fn movements_for_axis(point: Point, axis: Axis) -> [Point; 2] {
    match axis {
        Axis::Vertical => [
            (point.0, point.1 - 1),
            (point.0, point.1 + 1),
        ],
        Axis::Horizontal => [
            (point.0 - 1, point.1),
            (point.0 + 1, point.1),
        ],
    }
}

fn parse_input(input: &str) -> (Grid, (Point, Tile, Axis)) {
    let mut grid = Grid::new();

    for (point, cell) in input.lines()
        .enumerate()
        .flat_map(|(y, row)| row.chars().enumerate().map(move |(x, cell)| ((x as isize, y as isize), cell)))
        .filter(|(_point, cell)| !cell.is_whitespace())
    {
        grid.insert(point, match cell {
            '|' => Tile::Vertical,
            '-' => Tile::Horizontal,
            '+' => Tile::Corner,
            c => {
                if c.is_ascii_uppercase() {
                    Tile::PointOfInterest(c)
                } else {
                    panic!("Invalid character: ({}, {}): {c}", point.0, point.1);
                }
            }
        });
    }

    let first_point = {
        let mut iter = grid.iter().filter(|((_x, y), _tile)| *y == 0).map(|((x, y), tile)| ((*x, *y), *tile));
        let first_point = iter.next().expect("No point found on top row");
        assert!(iter.next().is_none(), "More than one point on top row");
        first_point
    };

    (grid, (first_point.0, first_point.1, Axis::Vertical))
}