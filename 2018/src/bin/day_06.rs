use std::collections::HashMap;

use aoc_2018::get_input;
use itertools::Itertools;

fn main() {
    let input = get_input(6);
//     let input = "1, 1
// 1, 6
// 8, 3
// 3, 4
// 5, 5
// 8, 9";
    let points_of_interest = parse_input(&input);

    dbg!(part_1(&points_of_interest));

}

fn part_1(points_of_interest: &[(isize, isize)]) -> usize {
    let max_x = points_of_interest.iter().map(|(x, _y)| *x).max().unwrap();
    let max_y = points_of_interest.iter().map(|(_x, y)| *y).max().unwrap();
    let min_x = points_of_interest.iter().map(|(x, _y)| *x).min().unwrap();
    let min_y = points_of_interest.iter().map(|(_x, y)| *y).min().unwrap();

    let _letters = ('a'..='z').chain('A'..='Z').collect_vec();

    let mut map = HashMap::<usize, usize>::new();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let point = (x, y);
            if let Some(z) = points_of_interest.iter().position(|z| *z == point) {
                *map.entry(z).or_insert(0) += 1;
                continue;
            }
            let nearest_pois = points_of_interest.iter()
                .enumerate()
                .min_set_by_key(|(_x, p)| manhattan_distance(point, **p))
                .into_iter()
                .map(|(x, _p)| x).collect_vec();
            if nearest_pois.len() > 1 {
                continue;
            }
            *map.entry(nearest_pois[0]).or_insert(0) += 1;
        }
    }

    map.iter().filter(|(k, _v)| {
        let p = points_of_interest[**k];
        p.0 < max_x && p.0 > min_x && p.1 < max_y && p.0 > min_y
    }).map(|(_k, v)| *v).max().unwrap()
}

fn parse_input(input: &str) -> Vec<(isize, isize)> {
    input.lines().map(|line| {
        line.split(", ").map(|x| x.parse().unwrap()).collect_tuple().unwrap()
    }).collect()
}

fn manhattan_distance(point: (isize, isize), other: (isize, isize)) -> usize {
    point.0.abs_diff(other.0) + point.1.abs_diff(other.1)
}