use itertools::Itertools;

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input.lines().filter_map(|l| l.parse().ok()).collect()
}

#[aoc(day24, part1)]
pub fn part1(input: &Vec<u64>) -> u64 {
    let target = input.iter().sum::<u64>() / 3;
    let group1 = input
        .iter()
        .combinations(6)
        .filter(|c| c.iter().map(|n| **n).sum::<u64>() == target)
        .map(|c| c.iter().map(|n| **n).product());
    group1.min().expect("No minimum found")
}

#[aoc(day24, part2)]
pub fn part2(input: &Vec<u64>) -> u64 {
    let target = input.iter().sum::<u64>() / 4;
    let group1 = input
        .iter()
        .combinations(4)
        .filter(|c| c.iter().map(|n| **n).sum::<u64>() == target)
        .map(|c| c.iter().map(|n| **n).product());
    group1.min().expect("No minimum found")
}
