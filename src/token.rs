#[derive(Debug)]
pub enum Token {
    ECHO,
    SET,
    GET,
    PX,
    Literal(String),
}

impl Token {
    pub fn from_string(value: &str) -> Token {
        match value.to_lowercase().as_str() {
            "echo" => Token::ECHO,
            "set" => Token::SET,
            "get" => Token::GET,
            "px" => Token::PX,
            val => Token::Literal(val.to_string()),
        }
    }
}
