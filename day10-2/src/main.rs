use std::io::{self, BufRead};

struct SyntaxError {
    expected: char,
    found: Option<char>,
}
fn parse_program(program: &str) -> (Option<String>, Option<SyntaxError>) {
    let mut brackets_stack: Vec<char> = Vec::new();
    for char in program.chars() {
        match char {
            '(' | '[' | '{' | '<' => brackets_stack.push(char),
            ')' => {
                let pop = brackets_stack.pop();
                match pop {
                    Some(c) => {
                        if c != '(' {
                            return (
                                None,
                                Some(SyntaxError {
                                    expected: '(',
                                    found: Some(c),
                                }),
                            );
                        }
                    }
                    None => {
                        return (
                            None,
                            Some(SyntaxError {
                                expected: '(',
                                found: None,
                            }),
                        );
                    }
                };
            }
            ']' => {
                let pop = brackets_stack.pop();
                match pop {
                    Some(c) => {
                        if c != '[' {
                            return (
                                None,
                                Some(SyntaxError {
                                    expected: '[',
                                    found: Some(c),
                                }),
                            );
                        }
                    }
                    None => {
                        return (
                            None,
                            Some(SyntaxError {
                                expected: '[',
                                found: None,
                            }),
                        );
                    }
                };
            }
            '}' => {
                let pop = brackets_stack.pop();
                match pop {
                    Some(c) => {
                        if c != '{' {
                            return (
                                None,
                                Some(SyntaxError {
                                    expected: '{',
                                    found: Some(c),
                                }),
                            );
                        }
                    }
                    None => {
                        return (
                            None,
                            Some(SyntaxError {
                                expected: '{',
                                found: None,
                            }),
                        );
                    }
                };
            }
            '>' => {
                let pop = brackets_stack.pop();
                match pop {
                    Some(c) => {
                        if c != '<' {
                            return (
                                None,
                                Some(SyntaxError {
                                    expected: '<',
                                    found: Some(c),
                                }),
                            );
                        }
                    }
                    None => {
                        return (
                            None,
                            Some(SyntaxError {
                                expected: '<',
                                found: None,
                            }),
                        );
                    }
                };
            }
            _ => (),
        };
    }

    let rest_program = brackets_stack.iter().collect::<String>();
    (Some(rest_program), None)
}

fn calc_auto_complete_score(rest_program: String) -> i128 {
    let mut score: i128 = 0;
    for char in rest_program.chars().rev() {
        score *= 5;
        score += match char {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => 0,
        };
    }
    score
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut scores: Vec<i128> = Vec::new();
    for _line in lines {
        let line = _line.unwrap();
        let (rest_program, _) = parse_program(&line);
        match rest_program {
            Some(program) => scores.push(calc_auto_complete_score(program)),
            None => {}
        };
    }

    scores.sort();
    println!("ans {}", scores[scores.len() / 2]);
}
