use aoc_2015::get_input;

use regex::Regex;
use serde_json::Value;

fn main() {
    let input = get_input(12);

    dbg!(part_1(&input));
    dbg!(part_2(&input));
}

fn part_1(input: &str) -> i32 {
    let re = Regex::new(r#"(\-?[0-9]+)"#).unwrap();

    re.captures_iter(input)
        .map(|x| x.extract::<1>())
        .map(|x| x.0)
        .map(|x| x.parse::<i32>().unwrap())
        .sum::<i32>()
}

fn part_2(input: &str) -> i32 {
    recursive_find_num(&serde_json::from_str(input).unwrap())
}

fn recursive_find_num(x: &Value) -> i32 {
    let mut total = 0;
    match x {
        Value::Array(vals) => {
            for val in vals {
                total += recursive_find_num(val);
            }
        }
        Value::Number(num) => {
            total += num.as_i64().unwrap() as i32;
        }
        Value::Object(map) => {
            for (_k, v) in map.iter() {
                if *v == Value::String("red".to_string()) {
                    return 0;
                }
                total += recursive_find_num(v);
            }
        }
        _ => (),
    };
    total
}
