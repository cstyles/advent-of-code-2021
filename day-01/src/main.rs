fn main() {
    let input: Vec<i32> = include_str!("../input.txt")
        .lines()
        .filter_map(|s| s.parse().ok())
        .collect();

    let part1 = input
        .windows(2)
        .filter(|numbers| numbers[0] < numbers[1])
        .count();

    println!("part1 = {}", part1);

    let part2 = input
        .windows(3)
        .map(|numbers| numbers.iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|numbers| numbers[0] < numbers[1])
        .count();

    println!("part2 = {}", part2);
}
