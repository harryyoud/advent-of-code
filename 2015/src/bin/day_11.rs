use aoc_2015::get_input;
use itertools::Itertools;

fn main() {
    let input = get_input(11);
    dbg!(part_1(&input));
    dbg!(part_2(&input));
}

fn part_1(input: &str) -> String {
    let mut password = input.to_string();
    while !is_valid(&password) {
        password = increment(password);
    }
    password
}

fn part_2(input: &str) -> String {
    part_1(&increment(part_1(input)))
}

fn is_valid(password: &str) -> bool {
    if password.chars().any(|x| ['i', 'o', 'l'].contains(&x)) {
        return false;
    }

    if !password
        .chars()
        .tuple_windows()
        .enumerate()
        .filter(|(_idx, (a, b))| a == b)
        .map(|(idx, (_a, _b))| idx)
        .tuple_combinations()
        .any(|(x, y)| x.abs_diff(y) > 2)
    {
        return false;
    }

    password.chars().tuple_windows().any(|(x, y, z)| {
        if y == 'z' || x == 'z' {
            return false;
        };
        z == next_char(y) && y == next_char(x)
    })
}

fn next_char(c: char) -> char {
    let alphabet = ('a'..='z').collect_vec();
    alphabet[((c as usize) - ('a' as usize) + 1) % 26]
}

fn increment(password: String) -> String {
    let mut increase_next = true;
    let mut out = password.chars().rev().collect_vec();

    for c in out.iter_mut() {
        if !increase_next {
            break;
        }
        *c = next_char(*c);
        increase_next = *c == 'a';
    }

    out.iter().rev().collect()
}
