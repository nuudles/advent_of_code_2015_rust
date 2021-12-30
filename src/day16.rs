use std::collections::HashMap;
use regex::Regex;

fn input_generator(input: &str) -> Vec<HashMap<&str, u64>> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"([a-z]+): (\d+)")
                .expect("Error creating Regex");
    }
    input
        .lines()
        .map(|l| RE.captures_iter(l))
        .fold(Vec::new(), |mut result, captures| {
            result
                .push(
                    captures
                        .fold(
                            HashMap::new(), 
                            |mut map, c| {
                                map.insert(
                                    c.get(1)
                                        .expect("No property name found")
                                        .as_str(),
                                    c.get(2)
                                        .expect("No value found")
                                        .as_str()
                                        .parse()
                                        .unwrap_or(0)
                                );
                                map
                            }
                        )
                );
            result
        })
}

#[aoc(day16, part1)]
pub fn part1(input: &str) -> usize {
    let sues = input_generator(input);
    sues
        .iter()
        .enumerate()
        .filter(|(_, s)| {
            *s.get("children").unwrap_or(&3) == 3 &&
                *s.get("cats").unwrap_or(&7) == 7 &&
                *s.get("samoyeds").unwrap_or(&2) == 2 &&
                *s.get("pomeranians").unwrap_or(&3) == 3 &&
                *s.get("akitas").unwrap_or(&0) == 0 &&
                *s.get("vizslas").unwrap_or(&0) == 0 &&
                *s.get("goldfish").unwrap_or(&5) == 5 &&
                *s.get("trees").unwrap_or(&3) == 3 &&
                *s.get("cars").unwrap_or(&2) == 2 &&
                *s.get("perfumes").unwrap_or(&1) == 1
        })
        .next()
        .expect("No Sue found...")
        .0 + 1
}

#[aoc(day16, part2)]
pub fn part2(input: &str) -> usize {
    let sues = input_generator(input);
    sues
        .iter()
        .enumerate()
        .filter(|(_, s)| {
            *s.get("children").unwrap_or(&3) == 3 &&
                *s.get("cats").unwrap_or(&8) > 7 &&
                *s.get("samoyeds").unwrap_or(&2) == 2 &&
                *s.get("pomeranians").unwrap_or(&0) < 3 &&
                *s.get("akitas").unwrap_or(&0) == 0 &&
                *s.get("vizslas").unwrap_or(&0) == 0 &&
                *s.get("goldfish").unwrap_or(&0) < 5 &&
                *s.get("trees").unwrap_or(&4) > 3 &&
                *s.get("cars").unwrap_or(&2) == 2 &&
                *s.get("perfumes").unwrap_or(&1) == 1
        })
        .next()
        .expect("No Sue found...")
        .0 + 1
}
