#![feature(assert_matches)]

use aoc_2015::get_input;
use lazy_regex::regex_captures;
use pathfinding::prelude::dijkstra;

const DRAIN_POINTS: u32 = 2;
const MAGIC_MISSILE_DAMAGE: u32 = 4;
const SHIELD_ARMOR: u32 = 7;
const RECHARGE_MANA: u32 = 101;
const POISON_DAMAGE: u32 = 3;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct BattleState {
    player: Player,
    boss: Boss,
    spell_state: SpellState,
    hard_mode: bool,
}

impl BattleState {
    fn armor(&self) -> Option<u32> {
        if self.spell_state.shield_timer > 0 {
            Some(SHIELD_ARMOR)
        } else {
            None
        }
    }

    fn apply_effects(&mut self) {
        if self.spell_state.poison_timer > 0 {
            self.spell_state.poison_timer -= 1;
            self.boss.hit_points -= POISON_DAMAGE as i32;
        }
        if self.spell_state.recharge_timer > 0 {
            self.spell_state.recharge_timer -= 1;
            self.player.mana_remaining += RECHARGE_MANA as i32;
        }
        if self.spell_state.shield_timer > 0 {
            self.spell_state.shield_timer -= 1;
        }
    }

    fn check_result(&self) -> Option<TurnResult> {
        if self.player.hit_points <= 0 {
            return Some(TurnResult::PlayerDied);
        }
        if self.player.mana_remaining <= 0 {
            return Some(TurnResult::PlayerRanOutOfMana);
        }
        if self.boss.hit_points <= 0 {
            return Some(TurnResult::BossDied);
        }
        None
    }

    fn cast(&mut self, spell: Spell) -> Option<TurnResult> {
        if self.player.mana_remaining <= spell.cost() as i32 {
            return Some(TurnResult::PlayerRanOutOfMana);
        }
        match spell {
            Spell::MagicMissile => {
                self.boss.hit_points -= MAGIC_MISSILE_DAMAGE as i32;
            }
            Spell::Drain => {
                self.boss.hit_points -= DRAIN_POINTS as i32;
                self.player.hit_points += DRAIN_POINTS as i32;
            }
            Spell::Shield => {
                if self.spell_state.shield_timer > 0 {
                    return Some(TurnResult::EffectAlreadyInUse);
                }
                self.spell_state.shield_timer = 6;
            }
            Spell::Poison => {
                if self.spell_state.poison_timer > 0 {
                    return Some(TurnResult::EffectAlreadyInUse);
                }
                self.spell_state.poison_timer = 6;
            }
            Spell::Recharge => {
                if self.spell_state.recharge_timer > 0 {
                    return Some(TurnResult::EffectAlreadyInUse);
                }
                self.spell_state.recharge_timer = 5;
            }
        }
        self.player.mana_remaining -= spell.cost() as i32;
        self.player.mana_spent += spell.cost();
        None
    }

