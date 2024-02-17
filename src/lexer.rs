use crate::token::{Token, TokenType};

///Works only with ASCII strings
pub struct Lexer {
    input: Vec<char>,
    position: usize,      //points to current char
    read_position: usize, //points to next char after current
    current_char: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Self {
            input: input.escape_default().collect::<Vec<_>>(),
            position: 0,
            read_position: 0,
            current_char: 0 as char,
        };
        lexer.read_char();
        lexer
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.current_char = 0 as char;
        } else {
            self.current_char = self.input[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }
// Self::Assign => "=".to_owned(),
// Self::Semicolon => ";".to_owned(),
// Self::Plus => "+".to_owned(),
// Self::Comma => ",".to_owned(),
// Self::LParen => "(".to_owned(),
// Self::RParen => ")".to_owned(),
// Self::LBrace => "{".to_owned(),
// Self::RBrace => "}".to_owned(),
    fn NextToken(&mut self) -> Token {
        let token = match self.current_char {
            '=' => Token::new(TokenType::Assign, self.current_char.to_string()),
            '}' => Token::new(TokenType::RBrace , self.current_char.to_string()),
            '{' => Token::new(TokenType::LBrace , self.current_char.to_string()),
            ')' => Token::new(TokenType::RParen , self.current_char.to_string()),
            '(' => Token::new(TokenType::LParen , self.current_char.to_string()),
            ',' => Token::new(TokenType::Comma , self.current_char.to_string()),
            '+' => Token::new(TokenType::Plus , self.current_char.to_string()),
            ';' => Token::new(TokenType::Semicolon , self.current_char.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Lexer;
    use crate::token::TokenType;
    
    #[records::record]
    struct Test {
        expected_type: TokenType,
        expected_literal: String,
    }

    #[test]
    fn test_next_token() {
        let tests = vec![Test::new(TokenType::Assign, "=".to_owned())];
        let input = String::from("=+(){},;");

        let mut lexer = Lexer::new(input);

        for test in tests {
            let token = lexer.NextToken();

            if token.token_type != test.expected_type {
                panic!(
                    "Wrong tokentype. Expected {}, got {}",
                    test.expected_type.to_string(),
                    token.token_type.to_string()
                );
            }

            if token.literal != test.expected_literal {
                panic!(
                    "Wrong literal. Expected {}, got {}",
                    test.expected_literal, token.literal
                );
            }
        }
    }
}
