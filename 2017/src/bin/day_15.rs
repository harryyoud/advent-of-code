use aoc_2017::get_input;
use itertools::{iterate, Itertools};

const A_MULTIPLY: usize = 16807;
const B_MULTIPLY: usize = 48271;
const GEN_MOD: usize = 2147483647;

fn main() {
    let input = get_input(15);
    let (gen_a_start, gen_b_start) = parse_input(&input);

    let generator_a = make_generator(gen_a_start, A_MULTIPLY);
    let generator_b = make_generator(gen_b_start, B_MULTIPLY);

    dbg!(part_1(generator_a, generator_b));

    let generator_a = make_generator(gen_a_start, A_MULTIPLY);
    let generator_b = make_generator(gen_b_start, B_MULTIPLY);

    dbg!(part_2(generator_a, generator_b));
}

fn part_1(
    generator_a: impl Iterator<Item = usize>,
    generator_b: impl Iterator<Item = usize>,
) -> usize {
    generator_a.zip(generator_b)
        .skip(1)
        .take(40_000_000)
        .filter(|(a, b)| *a as u16 == *b as u16)
        .count()
}

fn part_2(
    generator_a: impl Iterator<Item = usize>,
    generator_b: impl Iterator<Item = usize>,
) -> usize {
    let generator_a = generator_a.filter(|a| a % 4 == 0);
    let generator_b = generator_b.filter(|b| b % 8 == 0);

    generator_a.zip(generator_b)
        .skip(1)
        .take(5_000_000)
        .filter(|(a, b)| *a as u16 == *b as u16)
        .count()
}

fn make_generator(start: usize, multiplier: usize) -> impl Iterator<Item = usize> {
    iterate(
        start,
        move |x| (x * multiplier) % GEN_MOD
    )
}

fn parse_input(input: &str) -> (usize, usize) {
    input.lines().map(|x| {
        x.rsplit_once(' ').unwrap().1.parse::<usize>().unwrap()
    }).collect_tuple().unwrap()
}