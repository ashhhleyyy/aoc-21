use std::io::BufRead;

fn main() {
    let mut syntax_score = 0;
    let mut autocomplete_scores = Vec::new();
    for line in std::io::stdin().lock().lines().map(Result::unwrap) {
        if let Some(chr) = check_if_line_corrupt(&line) {
            syntax_score += score_corrupt(chr);
        } else {
            let score = correct_incomplete_line(&line);
            autocomplete_scores.push(score);
        }
    }
    autocomplete_scores.sort();
    let middle = autocomplete_scores.len() / 2;
    println!("Syntax score: {}", syntax_score);
    println!("Autocomplete score: {}", autocomplete_scores.get(middle).unwrap());
}

fn score_corrupt(c: char) -> i64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

peg::parser! {
    grammar navigation_parser () for str {
        rule curved_block() -> ()
            = "(" block()* ")"

        rule square_block() -> ()
            = "[" block()* "]"

        rule curley_block() -> ()
            = "{" block()* "}"

        rule pointy_block() -> ()
            = "<" block()* ">"

        rule block() -> ()
            = curved_block() / square_block() / curley_block() / pointy_block() { () }

        pub rule navigation() -> ()
            = block() * { () }
    }
}

fn check_if_line_corrupt(line: &str) -> Option<char> {
    match navigation_parser::navigation(line) {
        Ok(_) => None,
        Err(e) => {
            line.chars().skip(e.location.offset).next()
        },
    }
}

fn correct_incomplete_line(line: &str) -> i64 {
    let mut line = String::from(line);
    let mut score = 0;
    while let Some(char) = try_fix_incomplete(&line) {
        line.push(char);
        let char_score = match char {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => unreachable!(),
        };
        score *= 5;
        score += char_score;
    }
    score
}

fn try_fix_incomplete(line: &str) -> Option<char> {
    match navigation_parser::navigation(line) {
        Ok(_) => None,
        Err(e) => {
            for token in e.expected.tokens() {
                let c= token.chars().skip(1).next().unwrap();
                if let None = get_end_char(c) {
                    return Some(c);
                }
            }
            None
        },
    }
}

fn get_end_char(c: char) -> Option<char> {
    Some(match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => return None,
    })
}

#[test]
fn test_corrupt() {
    assert!(check_if_line_corrupt("{([(<{}[<>[]}>{[]{[(<()>").is_some());
    assert!(check_if_line_corrupt("[[<[([]))<([[{}[[()]]]").is_some());
    assert!(check_if_line_corrupt("[{[{({}]{}}([{[{{{}}([]").is_some());
    assert!(check_if_line_corrupt("[<(<(<(<{}))><([]([]()").is_some());
    assert!(check_if_line_corrupt("<{([([[(<>()){}]>(<<{{").is_some());
}

#[test]
fn test_not_corrupt() {
    assert!(check_if_line_corrupt("[({(<(())[]>[[{[]{<()<>>").is_none());
    assert!(check_if_line_corrupt("[(()[<>])]({[<{<<[]>>(").is_none());
    assert!(check_if_line_corrupt("(((({<>}<{<{<>}{[]{[]{}").is_none());
    assert!(check_if_line_corrupt("{<[[]]>}<{[{[{[]{()[[[]").is_none());
    assert!(check_if_line_corrupt("<{([{{}}[<[[[<>{}]]]>[]]").is_none());
}
