use std::collections::HashSet;

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
    let mut basins = vec![];
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
                ugh.push(*number);
                basins.push((y as usize, x as usize));
            }
        }
    }

    let part1: u32 = ugh.into_iter().map(|x| x + 1).sum();
    println!("part1 = {}", part1);

    //

    // println!("basis = {:?}", basins);

    let mut basin_sizes = vec![];
    for (y, x) in basins {
        // println!("size = {}", get_basin_size(&grid, y, x));
        basin_sizes.push(get_basin_size(&grid, y, x));
    }

    basin_sizes.sort();
    let biggets_basins: u32 = basin_sizes.into_iter().rev().take(3).product();
    println!("part2 = {}", biggets_basins);
}

fn get_basin_size(grid: &Grid, y: usize, x: usize) -> u32 {
    let mut size = 0;
    let mut explored: HashSet<(usize, usize)> = HashSet::new();
    let mut to_explore: Vec<(usize, usize)> = vec![(y, x)];

    while let Some((y, x)) = to_explore.pop() {
        if explored.contains(&(y, x)) {
            continue;
        }

        size += 1;
        explored.insert((y, x));

        for neighbor in neighbors(grid, y, x) {
            to_explore.push(neighbor);
        }
    }

    size
}

fn neighbors(grid: &Grid, y: usize, x: usize) -> Vec<(usize, usize)> {
    let y = y as isize;
    let x = x as isize;

    let neighbors = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    neighbors
        .into_iter()
        .map(|(dy, dx)| (y + dy, x + dx))
        .filter(|&(y, x)| match grid.get(y, x) {
            None => false,
            Some(9) => false,
            _ => true,
        })
        .map(|(y, x)| (y as usize, x as usize))
        .collect()
}
