use std::collections::HashSet;
use itertools::repeat_n;
use regex::Regex;
use aoc_2019::{get_input, lcm};
use types::*;

const N: usize = 3;

fn main() {
    let system_3d = parse_input(&get_input(12));

    dbg!(part_1(system_3d.clone()));
    dbg!(part_2(system_3d));
}

fn part_1(mut system: Vec<Body<N>>) -> u32 {
    for _ in 0..1000 {
        system = tick(system);
    }
    calculate_total_energy(&system)
}

fn part_2(system: Vec<Body<N>>) -> usize {
    // We will simulate each dimension separately as they do not interact
    // Simulate until back in same positions & velocities
    // Then find the lowest common multiples of cycle time of each dimension

    let mut systems_1d = Vec::from_iter(repeat_n(vec![], N));
    for body in system {
        for i in 0..N {
            systems_1d[i].push(body.to_single_dimension(i))
        }
    }

    let mut dimension_repeat = vec![];
    for mut dimension in systems_1d {
        let mut seen = HashSet::new();
        for i in 0.. {
            dimension = tick(dimension);
            if seen.contains(&dimension) {
                dimension_repeat.push(i);
                break;
            }
            seen.insert(dimension.clone());
        }
    }
    dimension_repeat.into_iter().reduce(|a, b| lcm(a, b)).unwrap()
}

fn tick<const DIMENSIONS: usize>(system: Vec<Body<DIMENSIONS>>) -> Vec<Body<DIMENSIONS>> {
    let mut new_system = system.clone();

    for (offset, body) in system.iter().enumerate() {
        for other_body in system.iter() {
            if body == other_body {
                continue;
            }
            new_system[offset].apply_gravity(*other_body);
        }
        new_system[offset].apply_velocity();
    }

    new_system
}

fn calculate_total_energy<const DIMENSIONS: usize>(system: &[Body<DIMENSIONS>]) -> u32 {
    system.iter().map(|x| x.total_energy()).sum()
}

fn parse_input(input: &str) -> Vec<Body<N>> {
    let mut bodies = vec![];
    let re = Regex::new(r#"^<x=(?<x>-?\d+), y=(?<y>-?\d+), z=(?<z>-?\d+)>$"#).unwrap();
    for (line_number, line) in input.lines().enumerate() {
        let caps = re.captures(line).expect(&format!("Line {line_number} is not in expected format: {line}"));
        bodies.push(Body {
            position: Position::new([
                caps["x"].parse().unwrap(),
                caps["y"].parse().unwrap(),
                caps["z"].parse().unwrap(),
            ]),
            velocity: Velocity::default(),
        });
    }

    bodies
}

mod types {
    use aoc_lib::vector::Vector;

    pub type Position<const N: usize> = Vector<N>;
    pub type Velocity<const N: usize> = Vector<N>;

    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
    pub struct Body<const DIMENSIONS: usize> {
        pub position: Position<DIMENSIONS>,
        pub velocity: Velocity<DIMENSIONS>,
    }

    impl<const DIMENSIONS: usize> Body<DIMENSIONS> {
        pub fn to_single_dimension(&self, dimension: usize) -> Body<1> {
            Body {
                position: Position::new([self.position[dimension]]),
                velocity: Velocity::new([self.velocity[dimension]]),
            }
        }

        pub fn apply_gravity(&mut self, other_body: Self) {
            for i in 0..DIMENSIONS {
                self.velocity[i] -= (self.position[i] - other_body.position[i]).signum()
            }
        }

        pub fn apply_velocity(&mut self) {
            for i in 0..DIMENSIONS {
                self.position[i] += self.velocity[i];
            }
        }

        fn kinetic_energy(&self) -> u32 {
            self.velocity.iter().map(|x| x.unsigned_abs()).sum()
        }

        fn potential_energy(&self) -> u32 {
            self.position.iter().map(|x| x.unsigned_abs()).sum()
        }

        pub fn total_energy(&self) -> u32 {
            self.kinetic_energy() * self.potential_energy()
        }
    }
}

