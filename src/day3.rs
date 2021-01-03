use std::collections::HashSet;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> usize {
    let mut set = HashSet::new();
    let mut loc = (0, 0);
    set.insert(loc);
    input.bytes().for_each(|b| {
        match b {
            b'^' => loc.1 += 1,
            b'v' => loc.1 -= 1,
            b'<' => loc.0 -= 1,
            b'>' => loc.0 += 1,
            _ => (),
        }
        set.insert(loc);
    });
    set.len()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> usize {
    let mut set = HashSet::new();
    let mut loc = [(0, 0); 2];
    set.insert(loc[0]);
    input.bytes().enumerate().for_each(|(i, b)| {
        match b {
            b'^' => loc[i % 2].1 += 1,
            b'v' => loc[i % 2].1 -= 1,
            b'<' => loc[i % 2].0 -= 1,
            b'>' => loc[i % 2].0 += 1,
            _ => (),
        }
        set.insert(loc[i % 2]);
    });
    set.len()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(">"), 2);
        assert_eq!(part1("^>v<"), 4);
        assert_eq!(part1("^v^v^v^v^v"), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("^v"), 3);
        assert_eq!(part2("^>v<"), 3);
        assert_eq!(part2("^v^v^v^v^v"), 11);
    }
}
