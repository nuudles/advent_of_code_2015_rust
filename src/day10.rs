#[derive(Debug, Clone, Copy)]
pub struct Count {
    count: u32,
    digit: u32
}

impl Count {
    fn increment_count(&mut self) {
        self.count += 1
    }
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Count> {
    let mut last_count: Option<Count> = None;
    let mut counts = input
        .chars()
        .fold(Vec::new(), |mut result, c| {
            let digit = c.to_digit(10).expect("Digit not found");

            let mut count = last_count.unwrap_or(
                Count {
                    count: 0,
                    digit: digit
                }
            );
            if count.digit != digit {
                result.push(count);
                last_count = Some(
                    Count {
                        count: 1,
                        digit: digit
                    }
                );
            } else {
                count.increment_count();
                last_count = Some(count);
            }
            result
        });
    if let Some(last_count) = last_count {
        counts.push(last_count);
    }
    counts
}

fn expand(counts: Vec<Count>) -> Vec<Count> {
    let mut last_count: Option<Count> = None;
    let mut expansion = vec![];

    let mut insert_to_expansion = |d| {
        let mut new_count = last_count.unwrap_or(
            Count {
                count: 0,
                digit: d
            }
        );
        if new_count.digit != d {
            expansion.push(new_count);
            last_count = Some(
                Count {
                    count: 1,
                    digit: d
                }
            );
        } else {
            new_count.increment_count();
            last_count = Some(new_count);
        }
    };

    for c in counts {
        insert_to_expansion(c.count);
        insert_to_expansion(c.digit);
    }

    if let Some(last_count) = last_count {
        expansion.push(last_count);
    }
    expansion
}

#[aoc(day10, part1)]
pub fn part1(input: &Vec<Count>) -> u32 {
    let mut counts = input.to_vec();
    for _ in 0..40 {
        counts = expand(counts);
    }
    counts
        .iter()
        .map(|c| c.count)
        .sum()
}

#[aoc(day10, part2)]
pub fn part2(input: &Vec<Count>) -> u32 {
    let mut counts = input.to_vec();
    for _ in 0..50 {
        counts = expand(counts);
    }
    counts
        .iter()
        .map(|c| c.count)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, input_generator};

    #[test]
    fn test_part1() {
        println!("{:?}", part1(&input_generator("111221")));
    }
}
