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

    // We only care abot coordinates inside the initialization area
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
