use std::io::{self, BufRead};

struct SyntaxError {
    expected: char,
    found: Option<char>,
}
fn parse_program(program: &str) -> Option<SyntaxError> {
    let mut brackets_stack: Vec<char> = Vec::new();
    for char in program.chars() {
        match char {
            '(' | '[' | '{' | '<' => brackets_stack.push(char),
            ')' => {
                let pop = brackets_stack.pop();
                match pop {
                    Some(c) => {
                        if c != '(' {
                            return Some(SyntaxError {
                                expected: '(',
                                found: Some(c),
                            });
                        }
                    }
                    None => {
                        return Some(SyntaxError {
                            expected: '(',
                            found: None,
                        });
                    }
                };
            }
            ']' => {
                let pop = brackets_stack.pop();
                match pop {
                    Some(c) => {
                        if c != '[' {
                            return Some(SyntaxError {
                                expected: '[',
                                found: Some(c),
                            });
                        }
                    }
                    None => {
                        return Some(SyntaxError {
                            expected: '[',
                            found: None,
                        });
                    }
                };
            }
            '}' => {
                let pop = brackets_stack.pop();
                match pop {
                    Some(c) => {
                        if c != '{' {
                            return Some(SyntaxError {
                                expected: '{',
                                found: Some(c),
                            });
                        }
                    }
                    None => {
                        return Some(SyntaxError {
                            expected: '{',
                            found: None,
                        });
                    }
                };
            }
            '>' => {
                let pop = brackets_stack.pop();
                match pop {
                    Some(c) => {
                        if c != '<' {
                            return Some(SyntaxError {
                                expected: '<',
                                found: Some(c),
                            });
                        }
                    }
                    None => {
                        return Some(SyntaxError {
                            expected: '<',
                            found: None,
                        });
                    }
                };
            }
            _ => (),
        };
    }

    None
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut ans = 0;
    for _line in lines {
        let line = _line.unwrap();
        let error = parse_program(&line);
        match error {
            Some(e) => {
                ans += match e.expected {
                    '(' => 3,
                    '[' => 57,
                    '{' => 1197,
                    '<' => 25137,
                    _ => 0,
                };
            }
            None => {}
        };
    }

    println!("ans {}", ans);
}
