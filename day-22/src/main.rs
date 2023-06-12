#[derive(Debug)]
struct Step {
    on: bool,
    xs: (isize, isize),
    ys: (isize, isize),
    zs: (isize, isize),
}

impl Step {
    fn parse(string: &str) -> Self {
        let (on, ranges) = string.split_once(' ').unwrap();
        let on = on == "on";

        let mut ranges = ranges.split(',');
        let xs = parse_range(ranges.next().unwrap());
        let ys = parse_range(ranges.next().unwrap());
        let zs = parse_range(ranges.next().unwrap());

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

    fn clamp(mut self) -> Option<Self> {
        if self.xs.0 < -50
            || self.xs.1 > 50
            || self.ys.0 < -50
            || self.ys.1 > 50
            || self.zs.0 < -50
            || self.zs.1 > 50
        {
            return None;
        }

        self.xs.0 = self.xs.0.max(-50);
        self.xs.1 = self.xs.1.min(50);
        self.ys.1 = self.ys.1.max(-50);
        self.ys.1 = self.ys.1.min(50);
        self.zs.1 = self.zs.1.max(-50);
        self.zs.1 = self.zs.1.min(50);

        Some(self)
    }
}

fn parse_range(string: &str) -> (isize, isize) {
    // Trim leading "x=", etc.
    let string = &string[2..];

    let (left, right) = string.split_once("..").unwrap();
    let left: isize = left.parse().unwrap();
    let right: isize = right.parse().unwrap();

    (left, right)
}

fn main() {
    let input = include_str!("../input.txt");
    let steps = input.lines().map(Step::parse);
    let part1_steps = steps.clone().filter_map(Step::clamp).collect();
    let part2_steps = steps.collect();

    part(1, part1_steps);
    part(2, part2_steps);
}

fn part(part: u8, steps: Vec<Step>) {
    let mut cubes: Vec<Step> = vec![];
    for step in steps {
        let to_push: Vec<Step> = cubes
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
    println!("part{part} = {total_volume}");
}
