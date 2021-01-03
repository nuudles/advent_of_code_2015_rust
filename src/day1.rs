#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    input
        .bytes()
        .fold(0, |floor, b| if b == b'(' { floor + 1 } else { floor - 1 })
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    let mut floor = 0;
    for (i, c) in input.chars().enumerate() {
        if c == '(' {
            floor += 1;
        } else {
            floor -= 1;
        }
        if floor < 0 {
            return i + 1;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1("(())"), 0);
        assert_eq!(part1("()()"), 0);
        assert_eq!(part1("((("), 3);
        assert_eq!(part1("(()(()("), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(")"), 1);
        assert_eq!(part2("()())"), 5);
    }
}
