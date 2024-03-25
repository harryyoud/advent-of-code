use aoc_2015::get_input;
use itertools::Itertools;
use regex::Regex;

fn main() {
    let input = get_input(15);

    let ingredients = parse_input(&input);

    dbg!(part_1(&ingredients));
    dbg!(part_2(&ingredients));
}

#[derive(Debug)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

fn part_1(ingredients: &[Ingredient]) -> u32 {
    ingredients
        .iter()
        .combinations_with_replacement(100)
        .map(|x| calculate_cookie_score(&x))
        .max()
        .unwrap()
}

fn part_2(ingredients: &[Ingredient]) -> u32 {
    ingredients
        .iter()
        .combinations_with_replacement(100)
        .filter(|x| calculate_cookie_calories(&x) == 500)
        .map(|x| calculate_cookie_score(&x))
        .max()
        .unwrap()
}

fn calculate_cookie_score(ingredients: &[&Ingredient]) -> u32 {
    let capacity: u32 = ingredients.iter().map(|x| x.capacity).sum::<i32>().max(0) as u32;
    let durability: u32 = ingredients.iter().map(|x| x.durability).sum::<i32>().max(0) as u32;
    let flavor: u32 = ingredients.iter().map(|x| x.flavor).sum::<i32>().max(0) as u32;
    let texture: u32 = ingredients.iter().map(|x| x.texture).sum::<i32>().max(0) as u32;
    [capacity, durability, flavor, texture].into_iter().product()
}

fn calculate_cookie_calories(ingredients: &[&Ingredient]) -> u32 {
    ingredients.iter().map(|x| x.calories).sum::<i32>().max(0) as u32
}

fn parse_input(input: &str) -> Vec<Ingredient> {
    let re = Regex::new(r#"(\-?[0-9]+)"#).unwrap();

    input.lines().map(|line| {
        let (capacity, durability, flavor, texture, calories) = re.captures_iter(line)
            .map(|x| x.extract::<1>().0)
            .map(|x| x.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();
        Ingredient {
            capacity, durability, flavor, texture, calories
        }
    }).collect_vec()
}