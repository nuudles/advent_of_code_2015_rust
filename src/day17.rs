use itertools::Itertools;

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input
        .lines()
        .filter_map(|l| l.parse().ok())
        .collect()
}

#[aoc(day17, part1)]
pub fn part1(input: &Vec<u64>) -> usize {
    let combinations: Vec<_> = (1..input.len())
        .flat_map(|s| input.iter().combinations(s))
        .filter(|c| (*c).iter().map(|x| *x).sum::<u64>() == 150)
        .collect();
    combinations.len()
}

#[aoc(day17, part2)]
pub fn part2(input: &Vec<u64>) -> usize {
    let combinations: Vec<_> = (1..input.len())
        .flat_map(|s| input.iter().combinations(s))
        .filter(|c| (*c).iter().map(|x| *x).sum::<u64>() == 150)
        .collect();
    let min_count = combinations.iter().map(|c| c.len()).min().unwrap_or(0);
    combinations
        .iter()
        .filter(|c| c.len() == min_count)
        .collect::<Vec<_>>()
        .len()
}
