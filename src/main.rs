mod lexer;
mod parser;

use std::collections::HashMap;

fn evaluate(expr: String) -> parser::ASTNode {
    let tokens = lexer::Lexer::new(expr).lex();

    let mut ast = parser::Parser::new(tokens).parse();

    ast = parser::remove_epsilon(ast);

    loop {
        let new = ast.reduce(HashMap::new());

        if ast == new {
            break;
        }

        ast = new;
    }

    ast
}

fn main() {
    println!("Hello, world!");
    println!("{:?}", evaluate(String::from(("(λx.(λx.y x) (λx.z x)) x"))));
}
