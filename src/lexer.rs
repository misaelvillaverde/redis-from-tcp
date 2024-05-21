use std::str::Chars;

use crate::Token;

pub fn tokenize(data: String) -> Vec<Token> {
    let mut tokens = vec![];

    let mut reader = data.chars();

    let first_char = match reader.next() {
        Some(ch) => ch,
        None => return tokens,
    };

    let mut iterations = 1;

    if first_char == '*' {
        iterations = read_number(&mut reader);
    }

    for _ in 0..iterations {
        match reader.next() {
            Some(c) => match c {
                '$' => {
                    tokens.push(read_bulk_string(&mut reader));
                }
                ':' => {
                    todo!("integers");
                }
                '+' => {
                    tokens.push(read_string(&mut reader));
                }
                '-' => {
                    todo!("simple errors");
                }
                _ => break,
            },
            None => break,
        }
    }

    tokens
}

/// `+OK\r\n`
/// Consumes CLRF after string
fn read_string(reader: &mut Chars) -> Token {
    let mut data = String::new();

    for ch in &mut *reader {
        match ch {
            '\r' => {
                reader.next();
                break;
            }
            _ => data.push(ch),
        }
    }

    Token::from_string(&data)
}

/// `$<length>\r\n<data>\r\n`
fn read_bulk_string(reader: &mut Chars) -> Token {
    let length = read_number(reader);

    let mut data = String::with_capacity(length);
    for _ in 0..length {
        match reader.next() {
            Some(ch) => data.push(ch),
            None => break,
        }
    }

    consume_clrf(reader);

    Token::from_string(&data)
}

/// Consumes CLRF after number
fn read_number(reader: &mut Chars) -> usize {
    let mut num = String::from("");

    for cur in &mut *reader {
        match cur {
            '\r' => {
                reader.next(); // consume \n to complete CLRF
                break;
            }
            ch => {
                num.push(ch);
            }
        }
    }

    usize::from_str_radix(&num, 10).expect("Expected number")
}

fn consume_clrf(reader: &mut Chars) {
    reader.next();
    reader.next();
}
