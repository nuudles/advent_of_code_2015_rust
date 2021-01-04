use regex::Regex;

fn char_count(input: &str) -> usize {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"(\\")|(\\\\)|(\\x[0-9a-f]{2})"#).unwrap();
    }
    RE.replace_all(input, "_").len() - 2
}

fn encoded_char_count(input: &str) -> usize {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"(\\)|(")"#).unwrap();
    }
    RE.replace_all(input, "__").len() + 2
}

#[aoc(day8, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| l.len() > 0)
        .fold(0, |sum, l| sum + l.len() - char_count(l))
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| l.len() > 0)
        .fold(0, |sum, l| sum + encoded_char_count(l) - l.len())
}

#[cfg(test)]
mod tests {
    use super::{char_count, encoded_char_count, part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(char_count(r#""""#), 0);
        assert_eq!(char_count(r#""abc""#), 3);
        assert_eq!(char_count(r#""aaa\"aaa""#), 7);
        assert_eq!(char_count(r#""\x27""#), 1);

        let string = r#"
			""
			"abc"
			"aaa\"aaa"
			"\x27"
		"#;
        assert_eq!(part1(string), 12);
    }

    #[test]
    fn test_part2() {
        assert_eq!(encoded_char_count(r#""""#), 6);
        assert_eq!(encoded_char_count(r#""abc""#), 9);
        assert_eq!(encoded_char_count(r#""aaa\"aaa""#), 16);
        assert_eq!(encoded_char_count(r#""\x27""#), 11);
        let string = r#"
			""
			"abc"
			"aaa\"aaa"
			"\x27"
		"#;
        assert_eq!(part2(string), 19);
    }
}
