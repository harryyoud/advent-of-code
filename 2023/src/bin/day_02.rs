use std::collections::HashMap;

use aoc_2023::get_input;

#[derive(PartialEq, Eq, Hash)]
enum CubeColour {
    Green,
    Red,
    Blue,
}

impl CubeColour {
    fn limit(&self) -> u32 {
        match self {
            Self::Green => 13,
            Self::Red => 12,
            Self::Blue => 14,
        }
    }
}

fn main() {
    let input = get_input(2);

    let mut possible_game_ids_sum = 0u32;
    let mut maximum_drawn_product_sum = 0u32;

    for line in input.lines() {
        // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        //         _____________ a set
        //         ______________________________________________ a game
        //      _ game_id

        let game_id = extract_game_id(line);
        let sets = extract_sets_from_line(line);

        let mut max_colour_value_in_game: HashMap<CubeColour, u32> = {
            let mut h = HashMap::new();
            h.insert(CubeColour::Blue, 0);
            h.insert(CubeColour::Green, 0);
            h.insert(CubeColour::Red, 0);
            h
        };

        for set in sets {
            for (colour, value) in extract_colors_and_numbers_from_set(set) {
                if max_colour_value_in_game.get(&colour).unwrap() < &value {
                    max_colour_value_in_game.insert(colour, value);
                }
            }
        }

        if ! max_colour_value_in_game.iter().any(|(colour, maximum_drawn)| maximum_drawn > &colour.limit()) {
            possible_game_ids_sum += game_id;
        }

        maximum_drawn_product_sum += max_colour_value_in_game.iter().map(|(_, max_drawn)| max_drawn).product::<u32>();
    }

    println!("Possible game ID sum: {possible_game_ids_sum}");
    println!("Minimum of each cube product sum: {maximum_drawn_product_sum}");
}

// input should be formatted as "1 green, 3 red, 6 blue"
fn extract_colors_and_numbers_from_set(set: &str) -> Vec<(CubeColour, u32)> {
    let mut out = vec![];

    for draw in set.split(",").map(|s| s.trim()).map(|s| s.split(" ").collect::<Vec<&str>>()) {
        let number = draw[0].parse::<u32>().unwrap();
        out.push((match draw[1] {
            "green" => CubeColour::Green,
            "red" => CubeColour::Red,
            "blue" => CubeColour::Blue,
            _ => { panic!("unknown color: {}", draw[1]) },
        }, number));
    }

    out
}

fn extract_game_id(line: &str) -> u32 {
    line.split(":").next().unwrap().split(" ").last().unwrap().parse::<u32>().unwrap()
}

fn extract_sets_from_line(line: &str) -> Vec<&str> {
    line.split(":").last().unwrap().trim().split(";").map(|s| s.trim()).collect()
}