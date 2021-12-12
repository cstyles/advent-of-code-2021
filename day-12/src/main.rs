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

    let mut paths = 0;
    for path in map.get("start").unwrap() {
        let visited = HashSet::from(["start"]);
        paths += try_path_part1(&map, visited.clone(), path);
    }

    println!("part1 = {}", paths);

    let mut paths = 0;
    for path in map.get("start").unwrap() {
        let visited = HashSet::from(["start"]);
        paths += try_path_part2(&map, visited, path, false);
    }

    println!("part2 = {}", paths);
}

fn try_path_part1<'a>(map: &Map, mut visited: HashSet<&'a str>, target: &'a str) -> usize {
    if target == "end" {
        return 1;
    }

    if is_lowercase(target) && visited.contains(target) {
        return 0;
    }

    visited.insert(target);

    let mut count = 0;
    for path in map.get(target).unwrap() {
        count += try_path_part1(map, visited.clone(), path);
    }

    count
}

fn try_path_part2<'a>(
    map: &Map,
    mut visited: HashSet<&'a str>,
    target: &'a str,
    mut visited_small_twice: bool,
) -> usize {
    if target == "end" {
        return 1;
    }

    if target == "start" {
        return 0;
    }

    if is_lowercase(target) && visited.contains(target) {
        if visited_small_twice {
            return 0;
        } else {
            visited_small_twice = true;
        }
    }

    visited.insert(target);

    let mut count = 0;
    for path in map.get(target).unwrap() {
        count += try_path_part2(map, visited.clone(), path, visited_small_twice);
    }

    count
}

fn is_lowercase(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_lowercase())
}
