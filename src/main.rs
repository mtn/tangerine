mod lexer;
mod parser;

fn main() {
    let mut lexer = lexer::Lexer::new(String::from("((λy.λx.(y x)) (λx.x x)) y"));
    let tokens = lexer.lex();
    for tok in tokens {
        println!("{:?}",tok);
    }
    println!("Hello, world!");
}
