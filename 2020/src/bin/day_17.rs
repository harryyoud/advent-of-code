use aoc_2020::get_input;
use itertools::Itertools;
use std::collections::HashSet;
use types::*;

fn main() {
    let input = get_input(17);

    let mut grid_3d = HashSet::new();
    let mut grid_4d = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, cha) in line.chars().enumerate() {
            match cha {
                '.' => {},
                '#' => {
                    grid_3d.insert(Vector([x as i32, y as i32, 0_i32]));
                    grid_4d.insert(Vector([x as i32, y as i32, 0_i32, 0i32]));
                },
                _ => panic!("Invalid character: {cha} at line {y} char {x}"),
            };
        }
    }

    dbg!(part_1(grid_3d));
    dbg!(part_2(grid_4d));
}

fn part_1(grid: HashSet<Vector<3>>) -> usize {
    run_simulation(grid).len()
}

fn part_2(grid: HashSet<Vector<4>>) -> usize {
    run_simulation(grid).len()
}

fn run_simulation<const DIMENSIONS: usize>(
    mut grid: HashSet<Vector<DIMENSIONS>>
) -> HashSet<Vector<DIMENSIONS>> {
    for _ in 0..6 {
        let mut new_grid = HashSet::new();
        for point in limits_iter(&grid) {
            match (grid.contains(&point), active_neighbours(&grid, point)) {
                (true, 2..=3) => { new_grid.insert(point); },
                (false, 3) => { new_grid.insert(point); },
                (_, _) => {},
            }
        }
        grid = new_grid;
    }
    grid
}

fn neighbours<const DIMENSIONS: usize>(point: Vector<DIMENSIONS>) -> Vec<Vector<DIMENSIONS>> {
    point
        .into_iter()
        .map(|d| (d - 1)..=(d +1))
        .multi_cartesian_product()
        .map(|p| Vector(p.try_into().unwrap()))
        .filter(|v| *v != point)
        .collect_vec()
}

fn active_neighbours<const DIMENSIONS: usize>(
    grid: &HashSet<Vector<DIMENSIONS>>,
    point: Vector<DIMENSIONS>
) -> usize {
    neighbours(point).into_iter()
        .filter(|v| grid.contains(v))
        .count()
}

fn limits_iter<const DIMENSIONS: usize>(
    grid: &HashSet<Vector<DIMENSIONS>>
) -> impl Iterator<Item = Vector<DIMENSIONS>> {
    (0..DIMENSIONS).into_iter()
        .map(|dim| {
            match grid.iter().map(|p| p[dim]).minmax().into_option() {
                None => panic!("Empty grid!"),
                Some((a, b)) => (a - 1)..=(b + 1)
            }
        })
        .multi_cartesian_product()
        .map(|v| Vector(v.try_into().unwrap()))
}

mod types {
    use std::ops::Deref;
    use std::ops::DerefMut;

    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
    pub struct Vector<const DIMENSIONS: usize>(pub [i32; DIMENSIONS]);

    impl<const DIMENSIONS: usize> Deref for Vector<DIMENSIONS> {
        type Target = [i32; DIMENSIONS];

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<const DIMENSIONS: usize> DerefMut for Vector<DIMENSIONS> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl<const DIMENSIONS: usize> Default for Vector<DIMENSIONS> {
        fn default() -> Self {
            Vector([0; DIMENSIONS])
        }
    }
}