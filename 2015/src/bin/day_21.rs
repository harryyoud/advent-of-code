use std::iter;

use aoc_2015::get_input;
use aoc_lib::paragraphs::Paragraphs;
use itertools::Itertools;

const SHOP: &str = r#"Weapons:    Cost  Damage  Armor
Dagger        8     4       0
Shortsword   10     5       0
Warhammer    25     6       0
Longsword    40     7       0
Greataxe     74     8       0

Armor:      Cost  Damage  Armor
Leather      13     0       1
Chainmail    31     0       2
Splintmail   53     0       3
Bandedmail   75     0       4
Platemail   102     0       5

Rings:      Cost  Damage  Armor
Damage +1    25     1       0
Damage +2    50     2       0
Damage +3   100     3       0
Defense +1   20     0       1
Defense +2   40     0       2
Defense +3   80     0       3"#;

#[derive(Default, Clone)]
struct Character {
    hit_points: u64,
    damage: u64,
    armour: u64,
}

impl Character {
    fn apply(mut self, item: Option<&Item>) -> Self {
        let Some(item) = item else {
            return self;
        };
        self.damage += item.damage;
        self.armour += item.armour;
        self
    }
}

#[derive(Debug)]
struct Item {
    cost: u64,
    damage: u64,
    armour: u64,
}

enum Outcome {
    BossWins,
    PlayerWins,
}

fn main() {
    let input = get_input(21);
    let boss = parse_input(&input);
    let player = Character {
        hit_points: 100,
        ..Default::default()
    };

    let (weapons, armour, rings) = parse_shop();
    dbg!(part_1(&player, &boss, &weapons, &armour, &rings));
    dbg!(part_2(&player, &boss, &weapons, &armour, &rings));
}

fn part_1(
    player: &Character,
    boss: &Character,
    weapons: &[Item],
    armour: &[Item],
    rings: &[Item],
) -> u64 {
    let mut min_cost: u64 = u64::MAX;

    for (weapon, armour, rings) in get_shop_combinations(weapons, armour, rings) {
        let cost = weapon.cost
            + armour.map(|x| x.cost).unwrap_or(0)
            + rings.0.map(|x| x.cost).unwrap_or(0)
            + rings.1.map(|x| x.cost).unwrap_or(0);

        match get_fight_outcome(boss, player, weapon, armour, rings) {
            Outcome::BossWins => (),
            Outcome::PlayerWins => {
                min_cost = min_cost.min(cost);
            }
        };
    }

    min_cost
}

fn part_2(
    player: &Character,
    boss: &Character,
    weapons: &[Item],
    armour: &[Item],
    rings: &[Item],
) -> u64 {
    let mut max_cost: u64 = u64::MIN;

    for (weapon, armour, rings) in get_shop_combinations(weapons, armour, rings) {
        let cost = weapon.cost
            + armour.map(|x| x.cost).unwrap_or(0)
            + rings.0.map(|x| x.cost).unwrap_or(0)
            + rings.1.map(|x| x.cost).unwrap_or(0);

        match get_fight_outcome(boss, player, weapon, armour, rings) {
            Outcome::BossWins => {
                max_cost = max_cost.max(cost);
            }
            Outcome::PlayerWins => (),
        };
    }

    max_cost
}

fn get_shop_combinations<'a>(
    weapons: &'a [Item],
    armour: &'a [Item],
    rings: &'a [Item],
) -> impl Iterator<
    Item = (
        &'a Item,
        Option<&'a Item>,
        (Option<&'a Item>, Option<&'a Item>),
    ),
> {
    weapons
        .iter()
        .cartesian_product(armour.iter().map(Some).chain(iter::once(None)))
        .cartesian_product(
            rings
                .iter()
                .map(Some)
                .chain(iter::once(None))
                .chain(iter::once(None))
                .tuple_combinations::<(_, _)>(),
        )
        .map(|((weapon, armour), rings)| (weapon, armour, rings))
}

fn get_fight_outcome(
    boss: &Character,
    player: &Character,
    weapon: &Item,
    armour: Option<&Item>,
    rings: (Option<&Item>, Option<&Item>),
) -> Outcome {
    let mut player: Character = player
        .clone()
        .apply(Some(weapon))
        .apply(armour)
        .apply(rings.0)
        .apply(rings.1);
    let mut boss = boss.clone();

    loop {
        boss.hit_points = boss
            .hit_points
            .saturating_sub(player.damage.saturating_sub(boss.armour));
        if boss.hit_points == 0 {
            break Outcome::PlayerWins;
        }
        player.hit_points = player
            .hit_points
            .saturating_sub(boss.damage.saturating_sub(player.armour));
        if player.hit_points == 0 {
            break Outcome::BossWins;
        }
    }
}

fn parse_input(input: &str) -> Character {
    let mut c: Character = Character::default();
    for line in input.lines() {
        let (attribute, amount) = line.split(": ").collect_tuple().unwrap();
        let amount = amount.parse().unwrap();
        match attribute {
            "Hit Points" => c.hit_points = amount,
            "Damage" => c.damage = amount,
            "Armor" => c.armour = amount,
            _ => panic!("Invalid attribute: {attribute}"),
        }
    }
    c
}

fn parse_shop() -> (Vec<Item>, Vec<Item>, Vec<Item>) {
    let mut weapons = vec![];
    let mut armour = vec![];
    let mut rings = vec![];

    for paragraph in SHOP.paragraphs() {
        let mut lines = paragraph.into_iter();
        let v = match lines.next().unwrap().split_once(':').unwrap().0 {
            "Weapons" => &mut weapons,
            "Armor" => &mut armour,
            "Rings" => &mut rings,
            s => panic!("Invalid item type: {s}"),
        };
        for line in lines {
            v.push(parse_item(line));
        }
    }

    (weapons, armour, rings)
}

fn parse_item(line: &str) -> Item {
    let mut split = line.split_whitespace().collect_vec();
    Item {
        armour: split.pop().unwrap().parse().unwrap(),
        damage: split.pop().unwrap().parse().unwrap(),
        cost: split.pop().unwrap().parse().unwrap(),
        // name: split.join(" "),
    }
}
