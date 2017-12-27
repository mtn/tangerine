mod lexer;
mod parser;

fn main() {
    let mut lexer = lexer::Lexer::new(String::from("(λx.λx.(x x)) y w"));
    // println!("{:?}", lexer.lex());
    let mut parser = parser::Parser::new(lexer.lex()).parse();
    println!("Hello, world!");
}
