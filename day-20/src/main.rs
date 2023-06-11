fn main() {
    let input = include_str!("../input.txt");
    // let input = include_str!("../test.txt");
    let mut sections = input.split("\n\n");

    let algorithm: Vec<bool> = sections
        .next()
        .unwrap()
        .chars()
        .map(pixel_to_bool)
        .collect();

    let image: Vec<Vec<bool>> = sections
        .next()
        .unwrap()
        .lines()
        .map(|line| line.chars().map(pixel_to_bool).collect())
        .collect();

    part1(image.clone(), &algorithm);
    // part2(image, &algorithm);
}

fn part1(mut image: Vec<Vec<bool>>, algorithm: &[bool]) {
    extend_image(&mut image, 5, false);
    // debug_image(&image);
    image = apply_algorithm(image, algorithm, false);
    // debug_image(&image);
    image = apply_algorithm(image, algorithm, true);
    // debug_image(&image);

    println!("part1 = {}", count_pixels(&image));
}

fn part2(mut image: Vec<Vec<bool>>, algorithm: &[bool]) {
    extend_image(&mut image, 50, false);
    // debug_image(&image);

    // for i in 0..50 {
    for i in 0..2 {
        image = apply_algorithm(image, algorithm, todo!());
        // extend_image(&mut image, 1, false);
        extend_image(&mut image, 1, i % 2 == 0);
        // debug_image(&image);
    }
    println!("part2 = {}", count_pixels(&image));
}

fn get_index(image: &[Vec<bool>], y: usize, x: usize, fill: bool) -> usize {
    let mut bit_pos = 8;
    let mut result = 0;
    let fill_wedge = vec![fill; image.len()];

    for dy in -1..=1 {
        for dx in -1..=1 {
            let y = y.checked_add_signed(dy);
            let x = x.checked_add_signed(dx);

            let row = match y {
                Some(y) if y >= image.len() => &fill_wedge,
                Some(y) => image[y].as_slice(),
                None => &fill_wedge,
            };
            let n = match x {
                Some(x) if x >= image.len() => fill,
                Some(x) => row[x],
                None => fill,
            };
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

fn extend_image(image: &mut Vec<Vec<bool>>, n: usize, with: bool) {
    let len = image.len();

    let empty_row = vec![with; len + 2 * n];
    let empty_wedge = vec![with; n];

    // Extend vertically up
    for i in 0..n {
        image.insert(i, empty_row.clone());
    }

    // Extend middle rows horizontally
    for row in image.iter_mut().skip(n).take(len) {
        *row = [&empty_wedge, row, &empty_wedge]
            .into_iter()
            .flatten()
            .copied()
            .collect();
    }

    // Extend vertically down
    for _ in 0..n - 1 {
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

fn apply_algorithm(image: Vec<Vec<bool>>, algorithm: &[bool], fill: bool) -> Vec<Vec<bool>> {
    let mut image2 = image.clone();

    for (y, row) in image.iter().enumerate() {
        for (x, _pixel) in row.iter().enumerate() {
            let index = get_index(&image, y, x, fill);
            image2[y][x] = algorithm[index];
        }
    }

    image2
}

fn count_pixels(image: &[Vec<bool>]) -> usize {
    image.iter().flatten().filter(|&&b| b).count()
}
