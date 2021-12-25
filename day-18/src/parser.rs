use crate::snailfish::SnailfishNumber;

use nom::{
    bytes::complete::tag,
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

fn just_num<'a>() -> impl Parser<&'a str, SnailfishNumber, ()> {
    nom::character::complete::u8.map(SnailfishNumber::Num)
}

fn either<'a>() -> impl Parser<&'a str, SnailfishNumber, ()> {
    just_num().or(SnailfishParser {})
}

fn pair<'a>() -> impl Parser<&'a str, SnailfishNumber, ()> {
    separated_pair(either(), tag(","), either()).map(SnailfishNumber::from)
}

// We have to give a name to this parser to avoid "opaque type" errors
pub struct SnailfishParser {}

impl<'input> Parser<&'input str, SnailfishNumber, ()> for SnailfishParser {
    fn parse(&mut self, input: &'input str) -> IResult<&'input str, SnailfishNumber, ()> {
        delimited(tag("["), pair(), tag("]")).parse(input)
    }
}

impl SnailfishParser {
    pub fn parse(input: &str) -> IResult<&str, SnailfishNumber, ()> {
        Self {}.parse(input)
    }
}
