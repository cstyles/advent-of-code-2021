#[derive(Debug)]
enum Bit {
    Zero,
    One,
}

fn main() {
    let input = include_str!("../input.txt");
    // let input = include_str!("../test.txt");

    let width = input.lines().next().unwrap().len();

    let lines: Vec<&str> = input.lines().collect();
    let number_of_zeroes = most_common_bit(&lines);

    let mut gamma = 0;
    for (i, bit) in number_of_zeroes.iter().enumerate() {
        if let Bit::One = bit {
            gamma |= 1 << (width - i - 1);
        }
    }

    // epsilon is just gamma but with every bit flipped
    let epsilon = gamma ^ 0b0000_1111_1111_1111;

    println!("part1 = {}", gamma * epsilon);

    let mut o2_lines = lines.clone();
    for position in 0..width {
        let number_of_zeroes = most_common_bit(&o2_lines);
        match number_of_zeroes[position] {
            Bit::Zero => o2_lines.retain(|num| num.chars().nth(position).unwrap() == '0'),
            Bit::One => o2_lines.retain(|num| num.chars().nth(position).unwrap() == '1'),
        }

        if o2_lines.len() == 1 {
            break;
        }
    }

    let oxygen = o2_lines.first().unwrap();
    let oxygen = u64::from_str_radix(oxygen, 2).unwrap();

    let mut lines = lines;
    for position in 0..width {
        let number_of_zeroes = most_common_bit(&lines);
        match number_of_zeroes[position] {
            Bit::Zero => lines.retain(|num| num.chars().nth(position).unwrap() == '1'),
            Bit::One => lines.retain(|num| num.chars().nth(position).unwrap() == '0'),
        }

        if lines.len() == 1 {
            break;
        }
    }

    let carbon_dioxide = lines.first().unwrap();
    let carbon_dioxide = u64::from_str_radix(carbon_dioxide, 2).unwrap();
    println!("part2 = {}", oxygen * carbon_dioxide);
}

fn most_common_bit(numbers: &[&str]) -> Vec<Bit> {
    let width = numbers.first().unwrap().len();
    let mut number_of_zeroes = vec![0; width];

    for number in numbers {
        for (i, bit) in number.chars().enumerate() {
            if bit == '0' {
                number_of_zeroes[i] += 1;
            }
        }
    }

    number_of_zeroes
        .into_iter()
        .map(|count| {
            if count <= numbers.len() / 2 {
                Bit::One
            } else {
                Bit::Zero
            }
        })
        .collect()
}
