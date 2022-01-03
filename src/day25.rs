use regex::Regex;

#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> (u64, u64) {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"To continue, please consult the code grid in the manual.  Enter the code at row (\d+), column (\d+).")
                .expect("Error creating Regex");
    }
    let captures = RE.captures(input).expect("Could not get Regex captures");
    (
        captures.get(1).expect("No row found").as_str().parse().unwrap_or(0), 
        captures.get(2).expect("No column found").as_str().parse().unwrap_or(0)
    )
}

#[aoc(day25, part1)]
pub fn part1(input: &(u64, u64)) -> u64 {
    let (row, col) = input;
    let mut target = 1;
    for r in 1..*row {
        target += r;
    }
    for c in 2..=*col {
        target += c + *row - 1;
    }
    let mut n = 20151125;
    for _ in 1..target {
        n = (n * 252533) % 33554393;
    }
    n
}

#[aoc(day25, part2)]
pub fn part2(input: &(u64, u64)) -> u64 {
    0
}
