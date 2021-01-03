use md5;

#[aoc(day4, part1)]
pub fn part1(input: &str) -> u32 {
    let mut x = 0;
    while !format!("{:x}", md5::compute(format!("{}{}", input, x))).starts_with("00000") {
        x += 1;
    }
    x
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> u32 {
    let mut x = 0;
    while !format!("{:x}", md5::compute(format!("{}{}", input, x))).starts_with("000000") {
        x += 1;
    }
    x
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn test_part1() {
        assert_eq!(part1("abcdef"), 609043);
        assert_eq!(part1("pqrstuv"), 1048970);
    }
}
