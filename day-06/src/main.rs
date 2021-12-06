fn main() {
    // let input = include_str!("../test-input.txt");
    let input = include_str!("../input.txt");

    let mut fishes: Vec<i8> = input
        .trim()
        .split(',')
        .map(|number| number.parse().unwrap())
        .collect();

    let mut newborns = vec![];
    for _ in 0..80 {
        for fish in fishes.iter_mut() {
            if *fish == 0 {
                *fish = 6;
                newborns.push(8);
            } else {
                *fish -= 1;
            }
        }

        fishes.extend(newborns.iter());
        newborns.clear();
    }

    println!("part1 = {}", fishes.len());
}
