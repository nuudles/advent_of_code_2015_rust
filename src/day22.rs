use std::ops::Add;
use pathfinding::prelude::dijkstra;

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> (u64, u64) {
    let mut lines = input.lines();
    let boss_hp = lines
        .next()
        .expect("HP line not found")
        .replace("Hit Points: ", "")
        .parse()
        .unwrap_or(0);
    let boss_damage = lines
        .next()
        .expect("Damage line not found")
        .replace("Damage: ", "")
        .parse()
        .unwrap_or(0);
    (boss_hp, boss_damage)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct State {
    player_hp: u64,
    player_mana: u64,
    boss_hp: u64,
    shield_timer: u64,
    poison_timer: u64,
    recharge_timer: u64,
    is_player_turn: bool
}

impl State {
    fn next(&self, boss_damage: u64, hard_mode: bool) -> Vec<(State, u64)> {
        if self.player_hp == 0 {
            return vec![];
        }

        let mut result = Vec::new();
        if self.is_player_turn {
            if hard_mode && self.player_hp == 1 {
                return vec![];
            }

            if self.player_mana >= 53 {
                // Magic Missile
                result.push(
                    (
                        State {
                            player_hp: self.player_hp
                                .saturating_sub(if hard_mode { 1 } else { 0 }),
                            player_mana: self.player_mana
                                .add(if self.recharge_timer > 0 { 101 } else { 0 })
                                .saturating_sub(53),
                            boss_hp: self.boss_hp
                                .saturating_sub(if self.poison_timer > 0 { 3 } else { 0 })
                                .saturating_sub(4),
                            shield_timer: self.shield_timer.saturating_sub(1),
                            poison_timer: self.poison_timer.saturating_sub(1),
                            recharge_timer: self.recharge_timer.saturating_sub(1),
                            is_player_turn: false
                        },
                        53
                    )
                );
            }
            if self.player_mana >= 73 {
                // Drain
                result.push(
                    (
                        State {
                            player_hp: self.player_hp
                                .saturating_sub(if hard_mode { 1 } else { 0 })
                                .add(2),
                            player_mana: self.player_mana
                                .add(if self.recharge_timer > 0 { 101 } else { 0 })
                                .saturating_sub(73),
                            boss_hp: self.boss_hp
                                .saturating_sub(if self.poison_timer > 0 { 3 } else { 0 })
                                .saturating_sub(2),
                            shield_timer: self.shield_timer.saturating_sub(1),
                            poison_timer: self.poison_timer.saturating_sub(1),
                            recharge_timer: self.recharge_timer.saturating_sub(1),
                            is_player_turn: false
                        },
                        73
                    )
                );
            }
            if self.player_mana >= 113 && self.shield_timer <= 1 {
                // Shield
                result.push(
                    (
                        State {
                            player_hp: self.player_hp
                                .saturating_sub(if hard_mode { 1 } else { 0 }),
                            player_mana: self.player_mana
                                .add(if self.recharge_timer > 0 { 101 } else { 0 })
                                .saturating_sub(113),
                            boss_hp: self.boss_hp.saturating_sub(if self.poison_timer > 0 { 3 } else { 0 }),
                            shield_timer: 6,
                            poison_timer: self.poison_timer.saturating_sub(1),
                            recharge_timer: self.recharge_timer.saturating_sub(1),
                            is_player_turn: false
                        },
                        113
                    )
                );
            }
            if self.player_mana >= 173 && self.poison_timer <= 1 {
                // Poison
                result.push(
                    (
                        State {
                            player_hp: self.player_hp
                                .saturating_sub(if hard_mode { 1 } else { 0 }),
                            player_mana: self.player_mana
                                .add(if self.recharge_timer > 0 { 101 } else { 0 })
                                .saturating_sub(173),
                            boss_hp: self.boss_hp.saturating_sub(if self.poison_timer > 0 { 3 } else { 0 }),
                            shield_timer: self.shield_timer.saturating_sub(1),
                            poison_timer: 6,
                            recharge_timer: self.recharge_timer.saturating_sub(1),
                            is_player_turn: false
                        },
                        173
                    )
                );
            }
            if self.player_mana >= 229 && self.recharge_timer <= 1 {
                // Recharge
                result.push(
                    (
                        State {
                            player_hp: self.player_hp
                                .saturating_sub(if hard_mode { 1 } else { 0 }),
                            player_mana: self.player_mana
                                .add(if self.recharge_timer > 0 { 101 } else { 0 })
                                .saturating_sub(229),
                            boss_hp: self.boss_hp.saturating_sub(if self.poison_timer > 0 { 3 } else { 0 }),
                            shield_timer: self.shield_timer.saturating_sub(1),
                            poison_timer: self.poison_timer.saturating_sub(1),
                            recharge_timer: 5,
                            is_player_turn: false
                        },
                        229
                    )
                );
            }
        } else {
            result.push(
                (
                    State {
                        player_hp: self.player_hp
                            .saturating_sub(
                                boss_damage - if self.shield_timer > 0 { 7 } else { 0 }
                            ),
                        player_mana: self.player_mana.add(if self.recharge_timer > 0 { 101 } else { 0 }),
                        boss_hp: self.boss_hp.saturating_sub(if self.poison_timer > 0 { 3 } else { 0 }),
                        shield_timer: self.shield_timer.saturating_sub(1),
                        poison_timer: self.poison_timer.saturating_sub(1),
                        recharge_timer: self.recharge_timer.saturating_sub(1),
                        is_player_turn: true
                    },
                    0
                )
            );
        }
        result
    }
}

#[aoc(day22, part1)]
pub fn part1(boss_stats: &(u64, u64)) -> u64 {
    let (boss_hp, boss_damage) = *boss_stats;
    let path = dijkstra(
        &State {
            player_hp: 50,
            player_mana: 500,
            boss_hp: boss_hp,
            shield_timer: 0,
            poison_timer: 0,
            recharge_timer: 0,
            is_player_turn: true
        },
        |state| state.next(boss_damage, false),
        |state| state.boss_hp == 0
    );
    path.expect("Best path not found").1
}

#[aoc(day22, part2)]
pub fn part2(boss_stats: &(u64, u64)) -> u64 {
    let (boss_hp, boss_damage) = *boss_stats;
    let path = dijkstra(
        &State {
            player_hp: 50,
            player_mana: 500,
            boss_hp: boss_hp,
            shield_timer: 0,
            poison_timer: 0,
            recharge_timer: 0,
            is_player_turn: true
        },
        |state| state.next(boss_damage, true),
        |state| state.boss_hp == 0
    );
    path.expect("Best path not found").1
}
