use aoc_2024::get_input;
use lazy_regex::regex;

fn main() {
    let input = get_input(3);

    dbg!(part_1(&input));
    dbg!(part_2(&input));
}

fn part_1(input: &str) -> u32 {
    regex!(r#"mul\((\d+),(\d+)\)"#)
        .captures_iter(&input)
        .map(|x| x[1].parse::<u32>().unwrap() * x[2].parse::<u32>().unwrap())
        .sum::<u32>()
}

// just delete everything between don't() and do()
fn part_2(input: &str) -> u32 {
    part_1(&regex!(r#"(?s)don't\(\).*?do\(\)"#).replace_all(&input, ""))
}
