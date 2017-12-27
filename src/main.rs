mod lexer;
mod parser;

use std::collections::HashMap;

fn main() {
    // let mut lexer = lexer::Lexer::new(String::from("(λx.λx.(x x)) y w"));
    // println!("{:?}", lexer.lex());
    // let mut parser = parser::Parser::new(lexer.lex()).parse();
    let mut node = parser::ASTNode::Abstraction { param: Box::new(parser::ASTNode::Atom(String::from("x"))), body: Box::new(parser::ASTNode::Atom(String::from("x"))) };
    node.reduce(HashMap::new());
    println!("Hello, world!");
}
