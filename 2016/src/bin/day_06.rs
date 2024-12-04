use aoc_2016::get_input;
use itertools::Itertools;

fn main() {
    let input = get_input(6);
    let all_chars = parse_input(&input);
    dbg!(part_1(&all_chars));
    dbg!(part_2(&all_chars));
}

fn part_1(all_chars: &Vec<Vec<char>>) -> String {
    let mut message = String::with_capacity(all_chars.len());

    for pos in all_chars {
        message.push(
            pos.iter()
                .counts()
                .into_iter()
                .max_by_key(|(_c, count)| *count)
                .map(|(cha, _count)| *cha)
                .unwrap(),
        );
    }

    message
}

fn part_2(all_chars: &Vec<Vec<char>>) -> String {
    let mut message = String::with_capacity(all_chars.len());

    for pos in all_chars {
        message.push(
            pos.iter()
                .counts()
                .into_iter()
                .min_by_key(|(_c, count)| *count)
                .map(|(cha, _count)| *cha)
                .unwrap(),
        );
    }

    message
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let message_length: usize = input.lines().next().unwrap().len();
    let mut all_chars = vec![];

    for _ in 0..message_length {
        all_chars.push(vec![]);
    }

    for line in input.lines() {
        for (idx, c) in line.chars().enumerate() {
            all_chars.get_mut(idx).unwrap().push(c);
        }
    }

    all_chars
}
