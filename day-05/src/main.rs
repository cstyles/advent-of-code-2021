use std::ops::Add;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

/// For converting a pair of coordinates to a Point
impl From<&str> for Point {
    fn from(pair: &str) -> Self {
        let (x, y) = pair.split_once(',').unwrap();

        Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
struct Line {
    start: Point,
    end: Point,
}

/// For converting a line of input to a Line type
impl From<&str> for Line {
    fn from(line: &str) -> Self {
        let (start, end) = line.split_once(" -> ").unwrap();
        let start = Point::from(start);
        let end = Point::from(end);

        Self { start, end }
    }
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    // TODO: Iterator!
    fn points(&self) -> Vec<Point> {
        let diff = if self.is_horizontal() {
            if self.start.x <= self.end.x {
                Point::new(1, 0)
            } else {
                Point::new(-1, 0)
            }
        } else if self.is_vertical() {
            if self.start.y <= self.end.y {
                Point::new(0, 1)
            } else {
                Point::new(0, -1)
            }
        } else {
            unreachable!("not reachable in part 1");
        };

        let mut start = self.start;
        let mut points: Vec<Point> = vec![start];

        while start != self.end {
            start = start + diff;
            points.push(start);
        }

        points
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let lines: Vec<Line> = input
        .lines()
        .map(Line::from)
        .filter(|line| line.is_vertical() || line.is_horizontal())
        .collect();

    let mut grid = vec![vec![0u16; 1000]; 1000];

    for line in lines {
        for point in line.points() {
            grid[point.x as usize][point.y as usize] += 1;
        }
    }

    let mut count = 0;
    for row in grid {
        for cell in row {
            if cell > 1 {
                count += 1;
            }
        }
    }

    println!("part1 = {}", count);
}
