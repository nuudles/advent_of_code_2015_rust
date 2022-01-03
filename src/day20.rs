fn sum_of_multiples(number: u64) -> u64 {
    (1..=number)
        .filter(|n| number % n == 0)
        .sum()
}

fn sum_of_multiples_stopping_at_50(number: u64) -> u64 {
    (1..=number)
        .filter(|n| number % n == 0)
        .filter(|n| n * 50 >= number)
        .sum()
}

/*
For both parts, I just fudged around with the numbers until I noticed a pattern, then tried did basically
a manual binary search to get in the vicinity to search for the number... It seemed to work?
*/

#[aoc(day20, part1)]
pub fn part1(input: &str) -> u64 {
    let target: u64 = input.parse().expect("No target found");
    let mut number = 0;
    while sum_of_multiples(number) * 10 < target {
        number += 210;
    }
    number
}

#[aoc(day20, part2)]
pub fn part2(input: &str) -> u64 {
    let target: u64 = input.parse().expect("No target found");
    let mut number = 850080;
    while sum_of_multiples_stopping_at_50(number) * 11 < target {
        number += 210;
    }
    number
}
