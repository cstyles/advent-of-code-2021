fn main() {
    let input = include_str!("../input.txt");
    let mut sections = input.split("\n\n");

    let algorithm: Vec<bool> = sections
        .next()
        .unwrap()
        .chars()
        .map(pixel_to_bool)
        .collect();

    let mut image: Vec<Vec<bool>> = sections
        .next()
        .unwrap()
        .lines()
        .map(|line| line.chars().map(pixel_to_bool).collect())
        .collect();

    debug_image(&image);
    extend_image(&mut image);

    for i in 0..50 {
        image = apply_algorithm(image, &algorithm, i);
        debug_image(&image);
    }

    // 5697 too high
    // 5507 too high
    // 5469 too high
    println!("part1 = {}", count_pixels(&image));
}

fn get_index(image: &[Vec<bool>], y: usize, x: usize) -> usize {
    let mut bit_pos = 8;
    let mut result = 0;

    for dy in -1..=1 {
        for dx in -1..=1 {
            let y = ((y as isize) + dy) as usize;
            let x = ((x as isize) + dx) as usize;

            let row = &image[y];
            let n = row[x];
            let n = usize::from(n) << bit_pos;
            // let n = usize::from(image[y][x]) << bit_pos;
            result |= n;

            bit_pos -= 1;
        }
    }

    result
}

fn pixel_to_bool(pixel: char) -> bool {
    match pixel {
        '#' => true,
        '.' => false,
        _ => unreachable!("bad char: {}", pixel),
    }
}

fn bool_to_pixel(b: bool) -> char {
    match b {
        true => '#',
        false => '.',
    }
}

fn extend_image(image: &mut Vec<Vec<bool>>) {
    const N: usize = 70;
    let len = image.len();

    let empty_row = vec![false; len + 2 * N];
    let empty_wedge = vec![false; N];

    // Extend vertically up
    for i in 0..N {
        image.insert(i, empty_row.clone());
    }

    // Extend middle rows horizontally
    for row in image.iter_mut().skip(N).take(len) {
        *row = [&empty_wedge, row, &empty_wedge]
            .into_iter()
            .flatten()
            .copied()
            .collect();
    }

    // Extend vertically down
    for _ in 0..N - 1 {
        image.push(empty_row.clone())
    }
    image.push(empty_row);
}

fn debug_image(image: &[Vec<bool>]) {
    for row in image {
        for b in row {
            print!("{}", bool_to_pixel(*b));
        }
        println!();
    }
    println!();
}

fn apply_algorithm(image: Vec<Vec<bool>>, algorithm: &[bool], round: usize) -> Vec<Vec<bool>> {
    let mut image2 = image.clone();

    for (y, row) in image.iter().enumerate().skip(1).take(image.len() - 2) {
        for (x, _pixel) in row.iter().enumerate().skip(1).take(row.len() - 2) {
            let index = get_index(&image, y, x);
            image2[y][x] = algorithm[index];
        }
    }

    image2
}

fn count_pixels(image: &[Vec<bool>]) -> usize {
    image.iter().flatten().filter(|&&b| b).count()
}
