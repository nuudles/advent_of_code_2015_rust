use regex::Regex;

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<(u64, u64, u64)> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(.*) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.")
                .expect("Error creating Regex");
    }
    input
        .lines()
        .filter_map(|l| RE.captures(l))
        .fold(vec![], |mut result, captures| {
            let speed = captures.get(2).expect("No speed found").as_str().parse::<u64>().unwrap_or(0);
            let flight_time = captures.get(3).expect("No flight time found").as_str().parse::<u64>().unwrap_or(0);
            let rest_time = captures.get(4).expect("No rest time found").as_str().parse::<u64>().unwrap_or(0);
            result.push((speed, flight_time, rest_time));
            result
        })
}

#[aoc(day14, part1)]
pub fn part1(input: &Vec<(u64, u64, u64)>) -> u64 {
    let check_at = 2503;
    let distances = input
        .iter()
        .map(|r| {
            let mut time_remaining = check_at;
            let mut distance = 0;
            loop {
                if time_remaining < r.1 {
                    distance += time_remaining * r.0;
                    break;
                }
                distance += r.0 * r.1;
                time_remaining -= r.1;
                if time_remaining < r.2 {
                    break;
                }
                time_remaining -= r.2;
            }
            distance
        });
    distances.max().unwrap_or(0)
}

#[aoc(day14, part2)]
pub fn part2(input: &Vec<(u64, u64, u64)>) -> u64 {
    let mut points: Vec<u64> = input.iter().map(|_| 0).collect();
    let mut distances: Vec<u64> = input.iter().map(|_| 0).collect();
    let mut timers: Vec<u64> = input.iter().map(|_| 0).collect();
    for _ in 0..=2503 {
        for (i, r) in input.iter().enumerate() {
            timers[i] += 1;
            if timers[i] <= r.1 {
                distances[i] += r.0;
            } else if timers[i] == r.1 + r.2 {
                timers[i] = 0;
            }
        }

        let winner = distances.iter().max().unwrap_or(&0);
        distances
            .iter()
            .enumerate()
            .filter(|d| d.1 == winner)
            .for_each(|d| points[d.0] += 1);
    }
    *points.iter().max().expect("No max found!")
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, input_generator};

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&input_generator(r"
Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."
            )),
            2660
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&input_generator(r"
Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."
            )),
            1564
        );
    }
}
