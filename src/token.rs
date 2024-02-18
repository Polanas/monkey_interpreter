#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Illegal(String),
    Ident(String),
    Int(i32),
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    LessThan,
    GreaterThan,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Function,
    Let,
}

impl Token {
    // pub fn to_string(&self) -> String {
    //     match self {
    //         Self::Function | Self::Let => {
    //             format!("{:?}", self).to_lowercase()
    //         }
    //         Self::Assign => "=".to_owned(),
    //         Self::Semicolon => ";".to_owned(),
    //         Self::Plus => "+".to_owned(),
    //         Self::Comma => ",".to_owned(),
    //         Self::LParen => "(".to_owned(),
    //         Self::RParen => ")".to_owned(),
    //         Self::LBrace => "{".to_owned(),
    //         Self::RBrace => "}".to_owned(),
    //         Self::Int(value) => value.to_string(),
    //         Self::Ident(value) => value.to_owned(),
    //         Self::Illegal(value) => value.to_owned(),
    //     }
    // }
}
