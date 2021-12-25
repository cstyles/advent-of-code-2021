use std::ops::Add;

// full binary tree
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SnailfishNumber {
    Num(u8),
    Snail(Box<(SnailfishNumber, SnailfishNumber)>),
}

impl From<(SnailfishNumber, SnailfishNumber)> for SnailfishNumber {
    fn from((x, y): (SnailfishNumber, SnailfishNumber)) -> Self {
        Self::Snail(Box::new((x, y)))
    }
}

impl From<(u8, u8)> for SnailfishNumber {
    fn from((x, y): (u8, u8)) -> Self {
        Self::from((Self::Num(x), Self::Num(y)))
    }
}

impl From<u8> for SnailfishNumber {
    fn from(x: u8) -> Self {
        Self::Num(x)
    }
}

impl From<&str> for SnailfishNumber {
    fn from(string: &str) -> Self {
        use crate::parser::SnailfishParser;

        let (rest, number) = SnailfishParser::parse(string).unwrap();
        assert_eq!("", rest);
        number
    }
}

impl Add<SnailfishNumber> for SnailfishNumber {
    type Output = SnailfishNumber;

    fn add(self, rhs: SnailfishNumber) -> Self::Output {
        Self::from((self, rhs))
    }
}

impl SnailfishNumber {
    fn reduce(&mut self) {
        // try exploding
        // if exploded, go to start
        // else try splitting
        // if split, go to start
        // if not, return
    }

    // Return true if exploded, false otherwise
    // This doesn't work :(
    // fn explode_recursive<'a>(&'a mut self, depth: usize, left: &mut Option<&'a mut u8>) -> bool {
    //     match self {
    //         SnailfishNumber::Num(num) => {
    //             *left = Some(num);
    //             false
    //         }
    //         SnailfishNumber::Snail(snail) => {
    //             if depth >= 4 {
    //                 match **snail {
    //                     (Self::Num(x), Self::Num(y)) => {
    //                         if let Some(left) = *left {
    //                             *left += x;
    //                         }

    //                         // TODO: right

    //                         *self = Self::Num(0);
    //                     }
    //                     _ => unreachable!("can't explode a non-Snail)"),
    //                 };

    //                 true
    //             } else {
    //                 snail.0.explode_recursive(depth + 1, left)
    //             }
    //         }
    //     }
    // }

    // [[[ [1,2], [3,4] ], 1], 2]
    // fn explode(&mut self) -> bool {
    //     let mut left: Option<&mut u8> = None;
    //     let mut ugh = self.clone();
    //     let mut stack: Vec<&mut SnailfishNumber> = vec![&mut ugh];

    //     while let Some(current) = stack.pop() {
    //         match current {
    //             SnailfishNumber::Num(num) => left = Some(num),
    //             SnailfishNumber::Snail(snail) => {
    //                 if stack.len() < 3 {
    //                     // 3 (i.e., 4 - 1) because we just popped
    //                     stack.push(&mut snail.1);
    //                     stack.push(&mut snail.0);
    //                 } else {
    //                     // TODO: left?
    //                 }
    //             }
    //         }
    //     }

    //     false
    // }

    // explode
    // | num -> left = num
    // | snail(x,y) -> explode(x)?; explode(y)?

    fn split(&mut self) {
        if let Self::Num(num) = *self {
            let x = num / 2;
            let y = if num % 2 == 0 { x } else { x + 1 };

            *self = Self::from((x, y));
        } else {
            unreachable!("can't split a Snail");
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            SnailfishNumber::Num(num) => *num as u32,
            SnailfishNumber::Snail(snail) => 3 * snail.0.magnitude() + 2 * snail.1.magnitude(),
        }
    }

    pub fn to_arr(&self) -> [Option<u8>; 32] {
        let mut vec = [None; 32];
        let mut stack = vec![(self, 1)];

        while let Some((current, current_index)) = stack.pop() {
            match current {
                SnailfishNumber::Num(num) => {
                    vec[current_index] = Some(*num);
                }
                SnailfishNumber::Snail(snail) => {
                    let children_indices = crate::tree::children_indices(current_index);
                    stack.push((&snail.0, children_indices.0));
                    stack.push((&snail.1, children_indices.1));
                }
            };
        }

        vec
    }
}

mod tests {
    #[allow(unused)]
    use super::SnailfishNumber;

    #[test]
    fn add() {
        let x = SnailfishNumber::from(1);
        let y = SnailfishNumber::from(2);
        assert_eq!(SnailfishNumber::from((1, 2)), x + y);

        let x = SnailfishNumber::from((1, 2));
        let y = SnailfishNumber::from((5, 6));
        assert_eq!(SnailfishNumber::from("[[1,2],[5,6]]"), x + y);
    }

    #[test]
    fn explode() {
        let mut x = SnailfishNumber::from("[[[[[9,8],1],2],3],4]");
        // x.explode();
        assert_eq!(SnailfishNumber::from("[[[[0,9],2],3],4]"), x);
    }

    #[test]
    fn split() {
        let mut snail = SnailfishNumber::from(10);
        snail.split();
        assert_eq!(SnailfishNumber::from((5, 5)), snail);

        let mut snail = SnailfishNumber::from(13);
        snail.split();
        assert_eq!(SnailfishNumber::from((6, 7)), snail);
    }

    #[test]
    fn magnitudes() {
        assert_eq!(143, SnailfishNumber::from("[[1,2],[[3,4],5]]").magnitude());
        assert_eq!(
            1384,
            SnailfishNumber::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").magnitude()
        );
        assert_eq!(
            445,
            SnailfishNumber::from("[[[[1,1],[2,2]],[3,3]],[4,4]]").magnitude()
        );
        assert_eq!(
            791,
            SnailfishNumber::from("[[[[3,0],[5,3]],[4,4]],[5,5]]").magnitude()
        );
        assert_eq!(
            1137,
            SnailfishNumber::from("[[[[5,0],[7,4]],[5,5]],[6,6]]").magnitude()
        );
        assert_eq!(
            3488,
            SnailfishNumber::from("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
                .magnitude()
        );
    }
}
