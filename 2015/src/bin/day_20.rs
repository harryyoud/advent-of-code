use aoc_2015::get_input;
use rayon::prelude::*;

fn main() {
    let input: u64 = get_input(20).trim().parse().unwrap();

    dbg!(part_1(input));
    dbg!(part_2(input));
}

fn part_1(min_presents: u64) -> u64 {
    (1..min_presents)
        .into_par_iter()
        .find_map_first(|house| {
            let presents = (1..=(((house as f64).sqrt() + 1.0) as u64))
                .filter(|n| house % n == 0)
                .map(|n| n + (house / n))
                .sum::<u64>() * 10;
            if presents < min_presents {
                return None;
            }
            return Some(house);
        })
        .unwrap()
}

fn part_2(min_presents: u64) -> u64 {
    (1..min_presents)
    .into_par_iter()
    .find_map_first(|house| {
        let presents = (1..=(((house as f64).sqrt() + 1.0) as u64))
            .filter(|n| house % n == 0)
            .map(|n| if n <= 50 { house / n } else { 0 } + if house / n <= 50 { n } else { 0 })            .sum::<u64>() * 11;
        if presents < min_presents {
            return None;
        }
        return Some(house);
    })
    .unwrap()
}