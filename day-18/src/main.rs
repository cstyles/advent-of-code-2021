mod parser;
mod snailfish;
mod tree;

use snailfish::SnailfishNumber;
use tree::Tree;

// ==== Main code ====

fn main() {
    let input = include_str!("../test.txt");
    let _first = input.lines().next().unwrap();

    // let mut snail = SnailfishNumber::from("[9,[8,[[1,2],[3,4]]]]");
    let mut snail = SnailfishNumber::from("[9,[8,[7,[1,2]]]]");
    // let snail = SnailfishNumber::from("[[[1,2],[3,4]],[[5,6],[7,8]]]");
    // let mut snail = SnailfishNumber::from("[[[[9,8],7],2],1]");
    // snail.explode_recursive(1, &mut None);
    let tree = Tree::from(snail.to_arr());
    // println!("{}", tree);
}
