use aoc_2020::get_input;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;

type Ticket = Vec<u32>;

fn main() {
    let input = &get_input(16);
    let (specification, my_ticket, nearby_tickets) = parse_input(input);

    dbg!(part_1(specification.clone(), &nearby_tickets));
    dbg!(part_2(specification, my_ticket, &nearby_tickets));
}

fn part_1(specification: TicketSpecification, nearby_tickets: &[Ticket]) -> u32 {
    nearby_tickets
        .iter()
        .filter_map(|x| specification.error_rate(x))
        .sum::<u32>()
}

fn part_2(specification: TicketSpecification, my_ticket: Ticket, nearby_tickets: &[Ticket]) -> u64 {
    let valid_tickets = nearby_tickets
        .iter()
        .filter(|x| specification.error_rate(x).is_none())
        .cloned()
        .collect_vec();

    let rule_count = valid_tickets[0].len();
    let mut final_field_positions = vec![None; rule_count];

    // make Map[field_name => Set[legal_index]] of possible positions for each rule
    // by checking against columns of ticket fields
    let mut valid_field_positions = specification
        .rules
        .iter()
        .map(|(field_name, rule)| {
            (
                field_name,
                (0..rule_count)
                    .filter(|idx| {
                        valid_tickets
                            .iter()
                            .all(|ticket| rule.in_range(ticket[*idx]))
                    })
                    .collect::<HashSet<_>>(),
            )
        })
        .collect::<HashMap<_, _>>();

    // loop and lock in rules that can be in one position only
    // then remove that possible position from all the other rules
    // example:
    //   rule a can only be in pos 1,2,3
    //   rule b can only be in pos 1
    //   rule c can only be in pos 1,2
    //   we first lock in rule b into pos 1
    //   < --- loop --- >
    //   then rule a can only be in pos 2,3 (as 1 is now c)
    //   and rule c can only be in pos 2, so we lock that in
    //   < --- loop --- >
    //   then rule a can only be in pos 3
    while final_field_positions.iter().any(|x| x.is_none()) {
        let (field_name, final_position) = valid_field_positions
            .iter()
            .filter(|(_, possible_positions)| possible_positions.len() == 1)
            .next()
            .map(|(field_name, possible_positions)| {
                (
                    **field_name,
                    // extract only possible position, verified above
                    *possible_positions.into_iter().next().unwrap(),
                )
            })
            .unwrap();

        final_field_positions[final_position] = Some(field_name);

        for possible_positions in valid_field_positions.iter_mut() {
            possible_positions.1.remove(&final_position);
        }
    }

    my_ticket
        .into_iter()
        .enumerate()
        .filter(|(idx, _number)| {
            final_field_positions[*idx]
                .unwrap()
                .starts_with("departure ")
        })
        .map(|(_idx, number)| number as u64)
        .product()
}

#[derive(Debug, Clone)]
struct TicketSpecification<'a> {
    rules: HashMap<&'a str, TicketRule>,
}

impl TicketSpecification<'_> {
    fn error_rate(&self, ticket: &[u32]) -> Option<u32> {
        ticket
            .into_iter()
            .copied()
            .filter(|number| !self.rules.values().any(|rule| rule.in_range(*number)))
            .reduce(|x, y| x + y)
    }
}

#[derive(Debug, Clone)]
struct TicketRule {
    ranges: Vec<RangeInclusive<u32>>,
}

impl TicketRule {
    fn in_range(&self, number: u32) -> bool {
        self.ranges.iter().any(|range| range.contains(&number))
    }
}

fn parse_input(input: &str) -> (TicketSpecification, Ticket, Vec<Ticket>) {
    let (rules, my_ticket, nearby_tickets) = input.split("\n\n").next_tuple().unwrap();
    let rules = rules
        .lines()
        .map(|rule| {
            let (field_name, ranges) = rule.split_once(": ").unwrap();
            let ranges = ranges
                .split(" or ")
                .map(|range| {
                    let (lower, upper) = range.split_once("-").unwrap();
                    lower.parse().unwrap()..=upper.parse().unwrap()
                })
                .collect_vec();
            (field_name, TicketRule { ranges })
        })
        .collect();

    let my_ticket = my_ticket
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect_vec();

    let nearby_tickets = nearby_tickets
        .lines()
        .skip(1)
        .map(|ticket| {
            ticket
                .split(",")
                .map(|x| x.parse::<u32>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    (TicketSpecification { rules }, my_ticket, nearby_tickets)
}
