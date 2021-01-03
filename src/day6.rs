use regex::Regex;
use std::cmp::max;

#[derive(PartialEq, Eq, Debug)]
pub enum Operation {
    TurnOn,
    TurnOff,
    Toggle,
}

impl Operation {
    fn from(string: &str) -> Operation {
        match string {
            "turn on" => Operation::TurnOn,
            "turn off" => Operation::TurnOff,
            _ => Operation::Toggle,
        }
    }
}

#[derive(Debug)]
pub struct Instruction {
    operation: Operation,
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
}

impl Instruction {
    fn from(string: &str) -> Option<Instruction> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"(turn on|toggle|turn off) (\d+),(\d+) through (\d+),(\d+)").unwrap();
        }
        let captures = RE.captures(string)?;
        let operation = captures.get(1).map(|m| m.as_str()).map(Operation::from)?;
        let get_usize = |index| {
            captures
                .get(index)
                .map(|m| m.as_str().parse::<usize>().unwrap_or(0))
        };
        let min_x = get_usize(2)?;
        let min_y = get_usize(3)?;
        let max_x = get_usize(4)?;
        let max_y = get_usize(5)?;
        Some(Instruction {
            operation,
            min_x,
            max_x,
            min_y,
            max_y,
        })
    }
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    input.lines().filter_map(Instruction::from).collect()
}

#[aoc(day6, part1)]
pub fn part1(input: &[Instruction]) -> usize {
    let mut grid = [[false; 1000]; 1000];
    for instruction in input {
        for y in instruction.min_y..=instruction.max_y {
            for x in instruction.min_x..=instruction.max_x {
                match &instruction.operation {
                    Operation::TurnOn => grid[y][x] = true,
                    Operation::TurnOff => grid[y][x] = false,
                    Operation::Toggle => grid[y][x] = !grid[y][x],
                }
            }
        }
    }
    grid.iter()
        .fold(0, |sum, r| sum + r.iter().filter(|x| **x).count())
}

#[aoc(day6, part2)]
pub fn part2(input: &[Instruction]) -> i32 {
    let mut grid = [[0; 1000]; 1000];
    for instruction in input {
        for y in instruction.min_y..=instruction.max_y {
            for x in instruction.min_x..=instruction.max_x {
                match &instruction.operation {
                    Operation::TurnOn => grid[y][x] += 1,
                    Operation::TurnOff => grid[y][x] = max(grid[y][x] - 1, 0),
                    Operation::Toggle => grid[y][x] += 2,
                }
            }
        }
    }
    grid.iter().fold(0, |sum, r| sum + r.iter().sum::<i32>())
}

#[cfg(test)]
mod tests {
    use super::{Instruction, Operation};

    #[test]
    fn test_instruction_generation() {
        let instruction = Instruction::from("turn on 0,0 through 999,999").unwrap();
        assert_eq!(instruction.operation, Operation::TurnOn);
        assert_eq!(instruction.min_x, 0);
        assert_eq!(instruction.max_x, 999);
        assert_eq!(instruction.min_y, 0);
        assert_eq!(instruction.max_y, 999);
    }
}
