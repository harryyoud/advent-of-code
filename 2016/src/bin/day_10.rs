use std::collections::{HashMap, HashSet};

use aoc_2016::get_input;
use regex::Regex;

fn main() {
    let input = get_input(10);
    let (mut bots, mut outputs) = parse_input(&input);
    process(&mut bots, &mut outputs);
    dbg!(part_1(&bots));
    dbg!(part_2(&outputs));
}

fn part_1(bots: &HashMap<u32, Bot>) -> u32 {
    bots
        .iter()
        .filter(|(_bot_num, bot)| bot.has_61_17())
        .map(|(bot_num, _bot)| *bot_num)
        .next()
        .unwrap()
}

fn part_2(outputs: &HashMap<u32, u32>) -> u32 {
    (0..=2).map(|x| outputs.get(&x).unwrap()).product::<u32>()
}

fn process(bots: &mut HashMap<u32, Bot>, outputs: &mut HashMap<u32, u32>) {
    let mut processed = HashSet::new();

    while let Some(next_bot) = bots
        .iter()
        .filter(|(_bot_num, bot)| bot.can_process()).find(|(bot_num, _bot)| !processed.contains(*bot_num))
    {
        processed.insert(*next_bot.0);

        let (
            (low_val, low_dest),
            (high_val, high_dest)
        ) = next_bot.1.process_chip();

        match low_dest {
            Destination::Bot(bot_num) => { bots.get_mut(&bot_num).unwrap().push_chip(low_val); },
            Destination::Output(output_num) =>{ outputs.insert(output_num, low_val); },
        }
        match high_dest {
            Destination::Bot(bot_num) => { bots.get_mut(&bot_num).unwrap().push_chip(high_val); },
            Destination::Output(output_num) => { outputs.insert(output_num, high_val); },
        }
    }
}

fn parse_input(input: &str) -> (HashMap<u32, Bot>, HashMap<u32, u32>) {
    let mut bots: HashMap<u32, Bot> = HashMap::new();
    let mut fixed_values: HashMap<u32, u32> = HashMap::new();
    let re = Regex::new(r#"(?:bot (?<bot_from>[0-9]+) gives low to (?<low_goes_to_type>bot|output) (?<low_goes_to_num>[0-9]+) and high to (?<high_goes_to_type>bot|output) (?<high_goes_to_num>[0-9]+)|value (?<value_from>[0-9]+) goes to bot (?<value_goes_to_bot_num>[0-9]+))"#).unwrap();

    for line in input.lines() {
        let captures = re.captures(line).unwrap();
        if let Some(bot_num) = captures.name("bot_from") {
            let low_dest = captures.name("low_goes_to_num").unwrap().as_str().parse().unwrap();
            let low_dest = match captures.name("low_goes_to_type").unwrap().as_str() {
                "bot" => Destination::Bot(low_dest),
                "output" => Destination::Output(low_dest),
                _ => panic!("Invalid destination type")
            };
            let high_dest = captures.name("high_goes_to_num").unwrap().as_str().parse().unwrap();
            let high_dest = match captures.name("high_goes_to_type").unwrap().as_str() {
                "bot" => Destination::Bot(high_dest),
                "output" => Destination::Output(high_dest),
                _ => panic!("Invalid destination type")
            };
            bots.insert(bot_num.as_str().parse().unwrap(), Bot {
                low_destination: low_dest,
                high_destination: high_dest,
                chips: (None, None),
            });
        } else {
            fixed_values.insert(
                captures.name("value_from").unwrap().as_str().parse().unwrap(),
                captures.name("value_goes_to_bot_num").unwrap().as_str().parse().unwrap(),
            );
        }
    }

    for (value, bot_num) in fixed_values {
        bots.get_mut(&bot_num).unwrap().push_chip(value);
    }

    (bots, HashMap::new())
}

#[derive(Debug, Copy, Clone)]
enum Destination {
    Bot(u32),
    Output(u32),
}

#[derive(Debug, Clone)]
struct Bot {
    low_destination: Destination,
    high_destination: Destination,
    chips: (Option<u32>, Option<u32>),
}

impl Bot {
    fn can_process(&self) -> bool {
        self.chips.0.is_some() && self.chips.1.is_some()
    }

    fn process_chip(&self) -> ((u32, Destination), (u32, Destination)) {
        if !self.can_process() {
            panic!("Unable to process, only have 2 chaps");
        }
        (
            (
                self.chips.0.unwrap().min(self.chips.1.unwrap()),
                self.low_destination,
            ),
            (
                self.chips.0.unwrap().max(self.chips.1.unwrap()),
                self.high_destination,
            ),
        )
    }

    fn push_chip(&mut self, chip: u32) {
        if self.chips.0.is_some() {
            if self.chips.1.is_some() {
                panic!("Bot is busy already!");
            }
            self.chips.1 = Some(chip);
        } else {
            self.chips.0 = Some(chip);
        }
    }

    fn has_61_17(&self) -> bool {
        self.chips.0.unwrap().min(self.chips.1.unwrap()) == 17 &&
        self.chips.0.unwrap().max(self.chips.1.unwrap()) == 61
    }
}