use std::collections::BinaryHeap;

fn main() {
    let input = include_str!("../input.txt");

    let mut grid: Vec<Vec<u32>> = input
        .lines()
        .map(str::chars)
        .map(|chars| chars.map(char_to_u32).collect())
        .collect();

    println!("part1 = {}", find_shortest_path_naive(&grid));

    extend_grid(&mut grid);
    println!("part2 = {}", dijkstra(&grid));
}

/// Assumes we can only move down and right
fn find_shortest_path_naive(grid: &[Vec<u32>]) -> u32 {
    let rows = grid.len();
    let columns = grid[0].len();
    let last_row = rows - 1;
    let last_column = columns - 1;

    let mut dynprog = vec![vec![0u32; columns]; rows];

    // Pre-populate bottom right cell
    dynprog[last_row][last_column] = grid[last_row][last_column];

    // Handle last row
    for (x, cell) in grid.last().unwrap().iter().enumerate().rev().skip(1) {
        dynprog[last_row][x] = dynprog[last_row][x + 1] + cell;
    }

    // Handle right column
    for (y, row) in grid.iter().enumerate().rev().skip(1) {
        for (x, cell) in row.iter().enumerate().rev() {
            dynprog[y][x] = dynprog[y + 1][x] + cell;
        }
    }

    // Handle everything else
    for (y, row) in grid.iter().enumerate().rev().skip(1) {
        for (x, cell) in row.iter().enumerate().rev().skip(1) {
            dynprog[y][x] = std::cmp::min(dynprog[y][x + 1], dynprog[y + 1][x]) + cell;
        }
    }

    // Account for the fact that the first cell is never entered)
    dynprog[0][0] -= grid[0][0];

    dynprog[0][0]
}

fn char_to_u32(c: char) -> u32 {
    (c as u32) - ('0' as u32)
}

fn extend_grid(grid: &mut Vec<Vec<u32>>) {
    let rows = grid.len();
    let columns = grid[0].len();

    // Extend rightward
    for i in 1..5 {
        for row in grid.iter_mut() {
            for x in 0..columns {
                row.push(wrap_add(row[x], i));
            }
        }
    }

    // Extend downward
    for i in 1..5 {
        for y in 0..rows {
            let new_row = grid[y].iter().map(|&x| wrap_add(x, i)).collect();
            grid.push(new_row);
        }
    }
}

fn wrap_add(x: u32, i: u32) -> u32 {
    let n = x + i;

    if n > 9 {
        n - 9
    } else {
        n
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Node {
    y: usize,
    x: usize,
    distance: u32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // use `other.cmp(self)` to reverse sort and create a min-heap
        match other.distance.cmp(&self.distance) {
            // Break ties using position
            std::cmp::Ordering::Equal => (other.y, other.x).cmp(&(self.y, self.x)),
            o => o,
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(grid: &[Vec<u32>]) -> u32 {
    let rows = grid.len();
    let columns = grid[0].len();
    let last_row = rows - 1;
    let last_column = columns - 1;
    let destination = (last_row, last_column);

    let mut heap: BinaryHeap<Node> = BinaryHeap::with_capacity(rows * columns);
    let mut distances = vec![vec![u32::MAX; columns]; rows];

    distances[0][0] = 0;
    heap.push(Node {
        x: 0,
        y: 0,
        distance: 0,
    });

    while let Some(current) = heap.pop() {
        for (y, x) in neighbors(grid, current.y, current.x).into_iter().flatten() {
            let distance = distances[current.y][current.x] + grid[y][x];

            if distances[y][x] <= distance {
                // Skip if we've already found a shorter path to the node
                continue;
            } else {
                // Otherwise set/update the shortest distance to that node
                distances[y][x] = distance;
            }

            if (y, x) == destination {
                return distances[last_row][last_column];
            }

            heap.push(Node { x, y, distance });
        }
    }

    unreachable!("couldn't reach destination :(");
}

fn neighbors<T>(grid: &[Vec<T>], y: usize, x: usize) -> [Option<(usize, usize)>; 4] {
    [
        y.checked_sub(1).map(|y| (y, x)),                // Up
        y.bounded_add(grid.len(), 1).map(|y| (y, x)),    // Down
        x.checked_sub(1).map(|x| (y, x)),                // Left
        x.bounded_add(grid[0].len(), 1).map(|x| (y, x)), // Right
    ]
}

trait BoundedAdd {
    type Output;

    fn bounded_add(&self, max: Self, rhs: Self) -> Option<Self::Output>;
}

impl BoundedAdd for usize {
    type Output = usize;

    fn bounded_add(&self, max: Self, rhs: Self) -> Option<Self::Output> {
        let sum = self + rhs;

        if sum >= max {
            None
        } else {
            Some(sum)
        }
    }
}
