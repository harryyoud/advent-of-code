use std::fmt;

use aoc_2023::get_input;
use itertools::Itertools;

fn main() {
    let input = get_input(7);
    let mut hands_a: Vec<(Hand, u32)> = vec![];
    let mut hands_b: Vec<(Hand, u32)> = vec![];
    for line in input.lines() {
        let (cards, bid) = {
            let (cards, bid) = line.split_whitespace().collect_tuple().unwrap();
            let bid = bid.parse::<u32>().unwrap();
            let cards = cards.chars().map(|x| Card::parse(&x).unwrap()).collect_vec();
            (cards, bid)
        };
        let hand_type_a = calculate_rank(&cards);
        let hand_a = Hand { cards: cards.clone(), hand_type: hand_type_a };
        let cards = cards.into_iter().map(|x| if matches!(x, Card::Jack) { Card::Joker } else { x }).collect_vec();
        let hand_type_b = calculate_best_rank(&cards);
        let hand_b = Hand { cards, hand_type: hand_type_b };
        hands_a.push((hand_a, bid));
        hands_b.push((hand_b, bid));
    }
    dbg!(get_total_value(hands_a));
    dbg!(get_total_value(hands_b));
}

fn get_total_value(mut hands: Vec<(Hand, u32)>) -> u32 {
    hands.sort_by(|a, b| a.0.cmp(&b.0));
    let mut total = 0u32;
    for (idx, (_, bid)) in hands.into_iter().enumerate() {
        total += (idx + 1) as u32 * bid;
    }
    total
}

fn calculate_rank(cards: &Vec<Card>) -> HandRank {
    let counts = cards.iter().copied().counts();
    return match (
        cards.iter().unique().count(),
        counts.values().min().unwrap(),
        counts.values().max().unwrap()
    ){
        (1, _, _) => HandRank::FiveOfAKind,
        (2, _, 4) => HandRank::FourOfAKind,
        (2, 2, 3) => HandRank::FullHouse,
        (3, 1, 3) => HandRank::ThreeOfAKind,
        (3, 1, 2) => HandRank::TwoPair,
        (4, _, _) => HandRank::OnePair,
        (5, _, _) => HandRank::HighCard,
        _ => unreachable!(),
    };
}

fn calculate_best_rank(cards: &Vec<Card>) -> HandRank {
    let joker_count = cards.iter().filter(|x| *x == &Card::Joker).count();
    if joker_count == 0 {
        return calculate_rank(cards);
    };
    CARDS.iter().filter(|x| !matches!(x, Card::Jack)).map(|possible_card| {
        calculate_rank(&cards.clone().into_iter().map(|x|
            if x == Card::Joker { possible_card.clone() } else { x }
        ).collect_vec())
    }).max().unwrap()
}

pub const CARDS: [Card; 13] = [
    Card::Number(2),
    Card::Number(3),
    Card::Number(4),
    Card::Number(5),
    Card::Number(6),
    Card::Number(7),
    Card::Number(8),
    Card::Number(9),
    Card::Number(10),
    Card::Jack,
    Card::Queen,
    Card::King,
    Card::Ace,
];

#[derive(PartialEq, Eq, Debug)]
pub struct Hand {
    pub hand_type: HandRank,
    pub cards: Vec<Card>,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type.eq(&other.hand_type) {
            return self.cards.cmp(&other.cards);
        }
        return self.hand_type.cmp(&other.hand_type);
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum Card {
    Number(u8),
    Ace,
    King,
    Queen,
    Jack,
    Joker,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Card {{{}}}", match &self {
            Card::Number(n) => match n {
                2..=9 => n.to_string().chars().next().unwrap(),
                10 => 'T',
                _ => panic!("Impossible card {}", n),
            },
            Card::Ace => 'A',
            Card::King => 'K',
            Card::Queen => 'Q',
            Card::Jack => 'J',
            Card::Joker => 'J',
        })
    }
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Card {
    pub fn parse(cha: &char) -> Result<Self, ()> {
        Ok(match cha {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Jack,
            'T' => Self::Number(10),
            '2'..='9' => Self::Number(cha.to_string().parse().unwrap()),
            _ => return Err(()),
        })
    }
    fn _as_number(&self) -> u8 {
        match self {
            Card::Ace => 14,
            Card::King => 13,
            Card::Queen => 12,
            Card::Jack => 11,
            Card::Number(n) => *n,
            Card::Joker => 0,
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self._as_number().cmp(&other._as_number())
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum HandRank {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandRank {
    fn _as_number(&self) -> u8 {
        match self {
            HandRank::FiveOfAKind => 6,
            HandRank::FourOfAKind => 5,
            HandRank::FullHouse => 4,
            HandRank::ThreeOfAKind => 3,
            HandRank::TwoPair => 2,
            HandRank::OnePair => 1,
            HandRank::HighCard => 0,
        }
    }
}

impl PartialOrd for HandRank {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for HandRank {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self._as_number().cmp(&other._as_number())
    }
}
