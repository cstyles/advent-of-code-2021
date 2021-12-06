use std::collections::HashMap;

fn main() {
    // let input = include_str!("../test-input.txt");
    let input = include_str!("../input.txt");

    let mut fishes: Vec<i8> = input
        .trim()
        .split(',')
        .map(|number| number.parse().unwrap())
        .collect();

    let fishes2 = fishes.clone();
    let mut newborns = vec![];
    for _day in 0..80 {
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

    // Map where the key is a day number and the value is
    // the number of descendents of a fish born on that day
    let mut dynprog: HashMap<i16, u64> = HashMap::with_capacity(256);

    let total_days = 256;

    for orig_day in (-9..=(total_days - 9)).rev() {
        let mut day = orig_day;

        let mut descendents = 0;
        let mut newborn = true;

        loop {
            if newborn {
                day += 9;
                newborn = false;
            } else {
                day += 7;
            }

            if day <= total_days {
                let diff = dynprog.get(&day).unwrap_or(&0) + 1;
                descendents += diff;
            } else {
                dynprog.insert(orig_day, descendents);
                break;
            }
        }
    }

    let mut total = fishes2.len() as u64;
    for fish in fishes2 {
        let birthday = (fish as i16) - 8;
        let descendents = dynprog.get(&birthday).unwrap_or(&0);
        total += descendents;
    }

    println!("part2 = {}", total);
}
