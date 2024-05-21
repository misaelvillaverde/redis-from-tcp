use crate::{Token, CLRF};

pub fn parse(tokens: &Vec<Token>) -> String {
    let mut token_iter = tokens.into_iter();

    match token_iter.next().expect("Could not find any token") {
        Token::Echo => match token_iter.next().expect("Echo missing value to echo") {
            Token::String(value) => encode_bulk_string(value),
            _ => String::new(),
        },
        Token::String(value) => match value.as_str() {
            "ping" => format!("+PONG{CLRF}"),
            _ => String::new(),
        },
    }
}

fn encode_bulk_string(value: &str) -> String {
    format!("${}{CLRF}{}{CLRF}", value.len(), value)
}
