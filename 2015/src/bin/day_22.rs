use aoc_2015::get_input;
use itertools::Itertools;

const SPELLS: [Spell; 5] = [
    Spell { instant_damage: 4, mana_cost: 53, heals: 0, effect: None },
    Spell { instant_damage: 2, mana_cost: 73, heals: 2, effect: None },
    Spell { instant_damage: 0, mana_cost: 113, heals: 0, effect: Some(Effect { effect_type: EffectType::Shield, remaining_turns: 6, amount: 7 })},
    Spell { instant_damage: 0, mana_cost: 173, heals: 0, effect: Some(Effect { effect_type: EffectType::Damage, remaining_turns: 6, amount: 3 })},
    Spell { instant_damage: 0, mana_cost: 229, heals: 0, effect: Some(Effect { effect_type: EffectType::Recharge, remaining_turns: 5, amount: 101 }) }
];

fn main() {
    let input = get_input(22);
    let (boss_hp, boss_damage) = parse_input(&input);

    let boss = Boss {
        hit_points: boss_hp,
        damage: boss_damage,
        effects: vec![],
    };
    let player = Player {
        hit_points: 50,
        mana: 500,
        armour: 0,
        shield: 0,
        effects: vec![],
    };
    let mut min_mana = u64::MAX;
    get_outcome(&boss, &player, &mut min_mana, 0);
    dbg!(min_mana);
}

fn parse_input(input: &str) -> (i64, u64) {
    let lines = input.lines()
        .map(|line| line.split_once(": ").unwrap().1)
        .collect_tuple::<(_, _)>()
        .unwrap();

    (
        lines.0.parse().unwrap(),
        lines.1.parse().unwrap(),
    )
}

fn get_outcome(boss: &Boss, player: &Player, min_mana: &mut u64, used_mana: u64) {
    for spell in SPELLS.iter() {
        let mut boss = boss.clone();
        let mut player = player.clone();

        if player.mana < (spell.mana_cost + used_mana) || used_mana > *min_mana {
            continue;
        }
        player.tick();
        boss.tick();
        if boss.is_dead() {
            println!("Boss died {used_mana}");
            *min_mana = (*min_mana).min(used_mana + spell.mana_cost);
        }
        if let Some(effect) = spell.effect.as_ref() {
            if player.add_effect(effect.clone()).is_err() {
                continue;
            };
            if boss.add_effect(effect.clone()).is_err() {
                continue;
            }
        }
        if spell.instant_damage > 0 {
            boss.hit_points -= spell.instant_damage as i64;
        }
        if spell.heals > 0 {
            player.hit_points += spell.heals as i64;
        }
        if boss.is_dead() {
            println!("Boss died {used_mana}");
            *min_mana = (*min_mana).min(used_mana + spell.mana_cost);
        }

        player.tick();
        boss.tick();
        if boss.is_dead() {
            println!("Boss died {used_mana}");
            *min_mana = (*min_mana).min(used_mana + spell.mana_cost);
        }
        player.hit_points -= (boss.damage.saturating_sub(player.armour + player.shield).max(1)) as i64;
        if player.is_dead() {
            continue;
        }

        get_outcome(&boss, &player, min_mana, used_mana + spell.mana_cost)
    }
}



#[derive(Clone)]
struct Boss {
    hit_points: i64,
    damage: u64,
    effects: Vec<Effect>,
}

impl Boss {
    fn tick(&mut self) {
        for effect in self.effects.iter_mut() {
            match effect.effect_type {
                EffectType::Damage => {
                    effect.remaining_turns = effect.remaining_turns.saturating_sub(1);
                    if effect.remaining_turns == 0 {
                        continue;
                    }
                    self.hit_points -= effect.amount as i64;
                },
                _ => {},
            }
        }
        self.effects.retain(|x| x.remaining_turns > 0);
    }

    fn add_effect(&mut self, effect: Effect) -> Result<(), ()> {
        if !self.can_add_effect_type(effect.effect_type) {
            return Err(());
        }
        self.effects.push(effect);
        Ok(())
    }

    fn can_add_effect_type(&self, effect_type: EffectType) -> bool {
        self.effects.iter().all(|x| x.effect_type != effect_type)
    }

    fn is_dead(&self) -> bool {
        self.hit_points <= 0
    }
}

#[derive(Clone)]
struct Player {
    hit_points: i64,
    mana: u64,
    armour: u64,
    shield: u64,
    effects: Vec<Effect>,
}

impl Player {
    fn tick(&mut self) {
        for effect in self.effects.iter_mut() {
            match effect.effect_type {
                EffectType::Shield => {
                    effect.remaining_turns = effect.remaining_turns.saturating_sub(1);
                    self.shield = effect.amount;
                    if effect.remaining_turns == 0 {
                        self.shield = 0;
                        continue;
                    }
                },
                EffectType::Recharge => {
                    effect.remaining_turns = effect.remaining_turns.saturating_sub(1);
                    self.mana += effect.amount;
                    if effect.remaining_turns == 0 {
                        continue;
                    }
                },
                _ => {}
            }
        }
        self.effects.retain(|x| x.remaining_turns > 0);
    }
    
    fn add_effect(&mut self, effect: Effect) -> Result<(), ()> {
        if !self.can_add_effect_type(effect.effect_type) {
            return Err(());
        }
        match effect.effect_type {
            EffectType::Shield => {
                self.shield = effect.amount;
            },
            _ => {},
        }
        self.effects.push(effect);
        Ok(())
    }

    fn can_add_effect_type(&self, effect_type: EffectType) -> bool {
        self.effects.iter().all(|x| x.effect_type != effect_type)
    }

    fn is_dead(&self) -> bool {
        self.hit_points <= 0
    }
}

#[derive(Clone)]
struct Effect {
    effect_type: EffectType,
    remaining_turns: u64,
    amount: u64,
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum EffectType {
    Shield,
    Damage,
    Recharge,
}

#[derive(Clone)]
struct Spell {
    instant_damage: u64,
    mana_cost: u64,
    heals: u64,
    effect: Option<Effect>,
}
