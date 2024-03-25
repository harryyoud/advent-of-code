use aoc_2015::get_input;
use itertools::Itertools;

const TOTAL_TIME: u32 = 2503;

struct Reindeer {
    speed: u32,
    move_time: u32,
    rest_time: u32,
}

fn main() {
    let input = get_input(14);

    let reindeers = parse_input(&input);
    dbg!(part_1(&reindeers));
    dbg!(part_2(&reindeers));
}

fn part_1(reindeers: &[Reindeer]) -> u32 {
    let mut max_distance = 0u32;

    for &Reindeer { speed, move_time, rest_time } in reindeers {
        let mut distance_travelled = 0;

        let cycle = move_time + rest_time;
        let complete_cycles = TOTAL_TIME / cycle;
        let remaining_time = TOTAL_TIME % cycle;

        distance_travelled += complete_cycles * speed * move_time;

        if remaining_time >= move_time {
            distance_travelled += speed * move_time;
        } else {
            distance_travelled += speed * remaining_time;
        }

        max_distance = max_distance.max(distance_travelled);
    }

    max_distance
}

fn part_2(reindeers: &[Reindeer]) -> u32 {
    let mut score_board = vec![0_u32; reindeers.len()];
    let mut distances = vec![0_u32; reindeers.len()];

    for current_time in 0..TOTAL_TIME {
        for (idx, reindeer) in reindeers.iter().enumerate() {
            let cycle = reindeer.move_time + reindeer.rest_time;
            let remaining = current_time % cycle;
            
            if remaining < reindeer.move_time {
                distances[idx] += reindeer.speed;
            }
        }

        let max_distance = distances.iter().max().unwrap();

        distances.iter()
            .enumerate()
            .filter(|(_idx, dist)| *dist == max_distance)
            .map(|(idx, _dist)| idx)
            .for_each(|idx| score_board[idx] += 1);
    }

    *score_board.iter().max().unwrap()
}

fn parse_input(input: &str) -> Vec<Reindeer> {
    input.lines().map(|line| {
        let (speed, move_time, rest_time) = line.split_whitespace().filter_map(|x| x.parse::<u32>().ok()).collect_tuple().unwrap();
        Reindeer {
            speed, move_time, rest_time
        }
    }).collect_vec()
}