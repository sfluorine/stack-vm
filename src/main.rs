use std::fs;
use std::ops::Index;
use std::process::exit;
use core::panic;

enum Token {
    PUSH(String),
    DROP,
    OVER,
    DUP,
    PLUS,
    MINUS,
    MUL,
    DIV,
    PRINT
}

fn is_number(str: &String) -> bool {
    for c in str.chars(){
        if !c.is_numeric() {
            return false;
        }
    }

    return true;
}

fn lex(file_path: &String) -> Vec<Token> {
    let mut buffer: Vec<Token> = Vec::new();
    let mut source_code: Vec<String> = Vec::new();
    let text = fs::read_to_string(file_path);

    match text {
        Ok(s) => { 
            let raw_source_code: Vec<&str> = s.split_whitespace().collect();

            for src in raw_source_code {
                source_code.push(String::from(src));
            }
        }, 

        Err(i) => { 
            println!("ERROR: '{}'!", i);
            exit(1);
        }
    }

    for src in source_code {
        if is_number(&src) {
            buffer.push(Token::PUSH(src));
        } else if src == "DROP" {
            buffer.push(Token::DROP);
        } else if src == "OVER" {
            buffer.push(Token::OVER);
        } else if src == "DUP" {
            buffer.push(Token::DUP);
        } else if src == "+" {
            buffer.push(Token::PLUS);
        } else if src == "-" {
            buffer.push(Token::MINUS);
        } else if src == "*" {
            buffer.push(Token::MUL);
        } else if src == "/" {
            buffer.push(Token::DIV);
        } else if src == "PRINT" {
            buffer.push(Token::PRINT);
        } else {
            println!("ERROR: unresolved '{}'", src);
            exit(1);
        }
    }

    buffer
}

fn simulate(tok: &Vec<Token>) {
    let mut stack: Vec<i64> = Vec::new();

    for item in tok {
        match &item {
            Token::PUSH(_s) => {
                stack.push(_s.parse::<i64>().unwrap());
            },

            Token::DROP => {
                if stack.len() < 1 {
                    panic!("ERROR: trying to pop empty stack!");
                } else {
                    stack.pop();
                }
            },

            Token::OVER => {
                if stack.len() < 2 {
                    panic!("ERROR: stack is less than 2!");
                } else {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();

                    stack.push(a);
                    stack.push(b);
                }
            },

            Token::DUP => {
                if stack.len() < 1 {
                    panic!("ERROR: trying access empty stack!");
                } else {
                    let buffer = stack.pop().unwrap();
                    
                    stack.push(buffer);
                    stack.push(buffer);
                }
            },

            Token::PLUS => {
                if stack.len() < 2 {
                    panic!("ERROR: stack is less than 2!");
                } else {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();

                    stack.push(a + b);
                }
            },

            Token::MINUS => {
                if stack.len() < 2 {
                    panic!("ERROR: stack is less than 2!");
                } else {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();

                    stack.push(b - a);
                }
            },

            Token::MUL => {
                if stack.len() < 2 {
                    panic!("ERROR: stack is less than 2!");
                } else {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();

                    stack.push(b * a);
                }
            },

            Token::DIV => {
                if stack.len() < 2 {
                    panic!("ERROR: stack is less than 2!");
                } else {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();

                    stack.push(b / a);
                }
            },

            Token::PRINT => {
                if stack.len() < 1 {
                    panic!("ERROR: trying access empty stack!");
                } else {
                    println!("{}", stack.index(stack.len() - 1));
                }
            }
        }
    }
}


fn main() {
    let tokens = lex(&String::from("test.vm"));
    simulate(&tokens);
}