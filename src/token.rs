#[derive(Debug)]
pub enum Token {
    Echo,
    String(String),
}

impl Token {
    pub fn from_string(value: &str) -> Token {
        match value.to_lowercase().as_str() {
            "echo" => Token::Echo,
            val => Token::String(val.to_string()),
        }
    }
}
