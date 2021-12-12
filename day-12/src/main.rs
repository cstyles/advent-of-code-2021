use std::collections::{HashMap, HashSet};

type Map<'a> = HashMap<&'a str, Vec<&'a str>>;

fn main() {
    let input = include_str!("../input.txt");
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();

    for (start, end) in input.lines().map(|line| line.split_once('-').unwrap()) {
        let entry = map.entry(start).or_insert_with(Vec::new);
        entry.push(end);

        let entry = map.entry(end).or_insert_with(Vec::new);
        entry.push(start);
    }

    dbg!(&map);

    let mut paths = 0;
    for path in map.get("start").unwrap() {
        let visited = HashSet::from(["start"]);
        paths += try_path(&map, visited.clone(), path);
    }

    println!("part1 = {}", paths);
}

fn try_path<'a>(map: &Map, mut visited: HashSet<&'a str>, target: &'a str) -> usize {
    if target == "end" {
        return 1;
    }

    if is_lowercase(target) && visited.contains(target) {
        return 0;
    }

    visited.insert(target);

    let mut count = 0;
    for path in map.get(target).unwrap() {
        count += try_path(map, visited.clone(), path);
    }

    count
}

fn is_lowercase(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_lowercase())
}
