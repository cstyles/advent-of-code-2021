use std::{
    collections::hash_map::DefaultHasher,
    fmt::Display,
    hash::{Hash, Hasher},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    East,
    South,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Tile::Empty => '.',
                Tile::East => '>',
                Tile::South => 'v',
            }
        )
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '>' => Self::East,
            'v' => Self::South,
            _ => unreachable!("bad char: {}", c),
        }
    }
}

#[derive(Hash)]
struct Grid {
    inner: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Grid {
    fn south(&self, y: usize, x: usize) -> (usize, usize) {
        if y >= self.height - 1 {
            (0, x)
        } else {
            (y + 1, x)
        }
    }

    fn east(&self, y: usize, x: usize) -> (usize, usize) {
        if x >= self.width - 1 {
            (y, 0)
        } else {
            (y, x + 1)
        }
    }

    // fn at(&self, y: usize, x: usize) -> Tile {
    fn at(&self, (y, x): (usize, usize)) -> Tile {
        self.inner[self.coords_to_offset((y, x))]
    }

    fn coords_to_offset(&self, (y, x): (usize, usize)) -> usize {
        y * self.width + x
    }

    fn offset_to_coords(&self, offset: usize) -> (usize, usize) {
        let y = offset / self.width;
        let x = offset % self.width;

        (y, x)
    }

    fn step(&mut self) {
        for direction in [Tile::East, Tile::South] {
            let mut new_inner = self.inner.clone();

            for (i, tile) in self.inner.iter().enumerate() {
                if *tile != direction {
                    continue;
                }

                let (y, x) = self.offset_to_coords(i);
                let target = match direction {
                    Tile::East => self.east(y, x),
                    Tile::South => self.south(y, x),
                    Tile::Empty => unreachable!(),
                };

                if self.at(target) == Tile::Empty {
                    new_inner[self.coords_to_offset(target)] = direction;
                    new_inner[i] = Tile::Empty;
                }
            }

            self.inner = new_inner;
        }
    }

    #[allow(unused)]
    fn debug(&self) {
        for (i, tile) in self.inner.iter().enumerate() {
            let (y, x) = self.offset_to_coords(i);
            if x == 0 && y != 0 {
                println!();
            }
            print!("{}", tile);
        }

        println!();
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let inner: Vec<Tile> = input
        .lines()
        .map(|line| line.chars())
        .flatten()
        .map(Tile::from)
        .collect();

    let mut grid = Grid {
        inner,
        width,
        height,
    };

    let mut hash = 0;
    let mut i = 1;

    loop {
        // println!("\nStep {}:", i);
        // grid.debug();
        grid.step();

        let mut hasher = DefaultHasher::new();
        grid.hash(&mut hasher);

        if hasher.finish() == hash {
            break;
        } else {
            hash = hasher.finish();
            i += 1;
        }
    }

    println!("part1 = {}", i);
}
