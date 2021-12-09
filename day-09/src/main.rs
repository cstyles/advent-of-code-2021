use std::collections::HashSet;

struct Grid(Vec<Vec<u32>>);

impl Grid {
    pub fn new(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();

        Self(grid)
    }

    fn get(&self, y: usize, x: usize) -> u32 {
        self.0[y][x]
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn width(&self) -> usize {
        self.0.first().unwrap().len()
    }

    fn neighbor_coords(&self, y: usize, x: usize) -> [Option<(usize, usize)>; 4] {
        [
            y.checked_sub(1).map(|y| (y, x)),
            x.checked_sub(1).map(|x| (y, x)),
            // y.lt(&(self.height() - 1)).then_some((y + 1, x)) // need nightly for this
            // x.lt(&(self.width() - 1)).then_some((y, x + 1)) // need nightly for this
            if y >= self.height() - 1 {
                None
            } else {
                Some((y + 1, x))
            },
            if x >= self.width() - 1 {
                None
            } else {
                Some((y, x + 1))
            },
        ]
    }

    fn neighbor_values(&self, y: usize, x: usize) -> impl Iterator<Item = &u32> {
        self.neighbor_coords(y, x)
            .into_iter()
            .flatten()
            .map(|(y, x)| &self.0[y][x])
    }

    fn min_neighbor(&self, y: usize, x: usize) -> u32 {
        *self.neighbor_values(y, x).min().unwrap()
    }
}

fn main() {
    let input = include_str!("../input.txt");
    // let input = include_str!("../test.txt");

    let grid = Grid::new(input);
    let mut low_points = vec![];
    let mut basins = vec![];

    for (y, line) in grid.0.iter().enumerate() {
        for (x, &height) in line.iter().enumerate() {
            if height < grid.min_neighbor(y, x) {
                low_points.push(height);
                basins.push((y, x));
            }
        }
    }

    let part1: u32 = low_points.into_iter().map(|x| x + 1).sum();
    println!("part1 = {}", part1);

    let mut basin_sizes = vec![];
    for (y, x) in basins {
        basin_sizes.push(get_basin_size(&grid, y, x));
    }

    basin_sizes.sort_unstable();
    let part2: usize = basin_sizes.into_iter().rev().take(3).product();
    println!("part2 = {}", part2);
}

fn get_basin_size(grid: &Grid, y: usize, x: usize) -> usize {
    let mut explored = HashSet::new();
    let mut to_explore = vec![(y, x)];

    while let Some((y, x)) = to_explore.pop() {
        if explored.contains(&(y, x)) {
            continue;
        }

        explored.insert((y, x));

        for neighbor in grid
            .neighbor_coords(y, x)
            .into_iter()
            .flatten()
            .filter(|&(y, x)| grid.get(y, x) < 9)
        {
            to_explore.push(neighbor);
        }
    }

    explored.len()
}
