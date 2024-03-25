use std::collections::HashSet;
use std::str::FromStr;

use aoc_2015::get_input;
use aoc_lib::Paragraphs;
use itertools::Itertools;

fn main() {
    let input = get_input(19);
    let (replacements, molecule) = parse_input(&input);

    dbg!(part_1(&replacements, molecule));
    // dbg!(part_2(&replacements, molecule));
}

fn part_1(replacements: &[(&str, &str)], molecule: &str) -> u64 {
    let mut set: HashSet<String> = HashSet::new();

    for (from, to) in replacements {
        for (idx, _match) in molecule.match_indices(from) {
            let mut s: String = String::from_str(&molecule[0..idx]).unwrap();
            s.push_str(&molecule[idx..].replacen(from, to, 1));
            set.insert(s);
        }
    }

    set.len() as u64
}

// fn part_2(replacements: &[(&str, &str)], molecule: &str) -> u64 {
//     let results = dijkstra(
//         molecule.to_string(),
//         |x: &str| replacements.iter().map(|(from, to)| {
//             x.match_indices(to).filter_map(move |(idx, _match)| {
//                 let mut s: String = String::from_str(&x[0..idx]).unwrap();
//                 s.push_str(&x[idx..].replacen(to, from, 1));
//                 Some(s)
//             })
//         }),
//         |x| x == "e"
//     );
//     (results.unwrap().len() - 1) as u64
// }

fn parse_input<'a>(input: &'a String) -> (Vec<(&'a str, &'a str)>, &'a str) {
    let (replacements, mut molecule) = input.paragraphs().collect_tuple().unwrap();
    let replacements = replacements.into_iter().map(|s| s.split(" => ").collect_tuple().unwrap()).collect_vec();
    let molecule = molecule.pop().unwrap();

    (replacements, molecule)
}