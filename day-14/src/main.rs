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

    let mut polymer = template;
    for _ in 0..10 {
        polymer = apply_rules(polymer, &rules);
    }

    println!("part 1 = {}", score(&polymer));
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
