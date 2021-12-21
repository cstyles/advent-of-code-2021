fn main() {
    let input = include_str!("../test2.txt");

    let mut grid: Vec<Vec<u32>> = input
        .lines()
        .map(str::chars)
        .map(|chars| chars.map(char_to_u32).collect())
        .collect();

    println!("part1 = {}", find_shortest_path(&grid));

    extend_grid(&mut grid);

    for row in &grid {
        for cell in row {
            print!("{} ", cell);
        }
        println!();
    }

    // 2822 too high
    println!("part2 = {}", find_shortest_path(&grid));
}

fn find_shortest_path(grid: &[Vec<u32>]) -> u32 {
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
