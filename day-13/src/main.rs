#[derive(Debug, Clone, Copy)]
enum Fold {
    X(usize),
    Y(usize),
}

impl From<&str> for Fold {
    fn from(string: &str) -> Self {
        let (_, string) = string.rsplit_once(' ').unwrap();
        let (var, number) = string.split_once('=').unwrap();
        let number = number.parse().unwrap();

        match var {
            "y" => Fold::Y(number),
            "x" => Fold::X(number),
            _ => unreachable!("bad var: {}", var),
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let (dots, folds) = input.split_once("\n\n").unwrap();
    let dots: Vec<(usize, usize)> = dots
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(x, y)| (y.parse().unwrap(), x.parse().unwrap()))
        .collect();

    let folds: Vec<Fold> = folds.lines().map(Fold::from).collect();

    // TODO: Dynamic
    let mut grid = vec![vec![false; 1311]; 895];
    // let mut grid = vec![vec![false; 11]; 15];
    init_grid(&mut grid, dots);

    grid = fold_paper(grid, folds[0]);
    println!("part1 = {}", count_dots(&grid));

    for fold in folds.into_iter().skip(1) {
        grid = fold_paper(grid, fold);
    }

    draw_grid(&grid);
}

fn init_grid(grid: &mut [Vec<bool>], dots: Vec<(usize, usize)>) {
    for (y, x) in dots {
        grid[y][x] = true;
    }
}

#[allow(unused)]
fn draw_grid(grid: &[Vec<bool>]) {
    for row in grid.iter() {
        for cell in row {
            match cell {
                true => print!("#"),
                false => print!("."),
            }
        }
        println!();
    }
}

fn count_dots(grid: &[Vec<bool>]) -> usize {
    grid.iter().flatten().filter(|&&x| x).count()
}

fn fold_paper(mut grid: Vec<Vec<bool>>, fold: Fold) -> Vec<Vec<bool>> {
    let mut mirrored_dots = vec![];

    match fold {
        Fold::X(fold_column) => {
            for (y, row) in grid.iter().enumerate() {
                for (x, is_dot) in row.iter().enumerate().skip(fold_column + 1) {
                    if *is_dot {
                        let distance_to_fold = x - fold_column;
                        let folded_x = fold_column - distance_to_fold;
                        mirrored_dots.push((y, folded_x));
                    }
                }
            }

            for (y, x) in mirrored_dots.into_iter() {
                grid[y][x] = true;
            }

            for row in grid.iter_mut() {
                row.truncate(fold_column);
            }
        }
        Fold::Y(fold_row) => {
            for (y, row) in grid.iter().enumerate().skip(fold_row + 1) {
                for (x, is_dot) in row.iter().enumerate() {
                    if *is_dot {
                        let distance_to_fold = y - fold_row;
                        let folded_y = fold_row - distance_to_fold;
                        mirrored_dots.push((folded_y, x));
                    }
                }
            }

            for (y, x) in mirrored_dots.into_iter() {
                grid[y][x] = true;
            }

            grid.truncate(fold_row);
        }
    }

    grid
}
