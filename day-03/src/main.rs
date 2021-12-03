fn main() {
    let input = include_str!("../input.txt");

    let mut number_of_zeroes = [0u32; 12];
    for line in input.lines() {
        for (i, bit) in line.chars().enumerate() {
            if bit == '0' {
                number_of_zeroes[i] += 1;
            }
        }
    }

    let mut gamma = 0;
    for (i, count) in number_of_zeroes.iter().enumerate() {
        if *count < 500 {
            gamma |= 1 << (12 - i - 1);
        }
    }

    let epsilon = gamma ^ 0b0000_1111_1111_1111;

    println!("ugh = {:?}", number_of_zeroes);
    println!("gamma   = {:12b}", gamma);
    println!("epsilon = {:12b}", epsilon);
    println!("part1 = {}", gamma * epsilon);
}
