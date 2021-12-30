use regex::Regex;
use itertools::iproduct;
use std::cmp::max;

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<(i64, i64, i64, i64, i64)> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(.*): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)")
                .expect("Error creating Regex");
    }
    input
        .lines()
        .filter_map(|l| RE.captures(l))
        .fold(Vec::new(), |mut result, captures| {
            let capacity = captures.get(2).expect("No capacity found").as_str().parse::<i64>().unwrap_or(0);
            let durability = captures.get(3).expect("No durability found").as_str().parse::<i64>().unwrap_or(0);
            let flavor = captures.get(4).expect("No flavor found").as_str().parse::<i64>().unwrap_or(0);
            let texture = captures.get(5).expect("No texture found").as_str().parse::<i64>().unwrap_or(0);
            let calories = captures.get(6).expect("No calories found").as_str().parse::<i64>().unwrap_or(0);

            result.push((capacity, durability, flavor, texture, calories));
            result
        })
}

#[aoc(day15, part1)]
pub fn part1(input: &Vec<(i64, i64, i64, i64, i64)>) -> i64 {
    let counts = iproduct!(0..=100, 0..=100, 0..=100, 0..=100)
        .filter(|(a, b, c, d)| a + b + c + d == 100)
        .map(|(a, b, c, d)| {
            let count = vec![a, b, c, d];
            let capacity = max(count.iter().zip(input.iter().map(|x| x.0)).map(|(c, v)| c * v).sum(), 0);
            let durability = max(count.iter().zip(input.iter().map(|x| x.1)).map(|(c, v)| c * v).sum(), 0);
            let flavor = max(count.iter().zip(input.iter().map(|x| x.2)).map(|(c, v)| c * v).sum(), 0);
            let texture = max(count.iter().zip(input.iter().map(|x| x.3)).map(|(c, v)| c * v).sum(), 0);
            capacity * durability * flavor * texture
        })
        .filter(|x| *x > 0);
    counts.max().unwrap_or(0)
}

#[aoc(day15, part2)]
pub fn part2(input: &Vec<(i64, i64, i64, i64, i64)>) -> i64 {
    let counts = iproduct!(0..=100, 0..=100, 0..=100, 0..=100)
        .filter(|(a, b, c, d)| a + b + c + d == 100)
        .filter(|(a, b, c, d)| input[0].4 * a + input[1].4 * b + input[2].4 * c + input[3].4 * d == 500)
        .map(|(a, b, c, d)| {
            let count = vec![a, b, c, d];
            let capacity = max(count.iter().zip(input.iter().map(|x| x.0)).map(|(c, v)| c * v).sum(), 0);
            let durability = max(count.iter().zip(input.iter().map(|x| x.1)).map(|(c, v)| c * v).sum(), 0);
            let flavor = max(count.iter().zip(input.iter().map(|x| x.2)).map(|(c, v)| c * v).sum(), 0);
            let texture = max(count.iter().zip(input.iter().map(|x| x.3)).map(|(c, v)| c * v).sum(), 0);
            capacity * durability * flavor * texture
        })
        .filter(|x| *x > 0);
    counts.max().unwrap_or(0)
}
