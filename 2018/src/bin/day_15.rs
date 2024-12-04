#![feature(let_chains)]

use std::collections::{HashMap, HashSet};

use aoc_2018::get_input;
use itertools::Itertools;
use pathfinding::directed::dijkstra::dijkstra;

fn main() {
    let input = get_input(15);
    let input = "#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";

    let mut game = parse_game_from_string(input);
    let mut iterations = 0usize;

    for i in 0.. {
        let fighters = game
            .elves
            .iter()
            .chain(game.goblins.iter())
            .map(|(coord, fighter)| (*coord, fighter.class))
            .sorted_by_key(|(coord, _class)| (coord.y, coord.x))
            .collect_vec();

        for (mut coord, fighter_class) in fighters.into_iter() {
            let enemy = game
                .neighbours(coord)
                .into_iter()
                .find(|(_c, s)| matches!(s, Square::Fighter(x) if *x == fighter_class.enemy()));

            if enemy.is_none()
                && let Some((_target, next_move, _path_len)) =
                    game.next_move(coord, game.get_fighters(fighter_class.enemy()).iter())
            {
                let map = game.get_fighters_mut(fighter_class);
                let fighter = map.remove(&coord).unwrap();
                map.insert(next_move, fighter);
                coord = next_move;
            };

            if let Some((enemy_coord, _square)) = enemy {
                // fight instead of move
                let Some(Fighter {
                    damage: our_damage, ..
                }) = game.get_fighters(fighter_class).get(&coord)
                else {
                    // we died on a previous iteration
                    continue;
                };
                let our_damage = *our_damage;
                let enemy = game
                    .get_fighters_mut(fighter_class.enemy())
                    .get_mut(&enemy_coord)
                    .unwrap();

                enemy.health_points = enemy.health_points.saturating_sub(our_damage);
                game.clean_dead();

                continue;
            }
        }

        if game.is_finished() {
            game.clean_dead();
            iterations = i;
            break;
        }
    }
    for (_, elf) in game.elves.iter() {
        dbg!(elf.health_points);
    }
    for (_, goblin) in game.goblins.iter() {
        dbg!(goblin.health_points);
    }
    let score = iterations
        * game
            .elves
            .iter()
            .chain(game.goblins.iter())
            .map(|(_coord, fighter)| fighter.health_points)
            .sum::<usize>();
    dbg!(score);
}

#[derive(Debug)]
struct Fighter {
    health_points: usize,
    damage: usize,
    class: FighterClass,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum FighterClass {
    Elf,
    Goblin,
}

impl FighterClass {
    fn enemy(&self) -> FighterClass {
        use FighterClass::*;
        match self {
            Elf => Goblin,
            Goblin => Elf,
        }
    }
}

impl Fighter {
    fn new(class: FighterClass) -> Self {
        Fighter {
            health_points: 200,
            damage: 3,
            class,
        }
    }
}

impl Fighter {
    fn is_alive(&self) -> bool {
        self.health_points > 0
    }

