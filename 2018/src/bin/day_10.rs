use std::collections::HashSet;

use aoc_2018::get_input;
use itertools::Itertools;
use lazy_regex::regex;

fn main() {
    let input = get_input(10);
    let particles = parse_input(&input);

    let (time_taken, message) = find_message(particles);
    println!("part_1:\n{}", message);
    println!("part_2: {}", time_taken);
}

fn find_message(mut particles: Vec<Particle>) -> (usize, String) {
    for i in 1..=100_000 {
        tick(&mut particles);
        if let Some((min_y, max_y)) = particles
            .iter()
            .map(|p| p.position.1)
            .minmax()
            .into_option()
        {
            if max_y - min_y <= 10 {
                return (i, make_sky_map(&particles));
            }
        }
    }
    unreachable!("No solution found after 100,000 iterations")
}

fn make_sky_map(particles: &[Particle]) -> String {
    let (min_x, max_x) = particles
        .iter()
        .map(|p| p.position.0)
        .minmax()
        .into_option()
        .unwrap();
    let (min_y, max_y) = particles
        .iter()
        .map(|p| p.position.1)
        .minmax()
        .into_option()
        .unwrap();

    let mut map = HashSet::new();

    for particle in particles {
        map.insert(particle.position);
    }

    ((min_y - 2)..(max_y + 2))
        .flat_map(|y| {
            ((min_x - 3)..=(max_x + 3))
                .map(|x| match map.get(&(x, y)) {
                    Some(_) => '#',
                    None => '.',
                })
                .chain(['\n'])
                .collect_vec()
        })
        .collect()
}

fn parse_input(input: &str) -> Vec<Particle> {
    let re = regex!(
        r#"(?x)^
        position=<
            \s?(?<pos_x>-?\d+),
            \s
            \s?(?<pos_y>-?\d+)
        >\s
        velocity=<
            \s?(?<vel_x>-?\d+),
            \s
            \s?(?<vel_y>-?\d+)
        >
    $"#
    );
    input
        .lines()
        .map(|line| {
            let captures = re.captures(line).unwrap();
            Particle {
                position: (
                    captures["pos_x"].trim().parse().unwrap(),
                    captures["pos_y"].trim().parse().unwrap(),
                ),
                velocity: (
                    captures["vel_x"].trim().parse().unwrap(),
                    captures["vel_y"].trim().parse().unwrap(),
                ),
            }
        })
        .collect_vec()
}

fn tick(particles: &mut [Particle]) {
    for particle in particles {
        particle.position.0 += particle.velocity.0;
        particle.position.1 += particle.velocity.1;
    }
}

#[derive(Debug, Copy, Clone)]
struct Particle {
    position: (isize, isize),
    velocity: (isize, isize),
}
