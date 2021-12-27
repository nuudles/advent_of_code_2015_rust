use std::collections::HashMap;
use std::collections::HashSet;
use itertools::Itertools;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<u32> {
    let mut map: HashMap<Vec<&str>, u32> = HashMap::new();
    let mut cities: HashSet<&str> = HashSet::new();
    input.lines().for_each(|line| {
        let mut components = line.split(" ");
        let city1 = components.next().expect("City not found");
        let city2 = components.nth(1).expect("City not found");
        let weight: u32 = components.nth(1).expect("Weight not found").parse().expect("Weight not valid");

        map.insert(vec![city1, city2], weight);
        map.insert(vec![city2, city1], weight);
        cities.insert(city1);
        cities.insert(city2);
    });

    let count = cities.len();
    cities
        .into_iter()
        .permutations(count)
        .filter_map(|permutation| {
            permutation
                .windows(2)
                .fold(Some(0), |result, w| {
                    if let Some(result) = result {
                        if let Some(distance) = map.get(w) {
                            return Some(result + distance)
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                })
        })
        .collect()
}

#[aoc(day9, part1)]
pub fn part1(distances: &Vec<u32>) -> u32 {
    *distances.iter().min().expect("No minimum found!")
}

#[aoc(day9, part2)]
pub fn part2(distances: &Vec<u32>) -> u32 {
    *distances.iter().max().expect("No maximum found!")
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, input_generator};

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&input_generator("London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141")),
            605
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&input_generator("London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141")),
            982
        );
    }
}
