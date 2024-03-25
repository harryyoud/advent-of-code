use std::collections::HashMap;

use aoc_2023::get_input;
use itertools::Itertools;
use num::integer::lcm;

#[derive(Debug)]
struct Module {
    connects_to: Vec<String>,
    module_type: ModuleType,
}

#[derive(Debug)]
enum ModuleType {
    FlipFlow(FlipFlowState),
    Conjunction(ConjunctionState),
    Broadcaster,
}

impl ModuleType {
    fn push_input(&mut self, pulse: Pulse, from: &str) -> Option<Pulse> {
        match self {
            ModuleType::FlipFlow(ref mut state) => {
                match pulse {
                    Pulse::Low => {
                        if state.on() {
                            state.flip();
                            Some(Pulse::Low)
                        } else {
                            state.flip();
                            Some(Pulse::High)
                        }
                    },
                    Pulse::High => return None,
                }
            },
            ModuleType::Conjunction(ref mut state) => {
                state.inputs.insert(from.to_string(), pulse);
                if state.inputs.iter().all(|(_name, pulse)| pulse.high()) {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            },
            ModuleType::Broadcaster => Some(pulse),
        }
    }
}

#[derive(Debug)]
enum FlipFlowState {
    On,
    Off,
}

impl FlipFlowState {
    fn on(&self) -> bool {
        matches!(self, Self::On)
    }

    fn flip(&mut self) {
        if self.on() {
            *self = FlipFlowState::Off;
        } else {
            *self = FlipFlowState::On;
        }
    }
}

#[derive(Debug)]
struct ConjunctionState {
    inputs: HashMap<String, Pulse>,
}

#[derive(Debug, Clone, Copy)]
enum Pulse {
    Low,
    High,
}

impl Pulse {
    fn high(&self) -> bool {
        matches!(self, Self::High)
    }
    fn low(&self) -> bool {
        matches!(self, Self::Low)
    }
}

fn main() {
    let input = get_input(20);

    let (mut modules, reverse_lookup) = build_module_tree_from_input(&input);

    let mut next_visits = vec![("button".to_string(), "broadcaster".to_string(), Pulse::Low)];
    let mut low_pulse_count = 0;
    let mut high_pulse_count = 0;

    let mut leads_to_rx: HashMap<String, Option<u64>> = HashMap::from_iter(reverse_lookup.get("rx").unwrap().iter().map(|x| reverse_lookup.get(x).unwrap().clone()).flatten().map(|x| (x, None)));

    for i in 1u64.. {
        let res = press_button(i, &mut modules, &mut next_visits, &mut leads_to_rx);
        low_pulse_count += res.0;
        high_pulse_count += res.1;

        if i == 1000 { // part a
            dbg!(low_pulse_count * high_pulse_count);
        }

        if i % 10_000 == 0 {
            println!("Cycle {i}");
        }

        if leads_to_rx.iter().all(|(_name, cycle_count)| cycle_count.is_some()) {
            let cycle = leads_to_rx.iter().fold(1u64, |acc, (_name, cycle_count)| lcm(acc, cycle_count.unwrap()));
            dbg!(cycle);
            break;
        }

        if next_visits.len() == 0 {
            next_visits.push(("button".to_string(), "broadcaster".to_string(), Pulse::Low));
        }
    }
}

fn press_button(cycle_count: u64, modules: &mut HashMap<String, Module>, next_visits: &mut Vec<(String, String, Pulse)>, leads_to_rx: &mut HashMap<String, Option<u64>>) -> (u64, u64) {
    let mut low_pulses = 0;
    let mut high_pulses = 0;
    while next_visits.len() > 0 {
        for visit in std::mem::take(next_visits).iter() {
            match visit.2 {
                Pulse::Low => low_pulses += 1,
                Pulse::High => high_pulses += 1,
            }
            if visit.2.low() && leads_to_rx.contains_key(&visit.1) && leads_to_rx.get(&visit.1).unwrap().is_none() {
                leads_to_rx.insert(visit.1.clone(), Some(cycle_count));
            }

            let Some(module) = modules.get_mut(&visit.1) else {
                // println!("Failed to find module \"{}\" to deliver Pulse::{:?}", visit.1, visit.2);
                continue;
            };
            let Some(output) = module.module_type.push_input(visit.2, &visit.0) else {
                // println!("No output generated from module {} with input {:?} (originally generated by {})", visit.1, visit.2, visit.0);
                continue;
            };
            for downstream_module in module.connects_to.iter() {
                // println!("Pushing Pulse::{output:?} from {} to {downstream_module}", visit.1);
                next_visits.push((visit.1.clone(), downstream_module.clone(), output));
            }
        }
    }
    (low_pulses, high_pulses)
}

fn build_module_tree_from_input(input: &str) -> (HashMap<String, Module>, HashMap<String, Vec<String>>) {
    let mut modules: HashMap<String, Module> = HashMap::new();
    let mut reverse_lookup: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.lines() {
        let module_type = match line.chars().next().unwrap() {
            '%' => ModuleType::FlipFlow(FlipFlowState::Off),
            '&' => ModuleType::Conjunction(ConjunctionState {inputs: HashMap::new()}),
            'b' => ModuleType::Broadcaster,
            _ => panic!("Invalid module type"),
        };

        let mut split = line.split(" -> ");
        let name = if matches!(module_type, ModuleType::Broadcaster) {
            split.next();
            "broadcaster"
        } else {
            &split.next().unwrap()[1..]
        };

        let connects_to = split.next().unwrap().split(", ").map(|s| s.to_string()).collect_vec();

        for downstream_module in connects_to.iter() {
            reverse_lookup.entry(downstream_module.clone()).or_default().push(name.to_string());
        }

        modules.insert(name.to_string(), Module {
            module_type,
            connects_to: connects_to.clone()
        });
    }

    for (module_name, inputs) in reverse_lookup.iter() {
        let Some(module) = modules.get_mut(module_name.as_str()) else {
            continue;
        };
        match module.module_type {
            ModuleType::Conjunction(ref mut state) => {
                for input in inputs.clone() {
                    state.inputs.insert(input, Pulse::Low);
                }
            },
            _ => (),
        }
    }

    (modules, reverse_lookup)
}