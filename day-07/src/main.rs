fn main() {
    let input = include_str!("../input.txt");
    // let input = include_str!("../test.txt");

    let mut positions: Vec<i32> = input
        .trim_end()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();

    positions.sort();
    let median = positions[positions.len() / 2];

    let fuel: i32 = positions
        .iter()
        .map(|position| (position - median).abs())
        .sum();

    println!("part1 = {}", fuel);
}
