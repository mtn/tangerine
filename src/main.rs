use std::collections::HashMap;
use std::env;

use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

extern crate rustyline;
use rustyline::error::ReadlineError;
use rustyline::Editor;

mod lexer;
mod parser;

fn evaluate(expr: &str) -> parser::ASTNode {
    let tokens = lexer::Lexer::new(&expr).lex();

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

fn run_batch_mode(filename: &str) {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    for line in f.lines() {
        println!("{}", evaluate(&line.unwrap()));
    }
}

fn run_repl() {
    let mut rl = Editor::<()>::new();
    loop {
        let line = rl.readline(">> ");
        match line {
            Ok(inp) => println!("{}", evaluate(&inp)),
            Err(ReadlineError::Interrupted) => {
                println!("Interrupted, goodbye!");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("Goodbye!");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => run_repl(),
        2 => run_batch_mode(&args[1]),
        _ => println!("Usage: cargo run [filename]"),
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use parser::ASTNode;

    #[test]
    fn test_evaluation() {
        // identity combinator
        // (λx.x) y -> y
        assert_eq!(evaluate("(λx.x) y"), ASTNode::Atom(String::from("y")));

        // application combinator
        // ((λy.λx.(y x)) (λx.x x)) y -> y y
        assert_eq!(
            evaluate("((λy.λx.(y x)) (λx.x x)) y"),
            ASTNode::Application {
                lhs: Box::new(ASTNode::Atom(String::from("y"))),
                rhs: Box::new(ASTNode::Atom(String::from("y"))),
            }
        );

        // simple eta-reducable expression
        // λx.(y x) -> y
        assert_eq!(evaluate("λx.(y x)"), ASTNode::Atom(String::from("y")));

        // complex eta-reducable expression
        // (λx.(λx.y x) (λx.z x)) x -> y z
        assert_eq!(
            evaluate("(λx.(λx.y x) (λx.z x)) x"),
            ASTNode::Application {
                lhs: Box::new(ASTNode::Atom(String::from("y"))),
                rhs: Box::new(ASTNode::Atom(String::from("z"))),
            }
        );

        // if-else combinator (5 beta-reductions)
        // (λp.λa.λb.p a b) (λa.λb. a) a b -> a
        assert_eq!(
            evaluate("(λp.λa.λb.p a b) (λa.λb. a) a b"),
            ASTNode::Atom(String::from("a"))
        );

        // alpha-beta-eta combination
        // (λz.z (λx. w x)) y -> y w
        assert_eq!(
            evaluate("(λz.z (λx. w x)) y"),
            ASTNode::Application {
                lhs: Box::new(ASTNode::Atom(String::from("y"))),
                rhs: Box::new(ASTNode::Atom(String::from("w"))),
            }
        );

        // possible name collision
        // (λx.(λx.y x) (λx.z x)) x -> w w
        assert_eq!(
            evaluate("(λx.λx.(x x)) y w"),
            ASTNode::Application {
                lhs: Box::new(ASTNode::Atom(String::from("w"))),
                rhs: Box::new(ASTNode::Atom(String::from("w"))),
            }
        );
    }
}
