use itertools::iproduct;
use itertools::Itertools;

fn did_player_win(player_stats: (i64, i64, i64), boss_stats: (i64, i64, i64)) -> bool {
    let (mut player_hp, player_damage, player_armor) = player_stats;
    let (mut boss_hp, boss_damage, boss_armor) = boss_stats;
    let mut turn = 0;
    while player_hp > 0 && boss_hp > 0 {
        if turn == 0 {
            boss_hp -= player_damage - boss_armor;
        } else {
            player_hp -= boss_damage - player_armor;
        }
        turn = 1 - turn;
    }
    player_hp > 0
}

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> (i64, i64, i64) {
    let mut lines = input.lines();
    let boss_hp = lines
        .next()
        .expect("HP line not found")
        .replace("Hit Points: ", "")
        .parse()
        .unwrap_or(0);
    let boss_damage = lines
        .next()
        .expect("HP line not found")
        .replace("Damage: ", "")
        .parse()
        .unwrap_or(0);
    let boss_armor = lines
        .next()
        .expect("HP line not found")
        .replace("Armor: ", "")
        .parse()
        .unwrap_or(0);
    (boss_hp, boss_damage, boss_armor)
}

#[aoc(day21, part1)]
pub fn part1(boss_stats: &(i64, i64, i64)) -> i64 {
    let weapons: Vec<(i64, i64, i64)> = vec![
        (8, 4, 0),
        (10, 5, 0),
        (25, 6, 0),
        (40, 7, 0),
        (74, 8, 0),
    ];
    let armor: Vec<(i64, i64, i64)> = vec![
        (0, 0, 0), // No armor
        (13, 0, 1),
        (31, 0, 2),
        (53, 0, 3),
        (75, 0, 4),
        (102, 0, 5),
    ];
    let rings: Vec<(i64, i64, i64)> = vec![
        (0, 0, 0), // No ring
        (0, 0, 0), // No ring
        (25, 1, 0),
        (50, 2, 0),
        (100, 3, 0),
        (20, 0, 1),
        (40, 0, 2),
        (80, 0, 3),
    ];

    let costs = iproduct!(
            weapons, 
            armor, 
            rings.iter().combinations(2).map(|c| (c[0].0 + c[1].0, c[0].1 + c[1].1, c[0].2 + c[1].2))
        )
        .filter(|(weapon, armor, rings)|
            did_player_win(
                (100, weapon.1 + rings.1, armor.2 + rings.2),
                *boss_stats
            )
        )
        .map(|(weapon, armor, rings)| weapon.0 + armor.0 + rings.0);
    costs.min().expect("No minimum cost found")
}

#[aoc(day21, part2)]
pub fn part2(boss_stats: &(i64, i64, i64)) -> i64 {
    let weapons: Vec<(i64, i64, i64)> = vec![
        (8, 4, 0),
        (10, 5, 0),
        (25, 6, 0),
        (40, 7, 0),
        (74, 8, 0),
    ];
    let armor: Vec<(i64, i64, i64)> = vec![
        (0, 0, 0), // No armor
        (13, 0, 1),
        (31, 0, 2),
        (53, 0, 3),
        (75, 0, 4),
        (102, 0, 5),
    ];
    let rings: Vec<(i64, i64, i64)> = vec![
        (0, 0, 0), // No ring
        (0, 0, 0), // No ring
        (25, 1, 0),
        (50, 2, 0),
        (100, 3, 0),
        (20, 0, 1),
        (40, 0, 2),
        (80, 0, 3),
    ];

    let costs = iproduct!(
            weapons, 
            armor, 
            rings.iter().combinations(2).map(|c| (c[0].0 + c[1].0, c[0].1 + c[1].1, c[0].2 + c[1].2))
        )
        .filter(|(weapon, armor, rings)|
            !did_player_win(
                (100, weapon.1 + rings.1, armor.2 + rings.2),
                *boss_stats
            )
        )
        .map(|(weapon, armor, rings)| weapon.0 + armor.0 + rings.0);
    costs.max().expect("No maximum cost found")
}
