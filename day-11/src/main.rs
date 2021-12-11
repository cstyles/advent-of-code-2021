use std::collections::HashSet;

struct Grid(Vec<Vec<u32>>);

impl Grid {
    fn height(&self) -> usize {
        self.0.len()
    }

    fn width(&self) -> usize {
        self.0.first().unwrap().len()
    }

    // Returns the number of flashes in this step
    fn step(&mut self) -> usize {
        let count = self.flash();
        self.reset_flashed();

        count
    }

    fn flash(&mut self) -> usize {
        let mut flashed: HashSet<(usize, usize)> = HashSet::new();
        let mut stack = vec![];

        // By pushing every octopus in, we increment them all by one
        for y in 0..self.height() {
            for x in 0..self.width() {
                stack.push((y, x));
            }
        }

        while let Some((y, x)) = stack.pop() {
            if flashed.contains(&(y, x)) {
                continue;
            }

            self.0[y][x] += 1;

            if self.0[y][x] > 9 {
                flashed.insert((y, x));
                for neighbor in neighbor_coords(y, x).into_iter().flatten() {
                    stack.push(neighbor);
                }
            }
        }

        flashed.len()
    }

    // Reset any octopuses that flashed to 0
    fn reset_flashed(&mut self) {
        for row in self.0.iter_mut() {
            for octopus in row.iter_mut() {
                if *octopus > 9 {
                    *octopus = 0;
                }
            }
        }
    }

    #[allow(unused)]
    fn debug(&self) {
        for row in self.0.iter() {
            for octopus in row {
                print!("{}", octopus);
            }

            println!();
        }

        println!();
    }
}

fn neighbor_coords(y: usize, x: usize) -> [Option<(usize, usize)>; 9] {
    [
        y.checked_sub(1).zip(x.checked_sub(1)),           // -1, -1
        y.checked_sub(1).map(|y| (y, x)),                 // -1,  0
        y.checked_sub(1).zip(x.bounded_add::<9>(1)),      // -1,  1
        x.checked_sub(1).map(|x| (y, x)),                 //  0, -1
        Some((y, x)),                                     //  0,  0
        x.bounded_add::<9>(1).map(|x| (y, x)),            //  0,  1
        y.bounded_add::<9>(1).zip(x.checked_sub(1)),      //  1, -1
        y.bounded_add::<9>(1).map(|y| (y, x)),            //  1,  0
        y.bounded_add::<9>(1).zip(x.bounded_add::<9>(1)), //  1,  1
    ]
}

fn main() {
    let input = include_str!("../input.txt");
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(str::chars)
        .map(|chars| chars.map(|c| c.to_digit(10).unwrap()))
        .map(Iterator::collect)
        .collect();

    let mut grid = Grid(grid);
    let mut flashes = 0;
    let mut step = 0;

    loop {
        step += 1;
        let flashes_on_this_step = grid.step();
        flashes += flashes_on_this_step;

        if flashes_on_this_step == 100 {
            println!("part2 = {}", step);
            break;
        }

        if step == 100 {
            println!("part1 = {}", flashes);
        }
    }
}

trait BoundedAdd {
    type Output;

    fn bounded_add<const N: usize>(&self, rhs: Self) -> Option<Self::Output>;
}

impl BoundedAdd for usize {
    type Output = usize;

    fn bounded_add<const N: usize>(&self, rhs: Self) -> Option<Self::Output> {
        let sum = self + rhs;

        if sum > N {
            None
        } else {
            Some(sum)
        }
    }
}
