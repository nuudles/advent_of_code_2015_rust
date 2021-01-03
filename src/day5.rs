use std::collections::HashMap;

fn is_nice(string: &str) -> bool {
    let vowels = ['a', 'e', 'i', 'e', 'o', 'u'];
    let vowel_count = string.chars().filter(|c| vowels.contains(c)).count();
    let has_duplicate = string
        .chars()
        .collect::<Vec<_>>()
        .windows(2)
        .any(|w| w[0] == w[1]);

    let naughty_words = [('a', 'b'), ('c', 'd'), ('p', 'q'), ('x', 'y')];
    let contains_naughty_word = string
        .chars()
        .collect::<Vec<_>>()
        .windows(2)
        .map(|w| (w[0], w[1]))
        .any(|w| naughty_words.contains(&w));

    vowel_count >= 3 && has_duplicate && !contains_naughty_word
}

fn is_nice_2(string: &str) -> bool {
    let has_duplicate = string
        .chars()
        .collect::<Vec<_>>()
        .windows(2)
        .map(|w| (w[0], w[1]))
        .enumerate()
        .fold(&mut HashMap::new(), |m, (i, w)| {
            let v = m.entry(w).or_insert(vec![]);
            v.push(i);
            m
        })
        .iter()
        .map(|(_, v)| v)
        .filter(|v| v.len() > 1)
        .any(|v| v.windows(2).any(|w| w[1] - w[0] > 1));
    let has_sandwich = string
        .chars()
        .collect::<Vec<_>>()
        .windows(3)
        .any(|w| w[0] == w[2]);
    has_duplicate && has_sandwich
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> usize {
    input.lines().filter(|s| is_nice(s)).count()
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> usize {
    input.lines().filter(|s| is_nice_2(s)).count()
}

#[cfg(test)]
mod tests {
    use super::{is_nice, is_nice_2};

    #[test]
    fn test_part1() {
        assert_eq!(is_nice("ugknbfddgicrmopn"), true);
        assert_eq!(is_nice("aaa"), true);
        assert_eq!(is_nice("jchzalrnumimnmhp"), false);
        assert_eq!(is_nice("haegwjzuvuyypxyu"), false);
        assert_eq!(is_nice("dvszwmarrgswjxmb"), false);
    }

    #[test]
    fn test_part2() {
        assert_eq!(is_nice_2("qjhvhtzxzqqjkmpb"), true);
        assert_eq!(is_nice_2("xxyxx"), true);
        assert_eq!(is_nice_2("uurcxstgmygtbstg"), false);
        assert_eq!(is_nice_2("ieodomkazucvgmuy"), false);
    }
}
