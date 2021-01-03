use std::cmp::min;

#[derive(Debug, PartialEq, Eq)]
pub struct Gift {
    l: u32,
    w: u32,
    h: u32,
}

impl Gift {
    fn from(string: &str) -> Gift {
        let components: Vec<u32> = string.split("x").map(|x| x.parse().unwrap_or(0)).collect();
        Gift {
            l: components[0],
            w: components[1],
            h: components[2],
        }
    }

    fn surface_area(&self) -> u32 {
        2 * self.l * self.w + 2 * self.w * self.h + 2 * self.h * self.l
    }

    fn slack(&self) -> u32 {
        min(min(self.l * self.w, self.w * self.h), self.l * self.h)
    }

    fn part1(&self) -> u32 {
        self.surface_area() + self.slack()
    }

    fn part2(&self) -> u32 {
        let mut vec = vec![self.l, self.w, self.h];
        vec.sort();
        vec.pop();
        vec.iter().fold(0, |sum, x| sum + x * 2) + self.l * self.w * self.h
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Gift> {
    input.lines().map(|l| Gift::from(l)).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Gift]) -> u32 {
    input.iter().fold(0, |sum, gift| sum + gift.part1())
}

#[aoc(day2, part2)]
pub fn part2(input: &[Gift]) -> u32 {
    input.iter().fold(0, |sum, gift| sum + gift.part2())
}

#[cfg(test)]
mod tests {
    use super::{input_generator, Gift};

    #[test]
    fn test_generator() {
        assert_eq!(input_generator("2x3x4"), vec![Gift { l: 2, w: 3, h: 4 }]);
    }

    #[test]
    fn test_part1() {
        assert_eq!(Gift { l: 2, w: 3, h: 4 }.part1(), 58);
        assert_eq!(Gift { l: 1, w: 1, h: 10 }.part1(), 43);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Gift { l: 2, w: 3, h: 4 }.part2(), 34);
        assert_eq!(Gift { l: 1, w: 1, h: 10 }.part2(), 14);
    }
}
