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
    println!("{}", evaluate(String::from(("(λx.(λx.y x) (λx.z x)) x"))));
}

#[cfg(test)]
mod tests {

    use super::*;
    use parser::ASTNode;

    #[test]
    fn test_evaluation() {
        // identity combinator
        // (λx.x) y -> y
        assert_eq!(evaluate(String::from("(λx.x) y")), ASTNode::Atom(String::from("y")));

        // application combinator
        // ((λy.λx.(y x)) (λx.x x)) y -> y y
        assert_eq!(evaluate(String::from("((λy.λx.(y x)) (λx.x x)) y")),
                   ASTNode::Application {
                       lhs: Box::new(ASTNode::Atom(String::from("y"))),
                       rhs: Box::new(ASTNode::Atom(String::from("y")))
                   });

        // simple eta-reducable expression
        // λx.(y x) -> y
        assert_eq!(evaluate(String::from("λx.(y x)")), ASTNode::Atom(String::from("y")));

        // complex eta-reducable expression
        // (λx.(λx.y x) (λx.z x)) x -> y z
        assert_eq!(evaluate(String::from("(λx.(λx.y x) (λx.z x)) x")),
                   ASTNode::Application {
                       lhs: Box::new(ASTNode::Atom(String::from("y"))),
                       rhs: Box::new(ASTNode::Atom(String::from("z")))
                   });

        // if-else combinator (5 beta-reductions)
        // (λp.λa.λb.p a b) (λa.λb. a) a b -> a
        assert_eq!(evaluate(String::from("(λp.λa.λb.p a b) (λa.λb. a) a b")),
                   ASTNode::Atom(String::from("a")));

        // alpha-beta-eta combination
        // (λz.z (λx. w x)) y -> y w
        assert_eq!(evaluate(String::from("(λz.z (λx. w x)) y")),
                   ASTNode::Application {
                       lhs: Box::new(ASTNode::Atom(String::from("y"))),
                       rhs: Box::new(ASTNode::Atom(String::from("w")))
                   });

        // possible name collision
        // (λx.(λx.y x) (λx.z x)) x -> w w
        assert_eq!(evaluate(String::from("(λx.λx.(x x)) y w")),
                   ASTNode::Application {
                       lhs: Box::new(ASTNode::Atom(String::from("w"))),
                       rhs: Box::new(ASTNode::Atom(String::from("w")))
                   });
    }
}
