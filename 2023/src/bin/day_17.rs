use std::collections::HashMap;

use aoc_2023::get_input;
use pathfinding::directed::dijkstra::dijkstra;

struct Grid {
    inner: HashMap<(isize, isize), u32>,
    x_len: usize,
    y_len: usize,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turns(&self) -> [Direction; 2] {
        match self {
            Direction::Up | Direction::Down => [Direction::Right, Direction::Left],
            Direction::Left | Direction::Right => [Direction::Up, Direction::Down],
        }
    }
}

impl Grid {
    // fn coord_in_limits(&self, x: isize, y: isize) -> bool {
    //     x >= 0 && x < self.x_len as isize &&
    //     y >= 0 && y < self.y_len as isize
    // }

    fn moves(
        &self,
        min: isize,
        max: isize,
        x: isize,
        y: isize,
        direction: &Direction,
    ) -> Vec<(((isize, isize), Direction), u32)> {
        let mut out = vec![];
        for dir in direction.turns() {
            let mut total_cost = 0u32;
            for i in 1..=max {
                let new_pos = match dir {
                    Direction::Up => (x, y - i),
                    Direction::Down => (x, y + i),
                    Direction::Left => (x - i, y),
                    Direction::Right => (x + i, y),
                };
                if let Some(cost) = self.inner.get(&new_pos) {
                    total_cost += cost;
                    if i >= min {
                        out.push(((new_pos, dir.clone()), total_cost));
                    }
                }
            }
        }
        out
    }
}

fn main() {
    let input = get_input(17);

    let mut grid = Grid {
        inner: HashMap::new(),
        x_len: input.lines().next().unwrap().len(),
        y_len: input.lines().count(),
    };

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.inner
                .insert((x as isize, y as isize), c.to_digit(10).unwrap());
        }
    }

    let part_a = get_answer(&grid, 1, 3);
    dbg!(part_a);
    let part_b = get_answer(&grid, 4, 10);
    dbg!(part_b);
}

fn get_answer(grid: &Grid, min: isize, max: isize) -> u32 {
    let result_down = dijkstra(
        &((0isize, 0isize), Direction::Down),
        |((x, y), direction)| grid.moves(min, max, *x, *y, direction),
        |((x, y), _)| *x as usize == &grid.x_len - 1 && *y as usize == &grid.y_len - 1,
    )
    .unwrap();
    let result_right = dijkstra(
        &((0isize, 0isize), Direction::Right),
        |((x, y), direction)| grid.moves(min, max, *x, *y, direction),
        |((x, y), _)| *x as usize == &grid.x_len - 1 && *y as usize == &grid.y_len - 1,
    )
    .unwrap();

    if result_down.1 > result_right.1 {
        result_right.1
    } else {
        result_down.1
    }
}
