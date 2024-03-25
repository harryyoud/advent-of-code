use aoc_2016::get_input;
use itertools::Itertools;

fn main() {
    let input = get_input(3);
    let part_1 = count_valid_triangles(&parse_input_per_line(&input));
    let part_2 = count_valid_triangles(&parse_input_per_3_rows(&input));
    dbg!(part_1, part_2);
}

fn count_valid_triangles(triangles: &Vec<(u32, u32, u32)>) -> u32 {
    triangles.iter()
        .filter(|(a, b, c)| {
            (a + b) > *c &&
            (a + c) > *b &&
            (b + c) > *a
        })
        .count() as u32
}

fn parse_input_per_line(input: &str) -> Vec<(u32, u32, u32)> {
    let mut v = vec![];
    for line in input.lines() {
        let (a, b, c) = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect_tuple()
            .unwrap();
        v.push((a, b, c));
    }
    v
}

fn parse_input_per_3_rows(input: &str) -> Vec<(u32, u32, u32)> {
    let mut v = vec![];
    for (line_a, line_b, line_c) in input.lines().tuples() {
        let line_a = line_a.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect_tuple::<(_, _, _)>().unwrap();
        let line_b = line_b.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect_tuple::<(_, _, _)>().unwrap();
        let line_c = line_c.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect_tuple::<(_, _, _)>().unwrap();
        v.push((line_a.0, line_b.0, line_c.0));
        v.push((line_a.1, line_b.1, line_c.1));
        v.push((line_a.2, line_b.2, line_c.2));
    }
    v
}
