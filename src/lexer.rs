use crate::char_util_trait::*;
use crate::token::Token;

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
    ///return None if end of file is reached
    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let token = match self.current_char {
            '=' => Some(Token::Assign),
            '{' => Some(Token::LBrace),
            '}' => Some(Token::RBrace),
            '(' => Some(Token::LParen),
            ')' => Some(Token::RParen),
            ',' => Some(Token::Comma),
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '!' => Some(Token::Bang),
            '*' => Some(Token::Asterisk),
            '/' => Some(Token::Slash),
            '<' => Some(Token::LessThan),
            '>' => Some(Token::GreaterThan),
            ';' => Some(Token::Semicolon),
            '\0' => None,
            _ if self.current_char.is_letter() => {
                let ident = self.read_ident();
                return match ident.as_str() {
                    "fn" => Some(Token::Function),
                    "let" => Some(Token::Let),
                    _ => Some(Token::Ident(ident)),
                };
            }
            _ if self.current_char.is_numeric() => {
                let number: i32 = self.read_number().parse().unwrap();
                return Some(Token::Int(number));
            }
            _ => Some(Token::Illegal(self.current_char.to_string()))
        };

        self.read_char();
        token
    }

    fn is_current_char_whitespace(&self) -> bool {
        let ch = self.current_char;
        let prev_ch = {
            if self.position == 0 {
                '\0'
            } else {
                self.input[self.position - 1]
            }
        };
        ch == ' '
            || ch == '\t'
            || ch == '\r'
            || ch == '\n'
            || ch == '\\'
            || (prev_ch == '\\' && ch == 'n')
    }

    fn skip_whitespace(&mut self) {
        while self.is_current_char_whitespace() {
            self.read_char();
        }
    }

    fn read_ident(&mut self) -> String {
        let position = self.position;

        while self.current_char.is_letter() {
            self.read_char();
        }

        (&self.input[position..self.position]).to_string()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;

        while self.current_char.is_numeric() {
            self.read_char();
        }

        (&self.input[position..self.position]).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::Lexer;
    use crate::token::Token;

    #[test]
    fn test_next_token() {
        let input = String::from(
            "let five = 5;
            let ten = 10;

            let add = fn(x, y) {
                x + y;
            };
            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;",
        );
        let expected_tokens = vec![
            Token::Let,
            Token::Ident("five".to_owned()),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".to_owned()),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,
            //...
        ];

        let mut lexer = Lexer::new(input);
        while let Some(token) = lexer.next_token() {
            if let Token::Illegal(value) = token {
                panic!("Illegal token found: {}", value);
            }
        }

        // if token != test.expected_type {
        //     panic!(
        //         "Wrong tokentype. Expected {}, got {}",
        //         test.expected_type.to_string(),
        //         token.to_string()
        //     );
        // }
        //
        // if token.to_string() != test.expected_literal {
        //     panic!(
        //         "Wrong literal. Expected {}, got {}",
        //         test.expected_literal,
        //         token.to_string()
        //     );
        // }
    }
}
