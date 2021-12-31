use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let (template, rule) = input.split_once("\n\n").unwrap();

    let template: Vec<char> = template.chars().collect();

    let rules: HashMap<(char, char), char> = rule
        .lines()
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|(pair, _result)| {
            let chars: Vec<char> = pair.chars().collect();
            ((chars[0], chars[1]), _result)
        })
        .map(|(_pair, result)| (_pair, result.chars().next().unwrap()))
        .collect();

    part1(&template, &rules);
    part2(&template, &rules);
}

fn part1(template: &[char], rules: &HashMap<(char, char), char>) {
    let mut polymer = template.to_vec();
    for _ in 0..10 {
        polymer = apply_rules(polymer, rules);
    }

    println!("part 1 = {}", score(&polymer));
}

fn part2(template: &[char], rules: &HashMap<(char, char), char>) {
    let mut pairs: HashMap<(char, char), u64> = HashMap::default();
    for pair in template.windows(2) {
        let pair = (pair[0], pair[1]);
        pairs.entry(pair).and_modify(|e| *e += 1).or_insert(1);
    }

    let mut counts: HashMap<char, u64> = HashMap::default();
    for (a, _b) in pairs.keys() {
        counts.entry(*a).and_modify(|e| *e += 1).or_insert(1);
    }

    for _ in 0..41 {
        let mut new_pairs: HashMap<(char, char), u64> = HashMap::default();
        let mut new_counts: HashMap<char, u64> = HashMap::default();

        for (pair, &count) in &pairs {
            let middle = rules.get(pair).unwrap();

            new_pairs
                .entry((pair.0, *middle))
                .and_modify(|e| *e += count)
                .or_insert(count);

            new_pairs
                .entry((*middle, pair.1))
                .and_modify(|e| *e += count)
                .or_insert(count);

            new_counts
                .entry(pair.0)
                .and_modify(|e| *e += count)
                .or_insert(count);
        }

        pairs = new_pairs;
        counts = new_counts;
    }

    counts
        .entry(*template.last().unwrap())
        .and_modify(|e| *e += 1)
        .or_insert(1);

    let counts: Vec<u64> = counts.into_values().collect();

    println!(
        "part2 = {}",
        counts.iter().max().unwrap() - counts.iter().min().unwrap()
    );
}

fn apply_rules(polymer: Vec<char>, rules: &HashMap<(char, char), char>) -> Vec<char> {
    let mut new_polymer: Vec<char> = vec![*polymer.first().unwrap()];

    for elements in polymer.windows(2) {
        let elem1 = elements[0];
        let elem2 = elements[1];

        new_polymer.push(*rules.get(&(elem1, elem2)).unwrap());
        new_polymer.push(elem2);
    }

    new_polymer
}

fn score(polymer: &[char]) -> usize {
    let mut map: HashMap<char, usize> = Default::default();

    for element in polymer {
        let count = map.entry(*element).or_default();
        *count += 1;
    }

    let (_max_elem, max_count) = map.iter().max_by_key(|(_element, count)| *count).unwrap();
    let (_min_elem, min_count) = map.iter().min_by_key(|(_element, count)| *count).unwrap();

    max_count - min_count
}
