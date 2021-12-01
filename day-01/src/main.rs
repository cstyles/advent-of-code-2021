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

    let part2: Vec<i32> = input
        .windows(3)
        .map(IntoIterator::into_iter)
        .map(Iterator::sum)
        .collect();

    let part2 = part2.windows(2).filter(|nums| nums[0] < nums[1]).count();

    println!("part2 = {}", part2);
}
