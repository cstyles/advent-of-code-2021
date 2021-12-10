#[derive(Debug)]
enum Validity {
    Valid,
    Incomplete,
    Corrupt(char),
}

fn main() {
    let input = include_str!("../input.txt");
    // let input = include_str!("../test.txt");
    let mut syntax_error_score = 0;

    for line in input.lines() {
        match check_validity(line) {
            Validity::Valid => continue,
            Validity::Incomplete => continue,
            Validity::Corrupt(illegal_char) => {
                let score = match illegal_char {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => unreachable!(),
                };

                syntax_error_score += score;
            }
        }
    }

    println!("part1 = {}", syntax_error_score);
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
        Validity::Incomplete
    }
}
