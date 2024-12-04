use aoc_2016::get_input;
use pathfinding::directed::{bfs::bfs, dijkstra::dijkstra_reach};

const START: (u64, u64) = (1, 1);
const END: (u64, u64) = (31, 39);

fn main() {
    let input = get_input(13);
    let input = input.trim().parse::<u64>().unwrap();

    dbg!(part_1(input));
    dbg!(part_2(input));
}

fn part_1(input: u64) -> usize {
    bfs(&START, |x| get_candidates(*x, input), |x| *x == END)
        .expect("No path found")
        .len()
        - 1
}

fn part_2(input: u64) -> usize {
    dijkstra_reach(&START, |x, _cost| {
        get_candidates(*x, input).into_iter().map(|a| (a, 1))
    })
    .filter(|x| x.total_cost <= 50)
    .count()
}

fn is_open((x, y): (u64, u64), input: u64) -> bool {
    let mut sum = x * x;
    sum += 3 * x;
    sum += 2 * x * y;
    sum += y;
    sum += y * y;
    sum += input;

    sum.count_ones() % 2 == 0
}

fn get_candidates(from: (u64, u64), input: u64) -> Vec<(u64, u64)> {
    let mut v = vec![
        (from.0 + 1, from.1), // right
        (from.0, from.1 + 1), // down
    ];
    if from.0 > 0 {
        v.push((from.0 - 1, from.1)); // left
    }
    if from.1 > 0 {
        v.push((from.0, from.1 - 1)); // up
    }
    v.retain(|x| is_open(*x, input));
    v
}