    fn tick(&mut self, spell: Spell) -> Option<TurnResult> {
        // Player turn
        if self.hard_mode {
            self.player.hit_points -= 1;
        }

        if let Some(result) = self.check_result() {
            return Some(result);
        }
        self.apply_effects();
        if let Some(result) = self.cast(spell) {
            return Some(result);
        }
        if let Some(result) = self.check_result() {
            return Some(result);
        }

        // boss turn
        self.apply_effects();
        if let Some(result) = self.check_result() {
            return Some(result);
        }
        self.player.hit_points -= (self.boss.damage - self.armor().unwrap_or(0)).max(1) as i32;
        if let Some(result) = self.check_result() {
            return Some(result);
        }

        None
    }
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
enum TurnResult {
    EffectAlreadyInUse,
    PlayerDied,
    PlayerRanOutOfMana,
    BossDied,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Player {
    hit_points: i32,
    mana_remaining: i32,
    mana_spent: u32,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Boss {
    hit_points: i32,
    damage: u32,
}

#[derive(Default, Debug, Clone, Eq, Hash, PartialEq)]
struct SpellState {
    shield_timer: u32,
    recharge_timer: u32,
    poison_timer: u32,
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    fn cost(&self) -> u32 {
        use Spell::*;
        match self {
            MagicMissile => 53,
            Drain => 73,
            Shield => 113,
            Poison => 173,
            Recharge => 229,
        }
    }

    fn all() -> [Self; 5] {
        [
            Spell::MagicMissile,
            Spell::Drain,
            Spell::Shield,
            Spell::Poison,
            Spell::Recharge,
        ]
    }
}

fn main() {
    let input = get_input(22);
    let boss = parse_input(&input);

    dbg!(part_1(boss.clone()));
    dbg!(part_2(boss.clone()));
}

fn part_1(boss: Boss) -> u32 {
    let initial_state = BattleState {
        boss,
        player: Player {
            hit_points: 50,
            mana_remaining: 500,
            mana_spent: 0,
        },
        spell_state: SpellState::default(),
        hard_mode: false,
    };
    find_lowest_mana_cost_route(initial_state)
}

fn part_2(boss: Boss) -> u32 {
    let initial_state = BattleState {
        boss,
        player: Player {
            hit_points: 50,
            mana_remaining: 500,
            mana_spent: 0,
        },
        spell_state: SpellState::default(),
        hard_mode: true,
    };
    find_lowest_mana_cost_route(initial_state)
}

fn find_lowest_mana_cost_route(initial_state: BattleState) -> u32 {
    dijkstra(
        &(initial_state, None, Spell::MagicMissile),
        |(state, _result, _spell)| next_state(state),
        |(_, result, _)| matches!(result, Some(TurnResult::BossDied)),
    )
    .unwrap()
    .1
}

fn next_state(
    state: &BattleState,
) -> impl Iterator<Item = ((BattleState, Option<TurnResult>, Spell), u32)> {
    let state = state.clone();
    Spell::all()
        .into_iter()
        .map(move |spell| {
            let mut state = state.clone();
            let result = state.tick(spell);
            ((state, result, spell), spell.cost())
        })
        .filter(|((_state, result, _spell), _spell_cost)| {
            if let Some(result) = result {
                matches!(result, TurnResult::BossDied)
            } else {
                true
            }
        })
}

fn parse_input(input: &str) -> Boss {
    let (boss_hp, boss_dmg) = regex_captures!(r"^Hit Points: (\d+)\nDamage: (\d+)\n?$", input)
        .map(|(_, hp, dmg)| (hp.parse::<u32>().unwrap(), dmg.parse::<u32>().unwrap()))
        .unwrap();
    Boss {
        hit_points: boss_hp as i32,
        damage: boss_dmg,
    }
}

#[test]
fn example_1() {
    use std::assert_matches::assert_matches;

    let spells = [Spell::Poison, Spell::MagicMissile];

    let mut state = BattleState {
        boss: Boss {
            hit_points: 13,
            damage: 8,
        },
        player: Player {
            hit_points: 10,
            mana_remaining: 250,
            mana_spent: 0,
        },
        spell_state: SpellState::default(),
        hard_mode: false,
    };

    let mut result = None;
    for spell in spells.into_iter() {
        result = state.tick(spell);
    }

    assert!(state.boss.hit_points <= 0);
    assert_matches!(result, Some(TurnResult::BossDied));
    assert_eq!(state.player.mana_remaining, 24);
    assert_eq!(state.player.hit_points, 2);
}

#[test]
fn example_2() {
    use std::assert_matches::assert_matches;

    let spells = [
        Spell::Recharge,
        Spell::Shield,
        Spell::Drain,
        Spell::Poison,
        Spell::MagicMissile,
    ];

    let mut state = BattleState {
        boss: Boss {
            hit_points: 14,
            damage: 8,
        },
        player: Player {
            hit_points: 10,
            mana_remaining: 250,
            mana_spent: 0,
        },
        spell_state: SpellState::default(),
        hard_mode: false,
    };

    let mut result = None;
    for spell in spells.into_iter() {
        result = state.tick(spell);
    }

    assert!(state.boss.hit_points <= 0);
    assert_matches!(result, Some(TurnResult::BossDied));
    assert_eq!(state.player.hit_points, 1);
    assert_eq!(state.player.mana_remaining, 114);
}
