use core::panic;
use std::sync::{Arc, Mutex};

use crate::{Store, Token, CLRF};

pub fn parse(tokens: &Vec<Token>, store: Arc<Mutex<Store>>) -> String {
    let mut token_iter = tokens.into_iter();

    match token_iter.next().expect("Could not find any token") {
        Token::ECHO => match token_iter.next().expect("Echo missing value to echo") {
            Token::String(value) => encode_bulk_string(value),
            _ => String::new(),
        },
        Token::SET => {
            let key = match token_iter.next().expect("Set expects a key") {
                Token::String(s) => s.to_string(),
                _ => panic!("Invalid key"),
            };

            let value = match token_iter.next().expect("Set expects a value") {
                Token::String(s) => s.to_string(),
                _ => panic!("Invalid value"),
            };

            let mut store = match store.lock() {
                Ok(s) => s,
                Err(e) => panic!("Got error trying to lock: {}", e),
            };

            store.set_kv(key, value);

            encode_simple_string("OK")
        }
        Token::GET => {
            let key = match token_iter.next().expect("Set expects a key") {
                Token::String(s) => s.to_string(),
                _ => panic!("Invalid key"),
            };

            let store = match store.lock() {
                Ok(s) => s,
                Err(e) => panic!("Got error trying to lock: {}", e),
            };

            match store.get_kv(key) {
                Some(v) => encode_bulk_string(&v),
                None => NULL_BULK.to_string(),
            }
        }
        Token::String(value) => match value.as_str() {
            "ping" => encode_simple_string("PONG"),
            _ => String::new(),
        },
    }
}

const NULL_BULK: &str = "$-1\r\n";

fn encode_simple_string(value: &str) -> String {
    format!("+{value}{CLRF}")
}

fn encode_bulk_string(value: &str) -> String {
    format!("${}{CLRF}{}{CLRF}", value.len(), value)
}
