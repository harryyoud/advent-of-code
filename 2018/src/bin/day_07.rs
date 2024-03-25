use std::collections::{HashMap, VecDeque};

use aoc_2018::get_input;
use itertools::Itertools;
use lazy_regex::regex;

fn main() {
    let input = get_input(7);
    let dependencies = parse_input(&input);

    dbg!(part_1(&dependencies));
    dbg!(part_2(&dependencies));
}

fn part_1(dependencies: &HashMap<StepKey, Step>) -> String {
    solve(dependencies, 1).1
}

fn part_2(dependencies: &HashMap<StepKey, Step>) -> usize {
    solve(dependencies, 5).0
}

fn solve(dependencies: &HashMap<StepKey, Step>, worker_count: usize) -> (usize, String) {
    let addon_time: HashMap<char, usize> = ('A'..='Z').enumerate().map(|(c, x)| (x, c + 61)).collect();
    let mut steps_taken = vec![];
    let mut actively_working = VecDeque::new(); // time first for sorting as we want to pop the closest one to finishing
    let mut wallclock = 0; // we advance this everytime we pop an step by that step's addon_time

    loop {
        let currently_working = actively_working.iter().map(|(_time, key)| *key).collect_vec();
        let mut next_candidates = find_next_satisfied(&steps_taken, dependencies).into_iter().filter(|x| !currently_working.contains(x)).collect::<VecDeque<_>>();

        if actively_working.is_empty() && next_candidates.is_empty() {
            break;
        }

        if actively_working.len() == worker_count || next_candidates.is_empty() {
            // remove one from active working
            actively_working.make_contiguous().sort();
            let (time_at_removal, step_key) = actively_working.pop_front().unwrap();
            wallclock += time_at_removal;

            steps_taken.push(step_key);
            actively_working.iter_mut().for_each(|(time, _k)| *time -= time_at_removal);
        } else {
            // start work on another
            let next_candidate = next_candidates.pop_front().unwrap();
            actively_working.push_back((addon_time[&next_candidate], next_candidate));
        }
    }

    (wallclock, steps_taken.into_iter().collect())
}

fn parse_input(input: &str) -> HashMap<StepKey, Step> {
    let mut dependencies = HashMap::new();
    let re = regex!(r#"^Step (?<dependency>\w) must be finished before step (?<depender>\w) can begin\.$"#);
    for line in input.lines() {
        let captures = re.captures(line).unwrap_or_else(|| panic!("Line did not match regex: {line}"));
        dependencies.entry(captures["depender"].chars().next().unwrap())
            .or_insert(Step { dependencies: vec![] })
            .dependencies
            .push(captures["dependency"].chars().next().unwrap());
        dependencies.entry(captures["dependency"].chars().next().unwrap()).or_insert(Step { dependencies: vec![] });
    }
    dependencies
}

fn find_next_satisfied(satisfied: &[StepKey], dependencies: &HashMap<StepKey, Step>) -> VecDeque<StepKey> {
    dependencies.iter()
        .filter(|(k, _v)| !satisfied.contains(k))
        .filter(|(_k, v)| {
            v.dependencies.iter().all(|x| satisfied.contains(x))
        })
        .map(|(k, _v)| *k)
        .sorted()
        .collect()
}

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
struct Step {
    dependencies: Vec<StepKey>,
}

type StepKey = char;