mod token;
mod lexer;

fn main() {
    let token_type = token::TokenType::Plus;
    println!("{}", token_type.to_string());
}
