use std::{collections::{BTreeMap, BTreeSet}, fmt};

use aoc_2016::get_input;
use itertools::Itertools;
use pathfinding::directed::dijkstra::dijkstra;

fn main() {
    let input = get_input(11);

    let state = State {
        current_floor: 1,
        floors: parse_input(&input),
    };

    dbg!(part_1(&state));
    dbg!(part_2(&state));
}

fn part_1(initial_state: &State) -> usize {
    let Some(x) = dijkstra(
        initial_state,
        |x| next_state(x),
        |x| x.solved()
    ) else {
        panic!("No solution found!");
    };

    x.1
}

fn part_2(initial_state: &State) -> usize {
    let mut initial_state = initial_state.clone();
    initial_state.floors.entry(1).and_modify(|x| {
        x.insert(Device::Generator("Elerium".to_string()));
        x.insert(Device::Microchip("Elerium".to_string()));
        x.insert(Device::Generator("Dilithium".to_string()));
        x.insert(Device::Microchip("Dilithium".to_string()));
    });

    part_1(&initial_state)
}

fn next_state(state: &State) -> Vec<(State, usize)> {
    let mut out = vec![];

    let items = &state.floors[&state.current_floor];

    for items_to_move in items
        .iter()
        .powerset()
        .filter(|x| [1, 2].contains(&x.len()))
        .map(|x| BTreeSet::<Device>::from_iter(x.into_iter().cloned())
    ) {
        let mut new_current_floor = items.clone();

        for item in items_to_move.iter() {
            new_current_floor.remove(item);
        }
        if !is_combination_safe(&new_current_floor) ||
            !is_combination_safe(&items_to_move)
        {
            continue;
        }

        for next_floor_num in state.next_movements() {
            let mut next_floor = state.floors.get(&next_floor_num).unwrap().clone();
            next_floor.extend(items_to_move.clone());
            if !is_combination_safe(&next_floor) {
                continue;
            }
            let mut new_state = state.clone();
            new_state.floors.insert(state.current_floor, new_current_floor.clone());
            new_state.floors.insert(next_floor_num, next_floor);
            new_state.current_floor = next_floor_num;
            out.push((new_state, 1));
        }
    }

    out
}

fn is_combination_safe(combination: &BTreeSet<Device>) -> bool {
    let combination = combination.iter().cloned().collect_vec();
    combination.iter().all(|x| x.is_compatible(&combination))
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct State {
    floors: BTreeMap<usize, BTreeSet<Device>>,
    current_floor: usize,
}

impl State {
    fn solved(&self) -> bool {
        self
            .floors
            .iter()
            .filter(|(floor_num, _devices)| **floor_num != self.floors.len())
            .map(|(_floor_num, devices)| devices.len())
            .sum::<usize>() == 0
    }

    fn next_movements(&self) -> Vec<usize> {
        let mut next_floor_nums = vec![];
        if self.current_floor > 1 && self.current_floor <= 4 {
            next_floor_nums.push(self.current_floor - 1)
        }
        if self.current_floor < 4 {
            next_floor_nums.push(self.current_floor + 1);
        }
        next_floor_nums
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (floor_num, devices) in self.floors.iter().rev() {
            writeln!(f,
                "F{floor_num} {} {}",
                if self.current_floor == *floor_num {'E'} else {' '},
                devices.iter().map(|x| {
                    match x {
                        Device::Generator(x) => format!("{}G", x.chars().next().unwrap().to_uppercase()),
                        Device::Microchip(x) => format!("{}M", x.chars().next().unwrap().to_uppercase()),
                    }
                }).join(" "),
            )?
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum Device {
    Generator(String),
    Microchip(String),
}

impl Device {
    fn is_generator(&self) -> bool {
        matches!(self, Self::Generator(_))
    }

    fn radiotype<'a>(&'a self) -> &'a str {
        match self {
            Device::Generator(x) => &x,
            Device::Microchip(x) => &x,
        }
    }

    fn is_compatible(&self, other_items: &[Device]) -> bool {
        let Device::Microchip(radiotype) = self else {
            return true;
        };

        if other_items.iter().any(|x: &Device| {
            x.is_generator() && x.radiotype() == radiotype
        }) {
            return true;
        }

        if other_items.iter().any(|x: &Device| {
            x.is_generator() && x.radiotype() != radiotype
        }) {
            return false;
        }
        return true;
    }
}


fn parse_input(input: &str) -> BTreeMap<usize, BTreeSet<Device>> {
    let mut map = BTreeMap::new();
    for line in input.lines() {
        let (floor, items) = parse_line(line);
        map.insert(floor, items);
    }
    map
}

fn parse_line(line: &str) -> (usize, BTreeSet<Device>) {
    let (floor_info, items) = line.trim_end_matches(".").split(" contains ").collect_tuple().unwrap();
    let floor_info = floor_info.trim_start_matches("The ").trim_end_matches(" floor");
    let floor_info = match floor_info {
        "first" => 1,
        "second" => 2,
        "third" => 3,
        "fourth" => 4,
        _ => panic!("Invalid floor information"),
    };

    if items == "nothing relevant" {
        return (floor_info, BTreeSet::new());
    }

    let items = items
        .split(", ")
        .map(|s| s.split("and "))
        .flatten()
        .filter(|s| !s.is_empty())
        .map(|s| s.trim_start_matches("a "))
        .map(|s| s.trim())
        .map(|item| {
            let (radiotype, item_type) = item.split(" ").collect_tuple().unwrap();
            let radiotype = radiotype.split("-").next().unwrap().to_string();
            match item_type {
                "microchip" => Device::Microchip(radiotype),
                "generator" => Device::Generator(radiotype),
                _ => panic!("Invalid item type: {item_type}"),
            }
        })
        .collect();

    (floor_info, items)
}