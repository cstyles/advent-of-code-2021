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
                let score = stack.into_iter().rev().fold(0, |acc, elm| {
                    acc * 5
                        + match elm {
                            '(' => 1,
                            '[' => 2,
                            '{' => 3,
                            '<' => 4,
                            _ => unreachable!("invalid opener: {}", elm),
                        }
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
    let chars: Vec<char> = line.chars().collect();
    let mut chars = &chars[..];
    let mut stack: Vec<char> = vec![];

    // While there are still chars to check
    while let Some(&head) = chars.first() {
        chars = &chars[1..];

        match head {
            '(' | '[' | '{' | '<' => stack.push(head),
            ')' | ']' | '}' | '>' => match (stack.pop(), head) {
                // No opening character
                (None, _) => return Validity::Corrupt(head),
                (Some('('), ')') => continue,
                (Some('['), ']') => continue,
                (Some('{'), '}') => continue,
                (Some('<'), '>') => continue,
                // Closing character doesn't match
                _ => return Validity::Corrupt(head),
            },
            _ => unreachable!("bad character: {}", head),
        };
    }

    if stack.is_empty() {
        Validity::Valid
    } else {
        Validity::Incomplete(stack)
    }
}
