use std::collections::{HashMap, VecDeque};

use aoc_2017::get_input;
use itertools::Itertools;

const PART_2_DANCE_TIMES: usize = 1_000_000_000;

fn main() {
    let input = get_input(16);
    let moves = parse_input(&input);

    dbg!(part_1(&moves));
    dbg!(part_2(&moves));
}

fn part_1(moves: &[DanceMove]) -> String {
    let mut dancers = ('a'..='p').collect();
    dance(&mut dancers, moves);
    dancers.into_iter().collect()
}

fn part_2(moves: &[DanceMove]) -> String {
    let mut dancers = ('a'..='p').collect();
    let mut seen = HashMap::new();

    let mut remaining_steps = 0usize;

    for i in 0..PART_2_DANCE_TIMES {
        dance(&mut dancers, moves);
        if let Some(last_seen_at) = seen.get(&dancers) {
            remaining_steps = (PART_2_DANCE_TIMES - i) % (i - last_seen_at) - 1;
            break;
        }
        seen.insert(dancers.clone(), i);
    }

    for _ in 0..remaining_steps {
        dance(&mut dancers, moves);
    }

    dancers.into_iter().collect()
}

fn dance(dancers: &mut VecDeque<char>, moves: &[DanceMove]) {
    for dance_move in moves {
        match dance_move {
            DanceMove::Spin(x) => dancers.rotate_right(x % dancers.len()),
            DanceMove::Exchange(a, b) => dancers.swap(*a, *b),
            DanceMove::Partner(a, b) => {
                let a_pos = dancers.iter().position(|x| *x == *a).unwrap();
                let b_pos = dancers.iter().position(|x| *x == *b).unwrap();
                dancers.swap(a_pos, b_pos);
            },
        }
    }
}

fn parse_input(input: &str) -> Vec<DanceMove> {
    input.trim().split(',').map(parse_dance_move).collect_vec()
}

fn parse_dance_move(input: &str) -> DanceMove {
    let mut chars = input.chars();
    let out = match chars.next().unwrap() {
        's' => {
            DanceMove::Spin(input[1..].parse().unwrap())
        },
        'x' => {
            let (a, b) = input[1..].split_once('/').unwrap();
            DanceMove::Exchange(a.parse().unwrap(), b.parse().unwrap())
        },
        'p' => {
            let a = chars.next().unwrap();
            assert!(chars.next().unwrap() == '/');
            let b = chars.next().unwrap();
            assert_eq!(chars.next(), None, "Should be no left over characters: {input}");
            DanceMove::Partner(a, b)
        },
        _ => panic!("Invalid dance move"),
    };
    out
}

enum DanceMove {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}