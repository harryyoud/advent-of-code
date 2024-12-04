use aoc_2023::get_input;
use itertools::Itertools;

// const LOWER_LIMIT: f64 =  7.0;
// const UPPER_LIMIT: f64 = 27.0;
const LOWER_LIMIT: f64 = 200_000_000_000_000.0;
const UPPER_LIMIT: f64 = 400_000_000_000_000.0;

#[derive(Debug, Clone)]
struct LinearPath {
    vertical_offset: f64,
    gradient: f64,
}

#[allow(dead_code)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

#[allow(dead_code)]
struct Velocity {
    x: f64,
    y: f64,
    z: f64,
}

struct Hailstone {
    point: Point,
    velocity: Velocity,
    path: LinearPath,
}

enum Outcome {
    PastCollisionBoth(Point),
    PastCollisionA(Point),
    PastCollisionB(Point),
    FutureCollisionInTestArea(Point),
    FutureCollisionOutsideTestArea(Point),
    NoCollisionPossible,
}

impl LinearPath {
    fn calculate_intersection_2d(&self, other: &Self) -> Option<Point> {
        if self.gradient == other.gradient {
            // parallel lines
            return None;
        };

        // y == m1x + c1
        // m2x + c2 == m1x + c1
        // m2x - m1x = c1 - c2
        // (m2 - m1)x == c1 - c2
        // x == (c1 - c2)/(m2 - m1)
        let x = (self.vertical_offset - other.vertical_offset) / (other.gradient - self.gradient);

        // y = mx + c
        let y = (self.gradient * x) + self.vertical_offset;

        // would be nice to assert here that y is equal in both equations, but cannot compare floats

        Some(Point { x, y, z: 0.0 })
    }
}

impl Hailstone {
    fn is_intersection_in_future(&self, intersection: &Point) -> bool {
        match (
            self.velocity.y.is_sign_negative(),
            intersection.y < self.point.y,
        ) {
            (true, true) => true,   // heading toward intersection
            (true, false) => false, // heading away from intersection
            (false, true) => false, // heading away from intersection
            (false, false) => true, // heading toward intersection
        }
    }

    fn calculate_intersection_2d(&self, other: &Self) -> Option<Point> {
        self.path.calculate_intersection_2d(&other.path)
    }
}

fn main() {
    let input = get_input(24);
    let paths = input.lines().map(parse_path).collect::<Vec<Hailstone>>();
    let mut total_possible_collisions = 0u32;

    for (hailstone_a, hailstone_b) in paths
        .iter()
        .tuple_combinations::<(&Hailstone, &Hailstone)>()
    {
        println!(
            "Hailstone A: {}, {} @ {}, {}",
            hailstone_a.point.x,
            hailstone_a.point.y,
            hailstone_a.velocity.x,
            hailstone_a.velocity.y
        );
        println!(
            "Hailstone B: {}, {} @ {}, {}",
            hailstone_b.point.x,
            hailstone_b.point.y,
            hailstone_b.velocity.x,
            hailstone_b.velocity.y
        );
        match get_outcome(hailstone_a, hailstone_b) {
            Outcome::PastCollisionBoth(intersection) => println!(
                "Hailstones' paths crossed in the past for both hailstones (at x={}, y={})",
                intersection.x, intersection.y
            ),
            Outcome::PastCollisionA(intersection) => println!(
                "Hailstones' paths crossed in the past for hailstone A (at x={}, y={})",
                intersection.x, intersection.y
            ),
            Outcome::PastCollisionB(intersection) => println!(
                "Hailstones' paths crossed in the past for hailstone B (at x={}, y={})",
                intersection.x, intersection.y
            ),
            Outcome::FutureCollisionOutsideTestArea(intersection) => println!(
                "Hailstones' paths will cross outside the test area (at x={}, y={})",
                intersection.x, intersection.y
            ),
            Outcome::NoCollisionPossible => {
                println!("Hailstones' paths are parallel; they never intersect.")
            }
            Outcome::FutureCollisionInTestArea(intersection) => {
                println!(
                    "Hailstones' paths will cross inside the test area (at x={}, y={})",
                    intersection.x, intersection.y
                );
                total_possible_collisions += 1;
            }
        }
        println!(
            "y = {}x + {}",
            hailstone_a.path.gradient, hailstone_a.path.vertical_offset
        );
        println!("({}, {})", hailstone_a.point.x, hailstone_a.point.y);
        println!(
            "y = {}x + {}",
            hailstone_b.path.gradient, hailstone_b.path.vertical_offset
        );
        println!("({}, {})", hailstone_b.point.x, hailstone_b.point.y);
        println!("x={}", LOWER_LIMIT);
        println!("x={}", UPPER_LIMIT);
        println!();
    }

    dbg!(total_possible_collisions);
}

fn parse_path(line: &str) -> Hailstone {
    let (positions, velocities) = line.split(" @ ").collect_tuple().unwrap();
    let (position_x, position_y, position_z) = positions
        .trim()
        .split(", ")
        .map(|s| s.trim().parse::<f64>().unwrap())
        .collect_tuple()
        .unwrap();
    let (velocity_x, velocity_y, velocity_z) = velocities
        .trim()
        .split(", ")
        .map(|s| s.trim().parse::<f64>().unwrap())
        .collect_tuple()
        .unwrap();

    let gradient = velocity_y / velocity_x;
    let vertical_offset = position_y - (gradient * position_x);

    Hailstone {
        point: Point {
            x: position_x,
            y: position_y,
            z: position_z,
        },
        path: LinearPath {
            vertical_offset,
            gradient,
        },
        velocity: Velocity {
            x: velocity_x,
            y: velocity_y,
            z: velocity_z,
        },
    }
}

fn get_outcome(hailstone_a: &Hailstone, hailstone_b: &Hailstone) -> Outcome {
    if let Some(intersection) = hailstone_a.calculate_intersection_2d(hailstone_b) {
        let hailstone_a_future = hailstone_a.is_intersection_in_future(&intersection);
        let hailstone_b_future = hailstone_b.is_intersection_in_future(&intersection);
        match (hailstone_a_future, hailstone_b_future) {
            (false, false) => return Outcome::PastCollisionBoth(intersection),
            (true, false) => return Outcome::PastCollisionB(intersection),
            (false, true) => return Outcome::PastCollisionA(intersection),
            (true, true) => (),
        };
        if intersection.x > UPPER_LIMIT
            || intersection.x < LOWER_LIMIT
            || intersection.y > UPPER_LIMIT
            || intersection.y < LOWER_LIMIT
        {
            // outside of test area
            return Outcome::FutureCollisionOutsideTestArea(intersection);
        }
        return Outcome::FutureCollisionInTestArea(intersection);
    }
    Outcome::NoCollisionPossible
}
