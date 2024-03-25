use std::collections::HashMap;

use aoc_2016::get_input;
use regex::Regex;

fn main() {
    let input = get_input(15);
    let discs = parse_input(&input);

    dbg!(part_1(&discs));
    dbg!(part_2(&discs));
}

fn part_1(discs: &HashMap<u64, Disc>) -> u64 {
    is_possible_to_pass(discs)
}

fn part_2(discs: &HashMap<u64, Disc>) -> u64 {
    let mut discs = discs.clone();
    discs.insert(discs.len() as u64 + 1, Disc {
        positions: 11,
        current_position: 0,
    });
    is_possible_to_pass(&discs)
}

fn is_possible_to_pass(discs: &HashMap<u64, Disc>) -> u64 {
    let mut discs = discs.clone();
    discs.iter_mut().for_each(|(disc_num, disc)| disc.advance_by(*disc_num));

    for i in 0.. {
        if discs.values().all(|x| x.current_position == 0) {
            return i;
        }
        discs.iter_mut().for_each(|(_disc_num, disc)| disc.advance());
    }

    unreachable!("No solution found")
}

fn parse_input(input: &str) -> HashMap<u64, Disc> {
    let re = Regex::new(r#"Disc #(?<disc_number>\d+) has (?<positions>\d+) positions; at time=0, it is at position (?<starting_position>\d+)"#).unwrap();
    let mut map = HashMap::new();
    for (_, line) in input.lines().enumerate() {
        let captures = re.captures(line).expect("Could not extract information at line {i}");
        map.insert(captures["disc_number"].parse().expect("Could not parse disc number"), Disc {
            positions: captures["positions"].parse().expect("Could not parse positions"),
            current_position: captures["starting_position"].parse().expect("Could not parse starting position"),
        });
    }
    map
}

#[derive(Debug, Clone)]
struct Disc {
    positions: u64,
    current_position: u64,
}

impl Disc {
    fn advance(&mut self) {
        self.advance_by(1)
    }

    fn advance_by(&mut self, i: u64) {
        self.current_position = (self.current_position + i) % self.positions;
    }
}
