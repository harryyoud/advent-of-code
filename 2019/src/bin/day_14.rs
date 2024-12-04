#![feature(hash_extract_if)]

use aoc_2019::get_input;
use itertools::Itertools;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

const MAX_ORE: u64 = 1_000_000_000_000;

fn main() {
    let input = get_input(14);
    let dependencies = parse_input(&input);

    let mut needed = HashMap::<&str, u32>::new();
    let mut leftovers = HashMap::<&str, u32>::new();

    for (ingredient, amount) in dependencies.get("FUEL").unwrap().1.iter() {
        *needed.entry(*ingredient).or_default() += *amount;
    }

    while needed.keys().filter(|x| **x != "ORE").count() > 0 {
        let mut new_needed = HashMap::new();
        for (outer_ingredient, needed_amount) in needed.extract_if(|x, _| *x != "ORE") {
            let (will_produce, dependencies) = dependencies.get(outer_ingredient).unwrap();

            let needed_amount = use_leftovers(&mut leftovers, outer_ingredient, needed_amount);
            if needed_amount == 0 {
                continue;
            }

            let factor = calculate_factor(*will_produce, needed_amount);

            if (*will_produce * factor) > needed_amount {
                *leftovers.entry(outer_ingredient).or_default() +=
                    (*will_produce * factor) - needed_amount;
            }

            for (ingredient, amount) in dependencies {
                *new_needed.entry(*ingredient).or_default() += factor * *amount;
            }
        }
        match needed.entry("ORE") {
            Entry::Occupied(x) => {
                *new_needed.entry("ORE").or_default() += *x.get();
            }
            Entry::Vacant(_) => {}
        }
        needed = new_needed
    }

    dbg!(needed);
    dbg!(leftovers);
}

fn calculate_factor(will_produce: u32, needed: u32) -> u32 {
    if will_produce >= needed {
        1
    } else {
        needed.div_ceil(will_produce)
    }
}

fn use_leftovers<'a>(
    leftovers: &mut HashMap<&'a str, u32>,
    ingredient: &'a str,
    mut amount_needed: u32,
) -> u32 {
    let Entry::Occupied(mut leftover) = leftovers.entry(ingredient) else {
        return amount_needed;
    };
    let leftover = leftover.get_mut();

    if *leftover > 0 {
        if amount_needed > *leftover {
            amount_needed -= *leftover;
            *leftover = 0;
        } else {
            *leftover -= amount_needed;
        }
    }

    amount_needed
}

fn parse_input(input: &str) -> HashMap<&str, (u32, Vec<(&str, u32)>)> {
    let mut dependencies = HashMap::new();

    for line in input.lines() {
        let (ingredients, output) = line.split_once(" => ").unwrap();
        let (output_amount, output_name) = output.split_once(" ").unwrap();
        let output_amount = output_amount.parse::<u32>().unwrap();
        let ingredients = ingredients
            .split(", ")
            .into_iter()
            .map(|x| {
                let (amount, ingredient) = x.split_once(" ").unwrap();
                (ingredient, amount.parse::<u32>().unwrap())
            })
            .collect_vec();
        dependencies
            .entry(output_name)
            .or_insert((output_amount, vec![]))
            .1
            .extend(ingredients);
    }

    dependencies
}
