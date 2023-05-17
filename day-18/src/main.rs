use std::ops::Add;

use ExplodeResult::*;
use SplitResult::*;
use Tree::*;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Tree {
    Number(u32),
    Pair { left: Box<Tree>, right: Box<Tree> },
}

const MAX_LAYERS: usize = 4;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum ExplodeResult {
    Exploded,
    DidNotExplode,
    ExplodingLeft(u32),
    ExplodingRight(u32),
    ExplodingBoth((u32, u32)),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum SplitResult {
    DidSplit,
    DidNotSplit,
}

impl SplitResult {
    fn or_else<F: FnMut() -> Self>(self, mut f: F) -> Self {
        match self {
            DidSplit => DidSplit,
            DidNotSplit => f(),
        }
    }
}

impl Tree {
    fn assume_number(&self) -> u32 {
        match self {
            Number(number) => *number,
            _ => unreachable!(),
        }
    }

    fn explode(&mut self) -> ExplodeResult {
        match self.try_explode(0) {
            DidNotExplode => DidNotExplode,
            _ => Exploded, // erase in-progress explodes
        }
    }

    // fn try_explode(&mut self, layer: usize) -> (Option<u32>, Option<u32>) {
    fn try_explode(&mut self, layer: usize) -> ExplodeResult {
        match (self, layer) {
            (Number(_), _) => DidNotExplode,
            (Pair { left, right }, MAX_LAYERS) => {
                let left = left.assume_number();
                let right = right.assume_number();
                ExplodingBoth((left, right))
            }
            (Pair { left, right }, layer) => {
                match left.try_explode(layer + 1) {
                    DidNotExplode => {}
                    Exploded => return Exploded,
                    ExplodingRight(b) => {
                        right.explode_right(b);
                        return Exploded;
                    }
                    x @ ExplodingLeft(_) => return x,
                    ExplodingBoth((a, b)) => {
                        **left = Number(0);
                        right.explode_right(b);
                        return ExplodingLeft(a);
                    }
                };

                match right.try_explode(layer + 1) {
                    DidNotExplode => {}
                    Exploded => return Exploded,
                    x @ ExplodingRight(_) => return x,
                    ExplodingLeft(a) => {
                        left.explode_left(a);
                        return Exploded;
                    }
                    ExplodingBoth((a, b)) => {
                        **right = Number(0);
                        left.explode_left(a);
                        return ExplodingRight(b);
                    }
                };

                DidNotExplode
            }
        }
    }

    // Find the first number to the left of some exploded number and add to it
    fn explode_left(&mut self, value: u32) {
        match self {
            Number(number) => *number += value,
            Pair { right, .. } => right.explode_left(value),
        }
    }

    // Find the first number to the right of some exploded number and add to it
    fn explode_right(&mut self, value: u32) {
        match self {
            Number(number) => *number += value,
            Pair { left, .. } => left.explode_right(value),
        }
    }

    // Returns an Option to allow short-circuiting
    fn split(&mut self) -> SplitResult {
        match self {
            Pair { left, right } => left.split().or_else(|| right.split()),
            Number(number) if *number > 9 => {
                let half = *number / 2;
                let (left, right) = if *number % 2 == 0 {
                    (half, half)
                } else {
                    (half, half + 1)
                };

                let left = Box::new(Number(left));
                let right = Box::new(Number(right));

                *self = Pair { left, right };
                DidSplit
            }
            _ => DidNotSplit,
        }
    }

    fn parse_inner<I: Iterator<Item = char>>(chars: &mut I) -> Self {
        match chars.next().unwrap() {
            '[' => {
                let left = Tree::parse_inner(chars);
                assert_eq!(Some(','), chars.next());
                let right = Tree::parse_inner(chars);
                assert_eq!(Some(']'), chars.next());

                Self::Pair {
                    left: Box::new(left),
                    right: Box::new(right),
                }
            }
            c => {
                let number = c.to_digit(10).unwrap();
                Self::Number(number)
            }
        }
    }

    fn parse(string: &str) -> Self {
        Self::parse_inner(&mut string.chars())
    }

    fn reduce(&mut self) {
        loop {
            let DidNotExplode = self.explode() else { continue };
            let DidNotSplit = self.split() else { continue };
            break;
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            Number(number) => *number,
            Pair { left, right } => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }
}

impl std::ops::Add for Tree {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Pair {
            left: Box::new(self),
            right: Box::new(rhs),
        }
    }
}

impl std::ops::Add for &Tree {
    type Output = Tree;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::Pair {
            left: Box::new(self.clone()),
            right: Box::new(rhs.clone()),
        }
    }
}

