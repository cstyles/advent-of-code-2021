use std::ops::Add;

#[derive(Debug)]
enum Cmd {
    Forward(i32),
    Down(i32),
    Up(i32),
}

#[derive(Debug, Default)]
struct Position {
    x: i32,
    y: i32,
}

impl Add<&Cmd> for Position {
    type Output = Position;

    fn add(self, cmd: &Cmd) -> Self::Output {
        match cmd {
            Cmd::Forward(magnitude) => Position {
                x: self.x + magnitude,
                ..self
            },
            Cmd::Down(magnitude) => Position {
                y: self.y + magnitude,
                ..self
            },
            Cmd::Up(magnitude) => Position {
                y: self.y - magnitude,
                ..self
            },
        }
    }
}

impl From<&str> for Cmd {
    fn from(string: &str) -> Self {
        let (direction, magnitude) = string.split_once(' ').unwrap();
        let magnitude: i32 = magnitude.parse().unwrap();

        match direction.chars().next().unwrap() {
            'f' => Cmd::Forward(magnitude),
            'd' => Cmd::Down(magnitude),
            'u' => Cmd::Up(magnitude),
            _ => panic!("Bad command: {}", string),
        }
    }
}

fn main() {
    let commands: Vec<Cmd> = include_str!("../input.txt")
        .lines()
        .map(Cmd::from)
        .collect();

    let final_position = commands
        .iter()
        .fold(Position::default(), |acc, cmd| acc + cmd);

    println!("part1 = {}", final_position.x * final_position.y);
}
