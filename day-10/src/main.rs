#[derive(Debug)]
enum Validity {
    Valid,
    Incomplete(Vec<char>),
    Corrupt(char),
}

fn main() {
    let input = include_str!("../input.txt");
    // let input = include_str!("../test.txt");
    let mut syntax_error_score = 0;
    let mut autocomplete_scores: Vec<u64> = vec![];

    for line in input.lines() {
        match check_validity(line) {
            Validity::Valid => unreachable!("no lines are valid"),
            Validity::Incomplete(stack) => {
                let score = stack.into_iter().rev().fold(0, |mut acc, elm| {
                    acc *= 5;
                    acc += match elm {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => unreachable!("invalid opener: {}", elm),
                    };
                    acc
                });

                autocomplete_scores.push(score);
            }
            Validity::Corrupt(illegal_char) => {
                syntax_error_score += match illegal_char {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => unreachable!(),
                };
            }
        }
    }

    println!("part1 = {}", syntax_error_score);

    autocomplete_scores.sort_unstable();
    let middle = autocomplete_scores[autocomplete_scores.len() / 2];
    println!("part2 = {:?}", middle);
}

fn check_validity(line: &str) -> Validity {
    let chars_orig: Vec<char> = line.chars().collect();
    let mut chars = &chars_orig[..];
    let mut stack: Vec<char> = vec![];

    while !chars.is_empty() {
        let head = chars[0];
        chars = &chars[1..];

        match head {
            '(' | '[' | '{' | '<' => {
                stack.push(head);
                continue;
            }
            ')' | ']' | '}' | '>' => {
                match (stack.pop(), head) {
                    (None, _) => return Validity::Corrupt(head),
                    (Some('('), ')') => continue,
                    (Some('['), ']') => continue,
                    (Some('{'), '}') => continue,
                    (Some('<'), '>') => continue,
                    _ => return Validity::Corrupt(head),
                };
            }
            _ => return Validity::Corrupt(head), // unreachable
        };
    }

    if stack.is_empty() {
        Validity::Valid
    } else {
        Validity::Incomplete(stack)
    }
}
