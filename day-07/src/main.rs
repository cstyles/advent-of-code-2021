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

    let fuel = total_fuel(median, &positions);
    println!("part1 = {}", fuel);

    let min = positions.first().unwrap();
    let max = positions.last().unwrap();

    let min_fuel = (*min..=*max)
        .map(|target| total_fuel_part2(target, &positions))
        .min()
        .unwrap();
    println!("part2 = {}", min_fuel);
}

fn total_fuel(target: i32, positions: &[i32]) -> i32 {
    positions
        .iter()
        .map(|position| (target - position).abs())
        .sum()
}

fn total_fuel_part2(target: i32, positions: &[i32]) -> i32 {
    positions
        .iter()
        .map(|position| part2_fuel(*position, target))
        .sum()
}

fn part2_fuel(start: i32, end: i32) -> i32 {
    let end = (end - start).abs();
    (end.pow(2) + end) / 2
}
