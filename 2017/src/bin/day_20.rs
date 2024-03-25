use std::collections::HashMap;

use aoc_2017::get_input;
use rayon::prelude::*;
use regex::{Captures, Regex};

const SIMULATION_ROUNDS: usize = 100_000;

fn main() {
    let input = get_input(20);
//     let input = r#"p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>
// p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>
// p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>
// p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>"#;
    let particles = parse_input(&input);
    
    dbg!(part_1(particles.clone()));
    dbg!(part_2(particles.clone()));
}

fn part_1(mut particles: HashMap<usize, Particle>) -> usize {
    particles.par_iter_mut().for_each(|(_p_num, p)| p.tick_n(SIMULATION_ROUNDS));

    particles
        .into_iter()
        .min_by_key(|(_particle_num, particle)| particle.manhattan_distance_from_origin())
        .unwrap()
        .0
}

fn part_2(mut particles: HashMap<usize, Particle>) -> usize {
    for _ in 0..SIMULATION_ROUNDS {
        let mut positions = HashMap::<Vec3d, Vec<usize>>::new();
        
        for (particle_num, particle) in particles.iter_mut() {
            particle.tick();
            positions.entry(particle.position).or_insert(Vec::with_capacity(2)).push(*particle_num);
        }

        for (_pos, particle_nums) in positions.into_iter().filter(|x| x.1.len() > 1) {
            particle_nums.into_iter().for_each(|x| { particles.remove(&x); });
        }
    }
    particles.len()
}

fn parse_input(input: &str) -> HashMap<usize, Particle> {
    let re = Regex::new(r#"(?x)^
        p=<(?<x_pos>-?\d+),(?<y_pos>-?\d+),(?<z_pos>-?\d+)>,\s
        v=<(?<x_vel>-?\d+),(?<y_vel>-?\d+),(?<z_vel>-?\d+)>,\s
        a=<(?<x_acc>-?\d+),(?<y_acc>-?\d+),(?<z_acc>-?\d+)>
    $"#).unwrap();
    
    fn extract(capture: &Captures<'_>, s: &str) -> isize {
        capture[s].parse().unwrap()
    }

    input.lines().enumerate().map(|(line_num, s)| {
        let c = &re.captures(s).expect("Line did not match regex");
        (line_num, Particle {
            position: Vec3d { x: extract(c, "x_pos"), y: extract(c, "y_pos"), z: extract(c, "z_pos") },
            velocity: Vec3d { x: extract(c, "x_vel"), y: extract(c, "y_vel"), z: extract(c, "z_vel") },
            acceleration: Vec3d { x: extract(c, "x_acc"), y: extract(c, "y_acc"), z: extract(c, "z_acc") },
        })
    }).collect()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Particle {
    position: Vec3d,
    velocity: Vec3d,
    acceleration: Vec3d,
}

impl Particle {
    fn tick(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
    }

    fn tick_n(&mut self, n: usize) {
        for _ in 0..n {
            self.velocity += self.acceleration;
            self.position += self.velocity;    
        }
    }

    fn manhattan_distance_from_origin(&self) -> usize {
        (self.position.x.abs() +
        self.position.y.abs() + 
        self.position.z.abs()) as usize
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Vec3d {
    x: isize,
    y: isize,
    z: isize,
}

impl std::ops::AddAssign for Vec3d {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

