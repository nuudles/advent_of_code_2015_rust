use cached::proc_macro::cached;
use cached::SizedCache;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Instruction<'a> {
    Assign { wire: &'a str },
    And { left: &'a str, right: &'a str },
    Or { left: &'a str, right: &'a str },
    LShift { wire: &'a str, amount: usize },
    RShift { wire: &'a str, amount: usize },
    Not { wire: &'a str },
}

impl Instruction<'_> {
    fn from(string: &str) -> Instruction {
        if string.contains("AND") {
            let mut components = string.split(" AND ");
            return Instruction::And {
                left: components.next().expect("No left operand"),
                right: components.next().expect("No right operand"),
            };
        } else if string.contains("OR") {
            let mut components = string.split(" OR ");
            return Instruction::Or {
                left: components.next().expect("No left operand"),
                right: components.next().expect("No right operand"),
            };
        } else if string.contains("LSHIFT") {
            let mut components = string.split(" LSHIFT ");
            return Instruction::LShift {
                wire: components.next().expect("No left operand"),
                amount: components
                    .next()
                    .expect("No right operand")
                    .parse()
                    .expect("Invalid amount"),
            };
        } else if string.contains("RSHIFT") {
            let mut components = string.split(" RSHIFT ");
            return Instruction::RShift {
                wire: components.next().expect("No left operand"),
                amount: components
                    .next()
                    .expect("No right operand")
                    .parse()
                    .expect("Invalid amount"),
            };
        } else if string.contains("NOT") {
            return Instruction::Not { wire: &string[4..] };
        }

        Instruction::Assign { wire: string }
    }
}

#[cached(
    type = "SizedCache<String, u16>",
    create = "{ SizedCache::with_size(100) }",
    convert = r#"{ format!("{}", wire) }"#
)]
fn evaluate(wire: &str, instructions: &HashMap<&str, Instruction>) -> u16 {
    let value_or_evaluate = |w: &str| match w.parse::<u16>() {
        Ok(value) => value,
        Err(_) => evaluate(w, instructions),
    };

    match instructions.get(wire).expect("Wire not found") {
        Instruction::Assign { wire } => value_or_evaluate(wire),
        Instruction::And { left, right } => value_or_evaluate(left) & value_or_evaluate(right),
        Instruction::Or { left, right } => value_or_evaluate(left) | value_or_evaluate(right),
        Instruction::LShift { wire, amount } => value_or_evaluate(wire) << amount,
        Instruction::RShift { wire, amount } => value_or_evaluate(wire) >> amount,
        Instruction::Not { wire } => !value_or_evaluate(wire),
    }
}

fn get_instructions(input: &str) -> HashMap<&str, Instruction> {
    input.lines().fold(HashMap::new(), |mut m, l| {
        let mut components = l.split(" -> ");
        let instruction = Instruction::from(components.next().expect("No instruction found"));
        m.insert(
            components.next().expect("No destination found"),
            instruction,
        );
        m
    })
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> u16 {
    let instructions = get_instructions(input);
    evaluate("a", &instructions)
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> u16 {
    let mut instructions = get_instructions(input);
    instructions.insert("b", Instruction::Assign { wire: "956" });
    evaluate("a", &instructions)
}

#[cfg(test)]
mod tests {
    use super::Instruction;

    #[test]
    fn test_instruction() {
        assert_eq!(
            Instruction::from("123"),
            Instruction::Assign { wire: "123" }
        );
        assert_eq!(
            Instruction::from("x AND y"),
            Instruction::And {
                left: "x",
                right: "y"
            }
        );
        assert_eq!(
            Instruction::from("x OR y"),
            Instruction::Or {
                left: "x",
                right: "y"
            }
        );
        assert_eq!(
            Instruction::from("x LSHIFT 2"),
            Instruction::LShift {
                wire: "x",
                amount: 2
            }
        );
        assert_eq!(
            Instruction::from("y RSHIFT 2"),
            Instruction::RShift {
                wire: "y",
                amount: 2
            }
        );
        assert_eq!(Instruction::from("NOT x"), Instruction::Not { wire: "x" });
    }
}
