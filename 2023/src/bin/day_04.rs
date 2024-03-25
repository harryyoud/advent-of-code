use std::collections::{HashSet, HashMap};

use aoc_2023::get_input;

fn main() {
    let input = get_input(4);

    let mut part_a_total = 0;
    let mut card_counts: HashMap<u32, u32> = HashMap::new();

    for (card_number, line) in input.lines().enumerate() {
        let card_number = card_number + 1;

        let matched_numbers = match line.char_indices().skip_while(|(_, c)| c != &':').skip(1).next() {
                Some((pos, _)) => &line[pos..],
                None => "",
            }
            .split('|')
            .map(|s|
                HashSet::from_iter(s.split_whitespace().map(|s| s.parse::<u32>().unwrap()))
            )
            .reduce(|acc: HashSet<u32>, x| acc.intersection(&x).cloned().collect())
            .unwrap()
            .len();

        if matched_numbers > 0 {
            part_a_total += 2_u32.pow(matched_numbers as u32 - 1);
        }

        for i in (card_number + 1)..(card_number + 1 + matched_numbers) {
            *card_counts.entry(i as u32).or_insert(1) += card_counts.entry(card_number as u32).or_insert(1).clone();
        }
    }

    let part_b_total = card_counts.iter().fold(0_u32, |acc, (_k, v)| acc + v);

    dbg!(part_a_total);
    dbg!(part_b_total);
}