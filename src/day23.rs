use std::collections::HashMap;
use std::convert::TryFrom;

fn run_program<'a>(instructions: Vec<Vec<&'a str>>, registers: &mut HashMap<&'a str, u64>) {
    let mut index = 0i64;
    while let Ok(i) = usize::try_from(index) {
        if i >= instructions.len() {
            break;
        }
        let instruction = &instructions[i];
        match instruction[0] {
            "hlf" => {
                registers.insert(instruction[1], registers.get(instruction[1]).unwrap_or(&0) / 2);
                index += 1;
            },
            "tpl" => {
                registers.insert(instruction[1], registers.get(instruction[1]).unwrap_or(&0) * 3);
                index += 1;
            },
            "inc" => {
                registers.insert(instruction[1], registers.get(instruction[1]).unwrap_or(&0) + 1);
                index += 1;
            },
            "jmp" => {
                index += instruction[1].trim_start_matches('+').parse::<i64>().unwrap_or(0);
            }
            "jie" => {
                if registers.get(instruction[1].trim_end_matches(',')).unwrap_or(&0) % 2 == 0 {
                    index += instruction[2].trim_start_matches('+').parse::<i64>().unwrap_or(0);
                } else {
                    index += 1;
                }
            }
            "jio" => {
                if registers.get(instruction[1].trim_end_matches(',')).unwrap_or(&0) == &1 {
                    index += instruction[2].trim_start_matches('+').parse::<i64>().unwrap_or(0);
                } else {
                    index += 1;
                }
            }
            _ => ()
        }
    }
}

#[aoc(day23, part1)]
pub fn part1(input: &str) -> u64 {
    let instructions = input
        .lines()
        .map(|l| l.split(" ").collect::<Vec<&str>>())
        .collect::<Vec<_>>();
    let mut registers: HashMap<&str, u64> = HashMap::new();
    run_program(instructions, &mut registers);
    *registers.get("b").unwrap_or(&0)
}

#[aoc(day23, part2)]
pub fn part2(input: &str) -> u64 {
    let instructions = input
        .lines()
        .map(|l| l.split(" ").collect::<Vec<&str>>())
        .collect::<Vec<_>>();
    let mut registers: HashMap<&str, u64> = HashMap::new();
    registers.insert("a", 1);
    run_program(instructions, &mut registers);
    *registers.get("b").unwrap_or(&0)
}
