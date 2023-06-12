use std::ops::RangeInclusive;

#[derive(Debug)]
struct Step {
    on: bool,
    x: RangeInclusive<usize>,
    y: RangeInclusive<usize>,
    z: RangeInclusive<usize>,
}

impl TryFrom<&str> for Step {
    type Error = ();

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        let (on, ranges) = string.split_once(' ').unwrap();
        let on = on == "on";

        let mut ranges = ranges.split(',');
        let x = parse_range(ranges.next().unwrap()).ok_or(())?;
        let y = parse_range(ranges.next().unwrap()).ok_or(())?;
        let z = parse_range(ranges.next().unwrap()).ok_or(())?;

        Ok(Self { on, x, y, z })
    }
}

impl Step {
    /// Returns an iterator over all coordinate triplets in the sub-cube
    fn coords(&self) -> impl Iterator<Item = (usize, usize, usize)> + '_ {
        // TODO: Might be responsible for the stack overflow in debug builds
        self.x
            .clone()
            .flat_map(|x| std::iter::repeat(x).zip(self.y.clone()))
            .flat_map(|xy| std::iter::repeat(xy).zip(self.z.clone()))
            .map(|((x, y), z)| (x, y, z))
    }
}

#[derive(Debug)]
struct Step2 {
    on: bool,
    xs: (isize, isize),
    ys: (isize, isize),
    zs: (isize, isize),
}

impl Step2 {
    fn parse(string: &str) -> Self {
        let (on, ranges) = string.split_once(' ').unwrap();
        let on = on == "on";

        let mut ranges = ranges.split(',');
        let xs = parse_range2(ranges.next().unwrap());
        let ys = parse_range2(ranges.next().unwrap());
        let zs = parse_range2(ranges.next().unwrap());

        Self { on, xs, ys, zs }
    }

    fn intersect(&self, other: &Self) -> Option<Self> {
        if other.xs.1 < self.xs.0
            || other.xs.0 > self.xs.1
            || other.ys.1 < self.ys.0
            || other.ys.0 > self.ys.1
            || other.zs.1 < self.zs.0
            || other.zs.0 > self.zs.1
        {
            return None;
        }

        let min_x = self.xs.0.max(other.xs.0);
        let max_x = self.xs.1.min(other.xs.1);

        let min_y = self.ys.0.max(other.ys.0);
        let max_y = self.ys.1.min(other.ys.1);

        let min_z = self.zs.0.max(other.zs.0);
        let max_z = self.zs.1.min(other.zs.1);

        let xs = (min_x, max_x);
        let ys = (min_y, max_y);
        let zs = (min_z, max_z);

        // let on = match (self.on, other.on) {
        //     (true, true) => false,
        //     (true, false) => true,
        //     (false, true) => false,
        //     (false, false) => true,
        // };
        let on = !other.on;

        Some(Self { on, xs, ys, zs })
    }

    fn volume(&self) -> isize {
        let x = self.xs.1 - self.xs.0 + 1;
        let y = self.ys.1 - self.ys.0 + 1;
        let z = self.zs.1 - self.zs.0 + 1;

        let sign = match self.on {
            true => 1,
            false => -1,
        };

        x * y * z * sign
    }
}

fn parse_range2(string: &str) -> (isize, isize) {
    // Trim leading "x=", etc.
    let string = &string[2..];

    let (left, right) = string.split_once("..").unwrap();
    let left: isize = left.parse().unwrap();
    let right: isize = right.parse().unwrap();

    (left, right)
}

fn parse_range(string: &str) -> Option<RangeInclusive<usize>> {
    // Trim leading "x=", etc.
    let string = &string[2..];

    let (left, right) = string.split_once("..").unwrap();
    let left = left.parse::<isize>().unwrap();
    let right = right.parse::<isize>().unwrap();

    // If the range is entirely outside the initialization area, exit early
    if (left < -50 && right < -50) || left > 50 && right > 50 {
        return None;
    }

    // We only care about coordinates inside the initialization area
    let left = left.clamp(-50, 50);
    let right = right.clamp(-50, 50);

    // Swap left and right so that the range is ascending (and thus not empty)
    let (left, right) = if left > right {
        (right, left)
    } else {
        (left, right)
    };

    Some(RangeInclusive::new(
        (left + 50) as usize,
        (right + 50) as usize,
    ))
}

fn main() {
    // part1();
    part2();
}

fn part2() {
    let input = include_str!("../input.txt");
    let steps: Vec<Step2> = input.lines().map(Step2::parse).collect();

    let mut cubes: Vec<Step2> = vec![];
    for step in steps {
        let to_push: Vec<Step2> = cubes
            .iter()
            .filter_map(|cube| step.intersect(cube))
            .collect();
        cubes.extend(to_push);

        if step.on {
            // Always push the entire on cube
            cubes.push(step);
        }
    }

    let total_volume: isize = cubes.into_iter().map(|cube| cube.volume()).sum();
    println!("part2 = {total_volume}");
}

#[allow(unused)]
fn part1() {
    let input = include_str!("../input.txt");
    let steps: Vec<Step> = input
        .lines()
        .filter_map(|line| Step::try_from(line).ok())
        .collect();

    // Inddex 0 => -50, 50 => 0, 100 => 50
    let mut cuboid = [false; 101 * 101 * 101];

    for step in &steps {
        for (x, y, z) in step.coords() {
            cuboid[z * 101 * 101 + y * 101 + x] = step.on;
        }
    }

    let count = cuboid.into_iter().filter(|x| *x).count();

    println!("part1 = {}", count);
}
