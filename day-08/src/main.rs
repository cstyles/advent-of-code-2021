fn main() {
    let input = include_str!("../input.txt");
    // let input = include_str!("../test.txt");

    println!("part1 = {}", part1(input));
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| line.split_once(" | "))
        .map(|(_, output)| output)
        .flat_map(|output| output.split(' '))
        .map(str::len)
        .filter(|&len| len == 2 || len == 3 || len == 4 || len == 7)
        .count()
}
