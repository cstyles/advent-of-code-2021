struct Grid(Vec<Vec<u32>>);

impl Grid {
    fn get(&self, y: isize, x: isize) -> Option<u32> {
        let y = if y.is_negative() {
            return None;
        } else if y as usize >= self.0.len() {
            return None;
        } else {
            y as usize
        };

        let x = if x.is_negative() {
            return None;
        } else if x as usize >= self.0.first().unwrap().len() {
            return None;
        } else {
            x as usize
        };

        Some(self.0[y][x])
    }
}

fn main() {
    let input = include_str!("../input.txt");
    // let input = include_str!("../test.txt");

    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect();
    let grid = Grid(grid);

    let mut ugh = vec![];
    for (y, line) in grid.0.iter().enumerate() {
        let y = y as isize;
        for (x, number) in line.iter().enumerate() {
            let x = x as isize;

            let neighbors = [
                grid.get(y - 1, x - 0),
                grid.get(y - 0, x - 1),
                grid.get(y + 1, x - 0),
                grid.get(y - 0, x + 1),
            ];

            let min_neighbor = neighbors.into_iter().flatten().min().unwrap();
            if *number < min_neighbor {
                println!("{}", *number);
                ugh.push(*number);
            }
        }
    }

    let part1: u32 = ugh.into_iter().map(|x| x + 1).sum();
    println!("part1 = {}", part1);
}
