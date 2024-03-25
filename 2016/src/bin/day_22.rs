use std::collections::HashMap;

use aoc_2016::get_input;
use itertools::Itertools;
use pathfinding::directed::dijkstra::dijkstra;
use regex::Regex;

fn main() {
    let input = get_input(22);
    let map = parse_input(&input);

    dbg!(part_1(&map));
    dbg!(part_2(&map));
}

fn part_1(map: &Map) -> usize {
    map.viable_pairs().len()
}

fn part_2(map: &Map) -> usize {
    let state = State { goal_data: (map.width - 1, 0), blank: map.find_blank() };
    dijkstra(
        &state,
        |x| next_states(x, map).into_iter().map(|x| (x, 1)).collect_vec(),
        |x| x.goal_data == (0, 0)
    ).expect("No solution found").1
}

#[derive(Debug)]
struct Map {
    inner: HashMap<(usize, usize), Node>,
    width: usize,
    height: usize,
}

impl Map {
    fn viable_pairs(&self) -> Vec<((usize, usize), (usize, usize))> {
        self.inner.iter().permutations(2).filter_map(|x| {
            let (a_pos, a) = x.first().unwrap();
            let (b_pos, b) = x.last().unwrap();
            if (!a.is_empty()) && a.used <= b.available {
                return Some((**a_pos, **b_pos));
            }
            None
        }).collect_vec()
    }

    fn neighbours(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let mut out = vec![];
        if x < self.width - 1 {
            out.push((x + 1, y))
        }
        if x > 0 {
            out.push((x - 1, y))
        }
        if y < self.height - 1 {
            out.push((x, y + 1))
        }
        if y > 0 {
            out.push((x, y - 1))
        }

        out
    }

    fn find_blank(&self) -> (usize, usize) {
        self.inner
            .iter()
            .find(|(_pos, node)| node.used == 0)
            .map(|(pos, _node)| *pos)
            .unwrap()
    }

    fn is_viable(&self, point: (usize, usize)) -> bool {
        for (x, y) in self.neighbours(point) {
            if self.inner[&point].used > self.inner[&(x, y)].used + self.inner[&(x, y)].available {
                return false;
            }
        }
        true
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct State {
    goal_data: (usize, usize),
    blank: (usize, usize),
}

fn next_states(state: &State, map: &Map) -> Vec<State> {
    let mut out = vec![];
    for next_blank in map.neighbours(state.blank) {
        if !map.is_viable(next_blank) {
            continue;
        }
        let mut next_goal = state.goal_data;
        if next_blank == state.goal_data {
            next_goal = state.blank;
        }
        out.push(State {
            goal_data: next_goal,
            blank: next_blank,
        })
    }
    out
}

fn parse_input(input: &str) -> Map {
    let mut lines = input.lines().enumerate();
    lines.next();
    lines.next();

    let mut map: HashMap<(usize, usize), Node> = HashMap::new();
    let re = Regex::new(r#"\/dev\/grid\/node-x(?<x>\d+)-y(?<y>\d+)\s+\d+T\s+(?<used>\d+)T\s+(?<available>\d+)T\s+(\d+)%"#).unwrap();
    for (line_num, line) in lines {
        let capture = re.captures(line).unwrap_or_else(|| panic!("Invalid line: line {line_num}"));
        let x = capture.name("x").unwrap().as_str().parse().unwrap();
        let y = capture.name("y").unwrap().as_str().parse().unwrap();
        let used = capture.name("used").unwrap().as_str().parse().unwrap();
        let available = capture.name("available").unwrap().as_str().parse().unwrap();
        map.insert((x, y), Node {
            used, available
        });
    }

    Map {
        width: *map.keys().map(|(x, _y)| x).max().unwrap() + 1,
        height: *map.keys().map(|(_x, y)| y).max().unwrap() + 1,
        inner: map,
    }
}

#[derive(Clone, Copy)]
#[derive(Debug)]
struct Node {
    used: usize,
    available: usize,
}

impl Node {
    fn is_empty(&self) -> bool {
        self.used == 0
    }
}