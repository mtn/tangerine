use std::collections::HashMap;
use std::fmt;

use lexer::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum ASTNode<'a> {
    Abstraction { param: Box<ASTNode<'a>>, body: Box<ASTNode<'a>> },
    Application { lhs: Box<ASTNode<'a>>, rhs: Box<ASTNode<'a>> },
    Atom(&'a str),
    Epsilon,
}

impl<'a> fmt::Display for ASTNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ASTNode::Abstraction { param: ref p, body: ref b } => {
                write!(f, "λ{}.{}", p, b)
            },
            &ASTNode::Application { lhs: ref l, rhs: ref r } => {
                write!(f, "({} {})", l, r)
            },
            &ASTNode::Atom(ref name) => write!(f, "{}", name),
            &ASTNode::Epsilon => write!(f, "ε"),
        }
    }
}

impl<'a> ASTNode <'a> {
    pub fn reduce(&'a self, env: HashMap<String, &'a Box<ASTNode>>) -> ASTNode {
        match self {
            &ASTNode::Abstraction { param: ref p, body: ref b } => {
                let mut new = env.clone();
                if let ASTNode::Atom(name) = **p {
                    if new.contains_key(name) {
                        new.remove(name);
                    }

                    if let ASTNode::Application { lhs: ref l, rhs: ref r } = **b {
                        if r == p && !l.free_in(&*p) {
                            return l.reduce(new)
                        }
                    }
                } else {
                    panic!("Incorrectly structured Abstraction");
                }

                ASTNode::Abstraction { param: p.clone(), body: Box::new(b.reduce(new)) }
            },
            &ASTNode::Application { lhs: ref l, rhs: ref r } => {
                if let ASTNode::Abstraction { param: ref p, body: ref b} = **l {
                    let mut new = env.clone();
                    if let ASTNode::Atom(name) = **p {
                        new.insert(name.to_string(), &*r);
                        return b.reduce(new)
                    }
                    panic!("Incorrectly structured Application");
                }

                ASTNode::Application {
                    lhs: Box::new(l.reduce(env.clone())),
                    rhs: Box::new(r.reduce(env.clone()))
                }
            },
            &ASTNode::Atom (name) => {
                match env.get(name) {
                    Some(node) => **node.clone(),
                    None => self.clone(),
                }
            },
            &ASTNode::Epsilon => {
                (*self).clone()
            }
        }
    }

    fn free_in (&'a self, atom: &ASTNode) -> bool {
        match self {
            &ASTNode::Abstraction { param: ref p, body: ref b } => {
                false
                // *atom != *p && b.free_in(atom)
            },
            &ASTNode::Application { lhs: ref l, rhs: ref r } => {
                false
                // l.free_in(atom) || r.free_in(atom)
            },
            &ASTNode::Atom (_) => {
                false
                // self == atom
            },
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
                None => return lexpr
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
            Token::Atom(_) => Some(self.parse_atom()),
            Token::Lambda  => Some(self.parse_abstraction()),
            Token::LParen  => Some(self.parse_parenthesized_expression()),
            _              => None,
        }
    }

    fn parse_atom(&mut self) -> ASTNode {
        if let Token::Atom(ref name) = self.tokens[self.ind] {
            self.ind += 1;
            return ASTNode::Atom(name)
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
        ASTNode::Application {lhs: ref l, rhs: ref r} => {
            ASTNode::Application {
                lhs: Box::new(remove_epsilon(*l.clone())),
                rhs: Box::new(remove_epsilon(*r.clone()))
            }
        },
        _ => ast
    };

    ast = match ast {
        ASTNode::Abstraction { param: ref p, body: ref b } => {
            ASTNode::Abstraction { param: p.clone(), body: Box::new(remove_epsilon(*b.clone())) }
        },
        _ => ast
    };

    ast
}
