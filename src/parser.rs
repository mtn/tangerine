use std::collections::HashMap;
use std::fmt;

use lexer::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum ASTNode {
    Abstraction { param: Box<ASTNode>, body: Box<ASTNode> },
    Application { lhs: Box<ASTNode>, rhs: Box<ASTNode> },
    Atom(String),
    Epsilon,
}

impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ASTNode::Abstraction { ref param, ref body } =>
                write!(f, "λ{}.{}", param, body),
            &ASTNode::Application { ref lhs, ref rhs } =>
                write!(f, "({} {})", lhs, rhs),
            &ASTNode::Atom(ref name) =>
                write!(f, "{}", name),
            &ASTNode::Epsilon => write!(f, "ε"),
        }
    }
}

impl ASTNode {
    pub fn reduce(&self, env: HashMap<String, ASTNode>) -> ASTNode {
        match self {
            &ASTNode::Abstraction { ref param, ref body } => {
                let mut new = env.clone();
                if let ASTNode::Atom(ref name) = **param {
                    if new.contains_key(name) {
                        new.remove(name);
                    }

                    if let ASTNode::Application { ref lhs, ref rhs } = **body {
                        if rhs == param && !lhs.free_in(&**param) {
                            return lhs.reduce(new)
                        }
                    }
                } else {
                    panic!("Incorrectly structured Abstraction");
                }

                ASTNode::Abstraction {
                    param: param.clone(),
                    body: Box::new(body.reduce(new))
                }
            },
            &ASTNode::Application { ref lhs, ref rhs } => {
                if let ASTNode::Abstraction { ref param, ref body } = **lhs {
                    let mut new = env.clone();
                    if let ASTNode::Atom(ref name) = **param {
                        new.insert(name.clone(), *rhs.clone());
                        return body.reduce(new)
                    }
                    panic!("Incorrectly structured Application");
                }

                ASTNode::Application {
                    lhs: Box::new(lhs.reduce(env.clone())),
                    rhs: Box::new(rhs.reduce(env.clone()))
                }
            },
            &ASTNode::Atom (ref name) => {
                match env.get(name) {
                    Some(ref node) =>
                        (*node).clone(),
                    None =>
                        self.clone(),
                }
            },
            &ASTNode::Epsilon => {
                (*self).clone()
            }
        }
    }

    fn free_in(&self, atom: &ASTNode) -> bool {
        match self {
            &ASTNode::Abstraction { ref param, ref body } =>
                *atom != **param && body.free_in(atom),
            &ASTNode::Application { ref lhs, ref rhs } =>
                lhs.free_in(atom) || rhs.free_in(atom),
            &ASTNode::Atom (_) =>
                self == atom,
            _ => false
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    ind: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, ind: 0 }
    }

    fn consume(&mut self, token: Token) -> &Token {
        if self.tokens[self.ind] != token {
            panic!("Unexpected token: Given {:?}, Expected {:?}",
                   self.tokens[self.ind], token);
        }

        self.ind += 1;
        &self.tokens[self.ind-1]
    }

    fn parse_abstraction(&mut self) -> ASTNode {
        self.consume(Token::Lambda);
        let param = self.parse_atom();
        self.consume(Token::Dot);

        ASTNode::Abstraction {
            param: Box::new(param),
            body: Box::new(self.parse_expression())
        }
    }

    fn parse_application(&mut self) -> ASTNode {
        let mut lexpr = self.parse_bounded().unwrap();
        loop {
            let rexpr = self.parse_bounded();
            match rexpr {
                Some(exp) => {
                    lexpr = ASTNode::Application {
                        lhs: Box::new(lexpr),
                        rhs: Box::new(exp)
                    }
                },
                None =>
                    return lexpr
            };
        }
    }

    fn parse_expression(&mut self) -> ASTNode {
        if self.tokens[self.ind] == Token::EOF
            || self.tokens[self.ind] == Token::RParen {
            return ASTNode::Epsilon
        }

        self.parse_application()
    }

    fn parse_parenthesized_expression(&mut self) -> ASTNode {
        self.consume(Token::LParen);
        let expr = self.parse_expression();
        self.consume(Token::RParen);

        expr
    }

    fn parse_bounded(&mut self) -> Option<ASTNode> {
        match self.tokens[self.ind] {
            Token::Atom(_) =>
                Some(self.parse_atom()),
            Token::Lambda =>
                Some(self.parse_abstraction()),
            Token::LParen =>
                Some(self.parse_parenthesized_expression()),
            _ => None,
        }
    }

    fn parse_atom(&mut self) -> ASTNode {
        if let Token::Atom(ref name) = self.tokens[self.ind] {
            self.ind += 1;
            return ASTNode::Atom(name.clone())
        }
        panic!("Unexpected token type: Given {:?}, Expected Atom",
               self.tokens[self.ind]);
    }

    pub fn parse(&mut self) -> ASTNode {
        let res = self.parse_expression();
        self.consume(Token::EOF);
        res
    }
}

pub fn remove_epsilon(mut ast: ASTNode) -> ASTNode {
    ast = if let ASTNode::Application { lhs: _, rhs: ref r } = ast {
        if **r == ASTNode::Epsilon {
            remove_epsilon(*r.clone())
        } else {
            ast.clone()
        }
    } else {
        ast
    };

    ast = match ast {
        ASTNode::Application { ref lhs, ref rhs } => {
            ASTNode::Application {
                lhs: Box::new(remove_epsilon(*lhs.clone())),
                rhs: Box::new(remove_epsilon(*rhs.clone()))
            }
        },
        _ => ast
    };

    ast = match ast {
        ASTNode::Abstraction { ref param, ref body } => {
            ASTNode::Abstraction {
                param: param.clone(),
                body: Box::new(remove_epsilon(*body.clone()))
            }
        },
        _ => ast
    };

    ast
}
