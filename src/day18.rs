use std::collections::HashSet;
use std::cmp::min;
use itertools::iproduct;

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> HashSet<(usize, usize)> {
    input
        .lines()
        .enumerate()
        .fold(HashSet::new(), |mut set, (y, l)| {
            l.bytes().enumerate().for_each(|(x, b)|
                if b == b'#' {
                    set.insert((x, y));
                }
            );
            set
        })
}

fn neighbors(point: (usize, usize), max: (usize, usize)) -> HashSet<(usize, usize)> {
    let mut points = HashSet::new();
    for (x, y) in iproduct!(
        point.0.saturating_sub(1)..=min(max.0 - 1, point.0 + 1), 
        point.1.saturating_sub(1)..=min(max.1 - 1, point.1 + 1)
    ) {
        if x == point.0 && y == point.1 {
            continue;
        }
        points.insert((x, y));
    }
    points
}

fn run_step(lights: HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
    let mut checked: HashSet<(usize, usize)> = HashSet::new();
    lights
        .iter()
        .fold(HashSet::new(), |mut set, light| {
            let n = neighbors(*light, (100, 100));
            let lit_count = n
                .iter()
                .filter(|l| lights.contains(l))
                .collect::<Vec<_>>()
                .len();
            if lit_count == 2 || lit_count == 3 {
                set.insert(*light);
            }
            n.iter()
                .filter(|l|
                    !lights.contains(l) && 
                        checked.insert(**l) &&
                        neighbors(**l, (100, 100))
                            .iter()
                            .filter(|n| lights.contains(n))
                            .collect::<Vec<_>>()
                            .len() == 3
                )
                .for_each(|l| {
                    set.insert(*l);
                });
            set
        })
}

#[aoc(day18, part1)]
pub fn part1<'a>(input: &HashSet<(usize, usize)>) -> usize {
    let mut lights = input.clone();
    for _ in 0..100 {
        lights = run_step(lights);
    }
    lights.len()
}

#[aoc(day18, part2)]
pub fn part2(input: &HashSet<(usize, usize)>) -> usize {
    let mut lights = input.clone();
    for _ in 0..100 {
        lights.insert((0, 0));
        lights.insert((99, 0));
        lights.insert((99, 99));
        lights.insert((0, 99));
        lights = run_step(lights);
    }
    lights.insert((0, 0));
    lights.insert((99, 0));
    lights.insert((99, 99));
    lights.insert((0, 99));
    lights.len()
}
