fn main() {
    let input = include_str!("../input.txt");

    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(str::chars)
        .map(|chars| chars.map(char_to_u32).collect())
        .collect();

    let rows = grid.len();
    let columns = grid[0].len();
    let last_row = rows - 1;
    let last_column = columns - 1;

    dbg!(rows, columns);

    // TODO: u32?
    let mut dynprog = vec![vec![0u32; columns]; rows];
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

    for (y, row) in grid.iter().enumerate().rev().skip(1) {
        for (x, cell) in row.iter().enumerate().rev().skip(1) {
            dynprog[y][x] = std::cmp::min(dynprog[y][x + 1], dynprog[y + 1][x]) + cell;
        }
    }

    // Account for the fact that the first cell is never entered)
    dynprog[0][0] -= grid[0][0];

    // for row in &dynprog {
    //     for cell in row {
    //         print!("{} ", cell);
    //     }
    //     println!();
    // }

    println!("part1 = {}", dynprog[0][0]);
}

fn char_to_u32(c: char) -> u32 {
    (c as u32) - ('0' as u32)
}
