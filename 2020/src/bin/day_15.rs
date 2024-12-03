use std::collections::HashMap;
use itertools::Itertools;
use aoc_2020::get_input;

fn main() {
    let input = get_input(15);
    let starting_numbers = input
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect_vec();

    dbg!(part_1(starting_numbers.clone()));
    dbg!(part_2(starting_numbers));
}

fn part_1(starting_numbers: Vec<u32>) -> u32 {
    GameState::new(starting_numbers).into_iter().nth(2020 - 1).unwrap()
}

fn part_2(starting_numbers: Vec<u32>) -> u32 {
    // probably a more efficient way, but this is quite quick
    GameState::new(starting_numbers).into_iter().nth(30000000 - 1).unwrap()
}

struct GameState {
    current_turn: usize,
    spoken_on_turn: HashMap<u32, Vec<usize>>,
    starting_numbers: Vec<u32>,
    last_spoken: u32,
}

impl GameState {
    fn new(starting_numbers: Vec<u32>) -> Self {
        Self {
            current_turn: 0,
            spoken_on_turn: HashMap::new(),
            last_spoken: u32::MAX,
            starting_numbers,
        }
    }

    fn last_two_turns_for(&self, number: u32) -> Option<(usize, usize)> {
        let turns = self.spoken_on_turn.get(&number)?;
        if turns.len() < 2 {
            return None
        };
        turns[turns.len() - 2..].into_iter().copied().next_tuple()
    }
}

impl IntoIterator for GameState {
    type Item = u32;
    type IntoIter = GameIterator;

    fn into_iter(self) -> Self::IntoIter {
        GameIterator { state: self }
    }
}

struct GameIterator {
    state: GameState,
}

impl Iterator for GameIterator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let out = if let Some(x) = self.state.starting_numbers.get(self.state.current_turn) {
            // still on the starting round
            *x
        } else if let Some((x, y)) = self.state.last_two_turns_for(self.state.last_spoken) {
            // been said in the last two turns, find the difference between the turn numbers
            y.abs_diff(x) as u32
        } else {
            // only been said once
            0
        };

        self.state.spoken_on_turn.entry(out).or_default().push(self.state.current_turn);
        self.state.current_turn += 1;
        self.state.last_spoken = out;

        Some(out)
    }
}