    fn attack(&self, other: &mut Fighter) {
        other.health_points = other.health_points.saturating_sub(self.health_points);
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Coord {
    x: isize,
    y: isize,
}

impl From<(usize, usize)> for Coord {
    fn from((x, y): (usize, usize)) -> Self {
        Self {
            x: x as isize,
            y: y as isize,
        }
    }
}

impl From<(isize, isize)> for Coord {
    fn from((x, y): (isize, isize)) -> Self {
        Self { x, y }
    }
}

impl Coord {
    fn up(&self) -> Coord {
        Coord {
            x: self.x,
            y: self.y - 1,
        }
    }
    fn down(&self) -> Coord {
        Coord {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn left(&self) -> Coord {
        Coord {
            x: self.x - 1,
            y: self.y,
        }
    }
    fn right(&self) -> Coord {
        Coord {
            x: self.x + 1,
            y: self.y,
        }
    }
}

#[derive(Default)]
struct Game {
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    walls: HashSet<Coord>,
    elves: HashMap<Coord, Fighter>,
    goblins: HashMap<Coord, Fighter>,
}

impl Game {
    fn is_square_empty(&self, coord: Coord) -> bool {
        !(self.walls.contains(&coord)
            && self.elves.contains_key(&coord)
            && self.goblins.contains_key(&coord))
    }

    fn coord_in_range(&self, coord: Coord) -> bool {
        coord.x >= self.min_x
            && coord.x <= self.max_x
            && coord.y >= self.min_y
            && coord.y <= self.max_y
    }

    fn get(&self, coord: Coord) -> Option<Square> {
        if !self.coord_in_range(coord) {
            return None;
        } else if self.walls.contains(&coord) {
            return Some(Square::Wall);
        } else if self.elves.contains_key(&coord) {
            return Some(Square::Fighter(FighterClass::Elf));
        } else if self.goblins.contains_key(&coord) {
            return Some(Square::Fighter(FighterClass::Goblin));
        }
        Some(Square::Empty)
    }

    fn neighbours(&self, coord: Coord) -> Vec<(Coord, Square)> {
        // order is important - reading order is the tie breaker
        [coord.up(), coord.left(), coord.right(), coord.down()]
            .into_iter()
            .map(|x| (x, self.get(x)))
            .filter(|(_x, s)| s.is_some())
            .map(|(x, s)| (x, s.unwrap()))
            .collect_vec()
    }

    fn empty_neighbours(&self, coord: Coord) -> Vec<(Coord, Square)> {
        self.neighbours(coord)
            .into_iter()
            .filter(|(_c, s)| s.can_occupy())
            .collect_vec()
    }

    fn next_move<'a>(
        &'a self,
        from: Coord,
        enemy_fighters: impl Iterator<Item = (&'a Coord, &'a Fighter)>,
    ) -> Option<(Coord, Coord, usize)> {
        enemy_fighters
            .flat_map(|(coord, _fighter)| {
                // Find empty squares around each target
                self.empty_neighbours(*coord)
                    .into_iter()
                    .map(|(coord, _square)| (coord))
            })
            // calculate path (djikstra) from 'from' to 'to'
            .map(|coord| (coord, self.find_path(from, coord)))
            // filter out unreachable targets
            .filter(|(_coord, path)| path.is_some())
            .map(|(coord, path)| (coord, path.unwrap()))
            // get path length then take first step in path
            .map(|(coord, path)| (coord, path[1], path.len()))
            .sorted_by_key(|(_target, next_move, path_len)| {
                // shortest path, ties filtered by reading order (L->R, T->B)
                (*path_len, next_move.y, next_move.x)
            })
            .next()
    }

    fn find_path(&self, from: Coord, to: Coord) -> Option<Vec<Coord>> {
        dijkstra(
            &from,
            |x| self.empty_neighbours(*x).into_iter().map(|(c, _s)| (c, 1)),
            |c| *c == to,
        )
        .map(|(path, _len)| path)
    }

    fn get_fighters(&self, class: FighterClass) -> &HashMap<Coord, Fighter> {
        use FighterClass::*;
        match class {
            Elf => &self.elves,
            Goblin => &self.goblins,
        }
    }

    fn get_fighters_mut(&mut self, class: FighterClass) -> &mut HashMap<Coord, Fighter> {
        use FighterClass::*;
        match class {
            Elf => &mut self.elves,
            Goblin => &mut self.goblins,
        }
    }

    fn print(&self) {
        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                if self.walls.contains(&(x, y).into()) {
                    print!("#");
                } else if self.goblins.contains_key(&(x, y).into()) {
                    print!("G");
                } else if self.elves.contains_key(&(x, y).into()) {
                    print!("E");
                } else {
                    print!(".");
                }
            }
            println!()
        }
    }

    fn is_finished(&self) -> bool {
        self.elves.is_empty() || self.goblins.is_empty()
    }

    fn clean_dead(&mut self) {
        self.elves.retain(|_coord, fighter| fighter.is_alive());
        self.goblins.retain(|_coord, fighter| fighter.is_alive());
    }
}

enum Square {
    Fighter(FighterClass),
    Wall,
    Empty,
}

impl Square {
    fn can_occupy(&self) -> bool {
        matches!(self, Square::Empty)
    }
}

fn parse_game_from_string(input: &str) -> Game {
    let mut map = Game {
        max_x: input.lines().next().unwrap().len() as isize - 1,
        max_y: input.lines().count() as isize - 1,
        ..Default::default()
    };

    for (y, line) in input.lines().enumerate() {
        for (x, square) in line.chars().enumerate() {
            match square {
                'E' => {
                    map.elves
                        .insert((x, y).into(), Fighter::new(FighterClass::Elf));
                }
                'G' => {
                    map.goblins
                        .insert((x, y).into(), Fighter::new(FighterClass::Goblin));
                }
                '#' => {
                    map.walls.insert((x, y).into());
                }
                '.' => {}
                _ => panic!("Invalid character: {square} at {y}:{x}"),
            };
        }
    }

    map
}