impl std::fmt::Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number(number) => write!(f, "{number}"),
            Pair { left, right } => write!(f, "[{left},{right}]"),
        }
    }
}

//   ^
//  / \
// 1   ^
//    / \
//   ^   4
//  / \
// 2   3
fn main() {
    let input = include_str!("../input.txt");
    // let input = include_str!("../test_input.txt");
    let numbers: Vec<Tree> = input.lines().map(Tree::parse).collect();
    part1(numbers.clone());
    part2(numbers);
}

fn part1(numbers: Vec<Tree>) {
    let mut numbers = numbers.into_iter();
    let mut sum = numbers.next().unwrap();
    for number in numbers {
        sum = sum + number;
        sum.reduce();
    }

    // println!("{sum}");
    println!("part1 = {}", sum.magnitude());
}

fn part2(numbers: Vec<Tree>) {
    let mut max = 0;

    for a in &numbers {
        for b in &numbers {
            if a == b {
                continue;
            }

            let mut sum = a + b;
            sum.reduce();
            max = sum.magnitude().max(max);
        }
    }

    println!("part2 = {max}");
}

#[allow(unused)]
fn explode_test1() {
    let mut tree = Pair {
        left: Box::new(Pair {
            left: Box::new(Pair {
                left: Box::new(Pair {
                    left: Box::new(Pair {
                        left: Box::new(Number(9)),
                        right: Box::new(Number(8)),
                    }),
                    right: Box::new(Number(1)),
                }),
                right: Box::new(Number(2)),
            }),
            right: Box::new(Number(3)),
        }),
        right: Box::new(Number(4)),
    };
    println!("{tree:#?}");

    let parsed = Tree::parse("[[[[[9,8],1],2],3],4]");
    assert_eq!(parsed, tree);

    tree.try_explode(0);
    println!("{tree:#?}");

    let parsed = Tree::parse("[[[[0,9],2],3],4]");
    assert_eq!(parsed, tree);
}

#[allow(unused)]
fn explode_test2() {
    let mut tree = Pair {
        left: Box::new(Number(7)),
        right: Box::new(Pair {
            left: Box::new(Number(6)),
            right: Box::new(Pair {
                left: Box::new(Number(5)),
                right: Box::new(Pair {
                    left: Box::new(Number(4)),
                    right: Box::new(Pair {
                        left: Box::new(Number(3)),
                        right: Box::new(Number(2)),
                    }),
                }),
            }),
        }),
    };
    println!("{tree:#?}");

    let parsed = Tree::parse("[7,[6,[5,[4,[3,2]]]]]");
    assert_eq!(parsed, tree);

    tree.try_explode(0);
    println!("{tree:#?}");
    println!("{tree}");

    let parsed = Tree::parse("[7,[6,[5,[7,0]]]]");
    assert_eq!(parsed, tree);
}

#[allow(unused)]
fn split_test() {
    let a = Tree::parse("[[[[4,3],4],4],[7,[[8,4],9]]]");
    let b = Tree::parse("[1,1]");
    let mut tree = a.add(b);
    assert_eq!("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]", tree.to_string());

    assert_eq!(Exploded, tree.explode());
    assert_eq!("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]", tree.to_string());
    assert_eq!(Exploded, tree.explode());
    assert_eq!("[[[[0,7],4],[15,[0,13]]],[1,1]]", tree.to_string());
    assert_eq!(DidNotExplode, tree.explode());

    assert_eq!(DidSplit, tree.split());
    assert_eq!("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]", tree.to_string());
    assert_eq!(DidSplit, tree.split());
    assert_eq!("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]", tree.to_string());
    assert_eq!(DidNotSplit, tree.split());

    assert_eq!(Exploded, tree.explode());
    assert_eq!("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", tree.to_string());
    assert_eq!(DidNotExplode, tree.explode());
    assert_eq!(DidNotSplit, tree.split());

    // Try again, this time with `reduce`
    let a = Tree::parse("[[[[4,3],4],4],[7,[[8,4],9]]]");
    let b = Tree::parse("[1,1]");
    let mut tree = a.add(b);
    tree.reduce();
    assert_eq!("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", tree.to_string());

    // println!("{tree}");
}
