use std::iter;

use aoc_2023::get_input;
use itertools::Itertools;


fn main() {
    let input = get_input(12);

    let mut part_a = 0u32;
    for line in input.lines() {
        let (template, groups) = parse_line(line);
        part_a += get_solutions(template, groups);
    }
    dbg!(part_a);
}

fn parse_line<'a>(line: &'a str) -> (&'a str, Vec<u32>) {
    let (template, groups) = line.split_whitespace().collect_tuple().unwrap();
    let groups = groups.split(",").map(|s| s.parse::<u32>().unwrap()).collect_vec();
    (template, groups)
}

fn get_solutions(template: &str, groups: Vec<u32>) -> u32 {
    dbg!(&template);
    let template = template.chars().collect::<Vec<char>>();
    let max_inner_gap_size: usize = template.len() - (groups.len() - 1) - groups.iter().sum::<u32>() as usize;

    let mut gaps = vec![];
    gaps.push(iter::once(0).into_iter().chain(1..=max_inner_gap_size).collect_vec());
    for _ in 0..(groups.len() - 1) {
        gaps.push(iter::once(1).into_iter().chain(2..=max_inner_gap_size).collect_vec());
    }
    gaps.push(iter::once(0).into_iter().chain(1..=max_inner_gap_size).collect_vec());

    let mut completed_solutions = 0u32;

    'outer: for gaps in gaps.into_iter().multi_cartesian_product().filter(|z| z.iter().sum::<usize>() == template.len() - groups.iter().sum::<u32>() as usize) {
        let mut out: Vec<char> = template.clone();
        let mut i = 0usize;
        for _ in 0..gaps[0] {
            if template.get(i).unwrap_or(&'#') == &'#' {
                continue 'outer;
            }
            out[i] = '.';
            i += 1;
        }
        for (idx, j) in groups.iter().enumerate() {
            for _ in 0..*j {
                if template.get(i).unwrap_or(&'.') == &'.' {
                    continue 'outer;
                }
                out[i] = '#';
                i += 1;
            }
            for _ in 0..gaps[idx + 1] {
                if template.get(i).unwrap_or(&'#') == &'#' {
                    continue 'outer;
                }
                out[i] = '.';
                i += 1;
            }
        }
        completed_solutions += 1;
        println!("{}", out.iter().collect::<String>());
    }
    
    completed_solutions
}
