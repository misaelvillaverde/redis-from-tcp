#[derive(Debug)]
pub enum Token {
    ECHO,
    SET,
    GET,
    String(String),
}

impl Token {
    pub fn from_string(value: &str) -> Token {
        match value.to_lowercase().as_str() {
            "echo" => Token::ECHO,
            "set" => Token::SET,
            "get" => Token::GET,
            val => Token::String(val.to_string()),
        }
    }
}
