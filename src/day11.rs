use std::collections::HashSet;
use std::str;
use std::iter::FromIterator;

#[aoc(day11, part1)]
pub fn part1(input: &str) -> String {
    let mut chars: Vec<_> = input.bytes().collect();
    loop {
        let mut index = chars.len() - 1;
        loop {
            if chars[index] == b'z' {
                chars[index] = b'a';
                index -= 1;
            } else {
                chars[index] += 1;
                break;
            }
        }

        if !chars.windows(3).any(|w| w[1] == w[0] + 1 && w[2] == w[0] + 2) {
            continue;
        }
        if chars.iter().any(|c| *c == b'i' || *c == b'o' || *c == b'l') {
            continue;
        }
        let pairs: HashSet<_> = HashSet::from_iter(
            chars
                .windows(2)
                .filter(|w| w[0] == w[1])
        );
        if pairs.len() < 2 {
            continue;
        }
        break;
    }
    String::from_utf8(chars).expect("Should have some real characters!")
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> String {
    part1(part1(input).as_str())
}

#[cfg(test)]
mod tests {
    use super::{part1};

    #[test]
    fn test_part1() {
        assert_eq!(part1("abcdefgh"), "abcdffaa");
        assert_eq!(part1("ghijklmn"), "ghjaabcc");
    }
}

