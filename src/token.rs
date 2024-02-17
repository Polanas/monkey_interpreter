#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Illegal,
    EOF,
    Ident,
    Int,
    Assign,
    Plus,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Function,
    Let,
}

impl TokenType {
    pub fn to_string(&self) -> String {
        match self {
            Self::Illegal | Self::EOF | Self::Ident | Self::Int | Self::Function | Self::Let => {
                format!("{:?}", self)
            }
            Self::Assign => "=".to_owned(),
            Self::Semicolon => ";".to_owned(),
            Self::Plus => "+".to_owned(),
            Self::Comma => ",".to_owned(),
            Self::LParen => "(".to_owned(),
            Self::RParen => ")".to_owned(),
            Self::LBrace => "{".to_owned(),
            Self::RBrace => "}".to_owned(),
        }
    }
}

#[records::record]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}
