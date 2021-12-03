#[derive(Debug)]
enum Bit {
    Zero,
    One
}

fn main() {
    let input = include_str!("../input.txt");
    // let input = include_str!("../test.txt");

    let width = input.lines().next().unwrap().len();

    let lines: Vec<&str> = input.lines().collect();
    let number_of_zeroes = most_common_bit(&lines, width);

    let mut gamma = 0;
    for (i, bit) in number_of_zeroes.iter().enumerate() {
        if let Bit::One = bit {
            gamma |= 1 << (width - i - 1);
        }
    }

    // epsilon is just gamma but with every bit flipped
    let epsilon = gamma ^ 0b0000_1111_1111_1111;

    println!("ugh = {:?}", number_of_zeroes);
    println!("gamma   = {:12b}", gamma);
    println!("epsilon = {:12b}", epsilon);
    println!("part1 = {}", gamma * epsilon);

    let mut o2_lines = lines.clone();
    dbg!(&o2_lines);
    for position in 0..width {
        let number_of_zeroes = most_common_bit(&o2_lines, width);

        dbg!(&number_of_zeroes);
        match number_of_zeroes[position] {
            Bit::Zero => println!("keeping 0s"),
            Bit::One => println!("keeping 1s"),
        }

        match number_of_zeroes[position] {
            Bit::Zero => o2_lines.retain(|num| num.chars().nth(position).unwrap() == '0'),
            Bit::One => o2_lines.retain(|num| num.chars().nth(position).unwrap() == '1'),
        }

        if o2_lines.len() == 1 {
            break;
        } else {
            dbg!(&o2_lines);
        }
    }

    let oxygen = o2_lines.first().unwrap();
    dbg!(oxygen);
    let oxygen = u64::from_str_radix(oxygen, 2).unwrap();

    let mut lines = lines.clone();
    for position in 0..width {
        let number_of_zeroes = most_common_bit(&lines, width);
        match number_of_zeroes[position] {
            Bit::Zero => lines.retain(|num| num.chars().nth(position).unwrap() == '1'),
            Bit::One => lines.retain(|num| num.chars().nth(position).unwrap() == '0'),
        }

        if lines.len() == 1 {
            break;
        }
    }

    let carbon_dioxide = lines.first().unwrap();
    dbg!(carbon_dioxide);
    let carbon_dioxide = u64::from_str_radix(carbon_dioxide, 2).unwrap();

    // 8190 = too low
    dbg!(oxygen);
    dbg!(carbon_dioxide);
}

fn most_common_bit(numbers: &[&str], width: usize) -> Vec<Bit> {
    let mut number_of_zeroes = vec![0; width];

    for number in numbers {
        for (i, bit) in number.chars().enumerate() {
            if bit == '0' {
                number_of_zeroes[i] += 1;
            }
        }
    }

    number_of_zeroes.into_iter().map(|count| {
        if count <= numbers.len() / 2 {
            Bit::One
        } else {
            Bit::Zero
        }
    }).collect()
}
