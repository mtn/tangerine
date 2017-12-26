use lexer::Token;

#[derive(Debug, PartialEq)]
enum ASTNode {
    Abstraction { param: Box<ASTNode>, body: Box<ASTNode> },
    Application { lhs: Box<ASTNode>, rhs: Box<ASTNode> },
    Atom { name: String },
    Epsilon,
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

    fn parseAbstraction(&mut self) -> ASTNode {
        unimplemented!();
    }

    fn parseApplication(&mut self) -> ASTNode {
        unimplemented!();
    }

    fn parseExpression(&mut self) -> ASTNode {
        unimplemented!();
    }

    fn parseAtom(&mut self) -> ASTNode {
        unimplemented!();
    }

    pub fn parse(&mut self) -> ASTNode {
        unimplemented!();
    }
}
