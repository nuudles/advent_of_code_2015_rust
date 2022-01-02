use std::collections::HashMap;
use std::collections::HashSet;
use pathfinding::prelude::astar;

#[aoc(day19, part1)]
pub fn part1(input: &str) -> usize {
    let mut components = input.split("\n\n");
    let replacements: HashMap<&str, HashSet<&str>> = components
        .next()
        .expect("No replacements found")
        .lines()
        .fold(HashMap::new(), |mut map, l| {
            let mut c = l.split(" => ");
            let find = c.next().unwrap_or("");
            let replace = c.next().unwrap_or("");
            if let Some(set) = map.get_mut(find) {
                set.insert(replace);
            } else {
                let mut set = HashSet::new();
                set.insert(replace);
                map.insert(find, set);
            }
            map
        });
    let string = components.next().expect("No string found");
    let molecules = replacements
        .iter()
        .fold(HashSet::<String>::new(), |mut set, (find, replace)| {
            string
                .match_indices(find)
                .for_each(|(i, _)| {
                    replace.iter().for_each(|r| {
                        let mut new_string = String::from(string);
                        new_string.replace_range(i..i + find.len(), r);
                        set.insert(new_string);
                    });
                });
            set
        });
    molecules.len()
}

#[aoc(day19, part2)]
pub fn part2(input: &str) -> usize {
    let mut components = input.split("\n\n");
    let replacements: HashMap<&str, &str> = components
        .next()
        .expect("No replacements found")
        .lines()
        .fold(HashMap::new(), |mut map, l| {
            let mut c = l.split(" => ");
            let find = c.next().unwrap_or("");
            let replace = c.next().unwrap_or("");
            map.insert(replace, find);
            map
        });
    let string = components.next().expect("No string found");
    let path = astar(
        &String::from(string),
        |from| {
            replacements
                .iter()
                .fold(Vec::new(), |mut result, (find, replace)| {
                    from
                        .match_indices(find)
                        .for_each(|(i, _)| {
                            let mut new_string = String::from(from);
                            new_string.replace_range(i..i + find.len(), replace);
                            result.push((new_string, 1));
                        });
                    result
                })
        },
        |s| s.len(),
        |s| s == "e"
    );

    path.expect("No path found").1
}
