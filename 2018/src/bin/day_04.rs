use std::{collections::HashMap, ops::Range};

use aoc_2018::get_input;
use itertools::Itertools;
use lazy_regex::regex;

fn main() {
    let input = get_input(4);
    let sleeps = parse_input(&input);

    dbg!(part_1(&sleeps));
    dbg!(part_2(&sleeps));
}

fn part_1(sleeps: &HashMap<usize, Vec<SleepEvent>>) -> usize {
    let (guard_num, events) = sleeps
        .iter()
        .max_by_key(|(_guard_num, v)| {
            v.iter()
                .map(|x| (x.hour_range.clone().count() * 60) + (x.minute_range.clone().count()))
                .sum::<usize>()
        })
        .unwrap();

    let most_common_minute = events
        .iter()
        .flat_map(|x| x.minute_range.clone())
        .counts()
        .into_iter()
        .max_by_key(|(_minute, count)| *count)
        .map(|(minute, _count)| minute)
        .unwrap();

    most_common_minute * guard_num
}

fn part_2(sleeps: &HashMap<usize, Vec<SleepEvent>>) -> usize {
    let (guard_num, asleep_max_minute) = sleeps
        .iter()
        .map(|(guard_num, events)| {
            (
                *guard_num,
                events
                    .iter()
                    .flat_map(|x| x.minute_range.clone())
                    .counts()
                    .into_iter()
                    .max_by_key(|(_minute, count)| *count)
                    .unwrap(),
            )
        })
        .max_by_key(|(_guard_num, (_minute, count))| *count)
        .map(|(guard_num, (minute, _count))| (guard_num, minute))
        .unwrap();

    guard_num * asleep_max_minute
}

#[derive(Debug)]
struct SleepEvent {
    hour_range: Range<usize>,
    minute_range: Range<usize>,
}

fn parse_input(input: &str) -> HashMap<usize, Vec<SleepEvent>> {
    let mut guard_sleeps: HashMap<usize, Vec<SleepEvent>> = HashMap::new();

    let mut current_guard = 0;
    let mut sleep_start = (0, 0);

    for line in input.lines().sorted() {
        let re = regex!(
            r#"^\[(?<year>\d{4})-(?<month>\d{2})-(?<day>\d{2}) (?<hour>\d{2}):(?<minute>\d{2})\] (?:Guard #(?<guard_num>\d+) begins shift|falls asleep|wakes up)$"#
        );
        let capture = re.captures(line).unwrap();

        if line.contains("begins shift") {
            current_guard = capture["guard_num"].parse().unwrap();
        } else if line.contains("falls asleep") {
            sleep_start = (
                capture["hour"].parse().unwrap(),
                capture["minute"].parse().unwrap(),
            );
        } else if line.contains("wakes up") {
            guard_sleeps
                .entry(current_guard)
                .or_default()
                .push(SleepEvent {
                    hour_range: (sleep_start.0)..capture["hour"].parse().unwrap(),
                    minute_range: (sleep_start.1)..capture["minute"].parse().unwrap(),
                })
        }
    }

    guard_sleeps
}
