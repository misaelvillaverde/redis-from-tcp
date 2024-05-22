use core::panic;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use crate::{KVData, Store, Token, CLRF};

pub fn parse(tokens: &Vec<Token>, store: Arc<Store>) -> String {
    let mut token_iter = tokens.into_iter();

    match token_iter.next().expect("Could not find any token") {
        Token::ECHO => match token_iter.next().expect("Echo missing value to echo") {
            Token::Literal(value) => encode_bulk_string(value),
            _ => String::new(),
        },
        Token::SET => {
            let key = match token_iter.next().expect("Set expects a key") {
                Token::Literal(s) => s.to_string(),
                _ => panic!("Invalid key"),
            };

            let value = match token_iter.next().expect("Set expects a value") {
                Token::Literal(s) => s.to_string(),
                _ => panic!("Invalid value"),
            };

            let expire_at = token_iter.next().and_then(|token| match token {
                Token::PX => match token_iter.next().expect("Expiry expects a value") {
                    Token::Literal(value) => u64::from_str_radix(value, 10)
                        .ok()
                        .and_then(|millis| Some(Instant::now() + Duration::from_millis(millis))),
                    _ => None,
                },
                _ => None,
            });

            store.set_kv(key, KVData::new(value, expire_at));

            encode_simple_string("OK")
        }
        Token::GET => {
            let key = match token_iter.next().expect("Set expects a key") {
                Token::Literal(s) => s.to_string(),
                _ => panic!("Invalid key"),
            };

            match store.get_kv(key) {
                Some(v) => encode_bulk_string(&v),
                None => NULL_BULK.to_string(),
            }
        }
        Token::Literal(value) => match value.as_str() {
            "ping" => encode_simple_string("PONG"),
            _ => String::new(),
        },
        _ => panic!("Invalid keyword"),
    }
}

const NULL_BULK: &str = "$-1\r\n";

fn encode_simple_string(value: &str) -> String {
    format!("+{value}{CLRF}")
}

fn encode_bulk_string(value: &str) -> String {
    format!("${}{CLRF}{}{CLRF}", value.len(), value)
}
