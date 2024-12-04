use aoc_2020::get_input;
use itertools::Itertools;
use lazy_regex::regex_captures;

fn main() {
    let input = get_input(2);
    let input = input.lines().map(parse_line).collect_vec();

    dbg!(part_1(&input));
    dbg!(part_2(&input));
}

fn part_1(input: &[(Policy, &str)]) -> usize {
    input
        .into_iter()
        .filter(|(policy, password)| policy.is_valid_v1(password))
        .count()
}

fn part_2(input: &[(Policy, &str)]) -> usize {
    input
        .into_iter()
        .filter(|(policy, password)| policy.is_valid_v2(password))
        .count()
}

#[derive(Clone)]
struct Policy {
    character: char,
    left: usize,
    right: usize,
}

impl Policy {
    fn is_valid_v1(&self, password: &str) -> bool {
        (self.left..=self.right)
            .contains(&password.chars().filter(|x| *x == self.character).count())
    }

    fn is_valid_v2(&self, password: &str) -> bool {
        let password = password.chars().collect_vec();
        (password[self.left - 1] == self.character) ^ (password[self.right - 1] == self.character)
    }
}

fn parse_line(line: &str) -> (Policy, &str) {
    let (_, lower, upper, c, password) = regex_captures!(r#"(\d+)-(\d+) ([a-z]): ([a-z]+)"#, line)
        .expect(&format!("Line not in valid format: {line}"));

    (
        Policy {
            left: lower.parse().unwrap(),
            right: upper.parse().unwrap(),
            character: c.chars().next().unwrap(),
        },
        password,
    )
}
