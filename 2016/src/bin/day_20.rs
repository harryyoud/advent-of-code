use aoc_2016::get_input;
use rangemap::RangeInclusiveMap;

fn main() {
    let input = get_input(20);
    let blocked = parse_input(&input);

    dbg!(part_1(&blocked));
    dbg!(part_2(&blocked));
}

fn part_1(blocked: &Firewall) -> Ip {
    *blocked
        .iter()
        .filter(|(_range, status)| status.allowed())
        .map(|(range, _status)| range.start())
        .next()
        .unwrap()
}

fn part_2(blocked: &Firewall) -> u32 {
    blocked
        .iter()
        .filter(|(_range, status)| status.allowed())
        .map(|(range, _status)| range.end() - range.start() + 1)
        .sum::<u32>()
}

fn parse_input(input: &str) -> Firewall {
    let mut map = RangeInclusiveMap::new();
    map.insert(0..=u32::MAX, Status::Allowed);
    input.lines().for_each(|line| {
        let (lower, upper) = line.split_once('-').unwrap();
        let (lower, upper) = (lower.parse().unwrap(), upper.parse().unwrap());
        map.insert(lower..=upper, Status::Blocked);
    });
    map
}

type Ip = u32;
type Firewall = RangeInclusiveMap<Ip, Status>;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Status {
    Allowed,
    Blocked,
}

impl Status {
    fn allowed(&self) -> bool {
        matches!(self, Self::Allowed)
    }
}
