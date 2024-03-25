use std::collections::HashMap;

use aoc_2023::get_input;
use itertools::Itertools;

#[derive(Debug)]
struct Vec3d {
    x: u32,
    y: u32,
    z: u32,
}

#[derive(Debug)]
struct Brick {
    name: String,
    start: Vec3d,
    end: Vec3d,
}

impl Brick {
    fn get_top_faces(&self) -> Vec<Vec3d> {
        let mut out = vec![];
        for x in self.start.x.min(self.end.x)..=self.start.x.max(self.end.x) {
            for y in self.start.y.min(self.end.y)..=self.start.y.max(self.end.y) {
                out.push(Vec3d { x, y, z: self.start.z.max(self.end.z) })
            }
        }
        out
    }

    fn down(&self, amount: u32) -> Self {
        Self {
            name: self.name.clone(),
            start: Vec3d {
                x: self.start.x,
                y: self.start.y,
                z: self.start.z - amount,
            },
            end: Vec3d {
                x: self.end.x,
                y: self.end.y,
                z: self.end.z - amount,
            },
        }
    }

    fn get_lowest_z(&self) -> u32 {
        self.start.z.min(self.end.z)
    }
}

fn main() {
    let input = get_input(22);
    let mut bricks = parse_input(&input);
    dbg!(&bricks);

    let mut heightmap: HashMap<(u32, u32), u32> = HashMap::new();

    for brick in bricks.iter_mut() {
        let mut min_height = 1;

        for Vec3d {x, y, z: _} in brick.get_top_faces() {
            min_height = min_height.max(
                *heightmap.get(&(x, y)).unwrap_or(&1)
            );
        }

        *brick = brick.down(brick.get_lowest_z() - min_height);

        for Vec3d { x, y, z } in brick.get_top_faces() {
            heightmap.entry((x, y)).and_modify(|h| *h = (*h).max(z + 1)).or_insert(z + 1);
        }
    }

    dbg!(bricks);
}

fn parse_input(input: &str) -> Vec<Brick> {
    let mut bricks: Vec<Brick> = vec![];
    let names = ["A", "B", "C", "D", "E", "F", "G"];

    for (idx, line) in input.lines().enumerate() {
        let (start, end) = line.split("~").map(|a| {
            let (x, y, z) = a.split(",").map(|b| b.parse::<u32>().unwrap()).collect_tuple().unwrap();
            Vec3d { x, y, z }
        }).collect_tuple().unwrap();
        bricks.push(Brick { name: names[idx].to_owned(), start, end });
    }

    bricks.sort_by(|brick_a, brick_b| {
        brick_a.start.z.min(brick_a.end.z).cmp(&brick_b.start.z.min(brick_b.end.z))
    });

    bricks
}