use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    // let input = include_str!("../test.txt");

    println!("part1 = {}", part1(input));
    println!("part2 = {}", part2(input));
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

fn part2(input: &str) -> i32 {
    input.lines().map(decode).sum()
}

fn decode(line: &str) -> i32 {
    // == Parse ==
    let (examples, output) = line.split_once(" | ").unwrap();
    let examples: Vec<&str> = examples.split(' ').collect();

    let one = examples.iter().find(|example| example.len() == 2).unwrap();
    let four = examples.iter().find(|example| example.len() == 4).unwrap();
    let seven = examples.iter().find(|example| example.len() == 3).unwrap();
    let eight = examples.iter().find(|example| example.len() == 7).unwrap();

    let zero_six_nine: Vec<&str> = examples
        .iter()
        .filter(|example| example.len() == 6)
        .copied()
        .collect();
    let two_three_five: Vec<&str> = examples
        .iter()
        .filter(|example| example.len() == 5)
        .copied()
        .collect();

    // == Deduce ==
    let (_a, cf) = deduce_a_and_cf(one, seven);

    let bd: Vec<char> = four.chars().filter(|&c| !one.contains(c)).collect();

    let (zero, six, nine) = deduce_zero_six_nine(&bd, &cf, &zero_six_nine);
    let (two, three, five) = deduce_two_three_five(&bd, &cf, &two_three_five);

    let map = HashMap::from([
        (sort_str(zero), 0),
        (sort_str(one), 1),
        (sort_str(two), 2),
        (sort_str(three), 3),
        (sort_str(four), 4),
        (sort_str(five), 5),
        (sort_str(six), 6),
        (sort_str(seven), 7),
        (sort_str(eight), 8),
        (sort_str(nine), 9),
    ]);

    // == Decode output ==
    let mut value = 0;
    for (digit, number) in output.split(' ').rev().enumerate() {
        let number = sort_str(number);
        let order_of_magnitude = 10i32.pow(digit as u32);

        value += map.get(&number).unwrap() * order_of_magnitude;
    }

    value
}

// Whichever segment in 7 is not in 1 must be `a`
// The other segments are either `c` or `f`
fn deduce_a_and_cf(one: &str, seven: &str) -> (char, Vec<char>) {
    let (a, cf): (Vec<_>, Vec<_>) = seven
        .chars()
        .partition(|&segment| one.find(segment).is_none());

    (*a.first().unwrap(), cf)
}

fn deduce_zero_six_nine<'a>(
    bd: &[char],
    cf: &[char],
    zero_six_nine: &[&'a str],
) -> (&'a str, &'a str, &'a str) {
    let mut zero = "";
    let mut six = "";
    let mut nine = "";

    for number in zero_six_nine {
        // If a number doesn't contain both `b` and `d`, it must be 0
        if !bd.iter().all(|&segment| number.contains(segment)) {
            zero = number
        // If a number contain both `c` and `f`, it must be 9
        } else if cf.iter().all(|&segment| number.contains(segment)) {
            nine = number
        } else {
            six = number
        }
    }

    (zero, six, nine)
}

fn deduce_two_three_five<'a>(
    bd: &[char],
    cf: &[char],
    two_three_five: &[&'a str],
) -> (&'a str, &'a str, &'a str) {
    let mut two = "";
    let mut three = "";
    let mut five = "";

    for number in two_three_five {
        // If a number contains both `b` and `d`, it must be 5
        if bd.iter().all(|&segment| number.contains(segment)) {
            five = number
        // If a number contains both `c` and `f`, it must be 3
        } else if cf.iter().all(|&segment| number.contains(segment)) {
            three = number
        } else {
            two = number
        }
    }

    (two, three, five)
}

// We sort the segment strings so they have a consistent order
fn sort_str(string: &str) -> String {
    let mut chars: Vec<char> = string.chars().collect();
    chars.sort_unstable();

    String::from_iter(chars)
}
