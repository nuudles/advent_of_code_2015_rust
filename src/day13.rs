use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;

pub fn input_generator(input: &str) -> HashMap<(&str, &str), i64> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(.*) would (lose|gain) (\d+) happiness units by sitting next to (.*).")
                .expect("Error creating Regex");
    }
    input
        .lines()
        .filter_map(|l| RE.captures(l))
        .fold(HashMap::new(), |mut result, captures| {
            let subject = captures.get(1).expect("No subject found").as_str();
            let multiplier = if captures.get(2).expect("No multiplier found").as_str() == "lose" { -1 } else { 1 };
            let happiness = captures
                .get(3)
                .expect("No happiness found")
                .as_str()
                .parse::<i64>()
                .unwrap_or(0) * multiplier;
            let target = captures.get(4).unwrap().as_str();

            result.insert((subject, target), happiness);
            result
        })
}

fn max_happiness(names: &HashSet<&str>, happiness_map: &HashMap<(&str, &str), i64>) -> i64 {
    let count = names.len();
    names
        .iter()
        .permutations(count)
        .filter_map(|permutation| {
            let mut sum = 0;
            for (i, n) in permutation.iter().enumerate() {
                let left = happiness_map.get(&(**n, permutation[if i > 0 { i - 1 } else { count - 1 }]))?;
                let right = happiness_map.get(&(**n, permutation[(i + 1) % count]))?;
                sum += left + right;
            }
            Some(sum)
        })
        .max()
        .unwrap_or(0)
}

#[aoc(day13, part1)]
pub fn part1(input: &str) -> i64 {
    let happiness_map = input_generator(input);
    let names = happiness_map
        .keys()
        .fold(HashSet::new(), |mut result, key| {
            result.insert(key.0);
            result.insert(key.1);
            result
        });
    max_happiness(&names, &happiness_map)
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> i64 {
    let mut happiness_map = input_generator(input);
    let mut names = happiness_map
        .keys()
        .fold(HashSet::new(), |mut result, key| {
            result.insert(key.0);
            result.insert(key.1);
            result
        });
    for n in &names {
        happiness_map.insert(("Me", n), 0);
        happiness_map.insert((n, "Me"), 0);
    }
    names.insert("Me");
    
    max_happiness(&names, &happiness_map)
}

#[cfg(test)]
mod tests {
    use super::{part1};

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(r"
Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol."
            ),
            330
        );
    }
}
