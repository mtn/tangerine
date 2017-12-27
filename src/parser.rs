use lexer::Token;

#[derive(Debug, PartialEq)]
pub enum ASTNode {
    Abstraction { param: Box<ASTNode>, body: Box<ASTNode> },
    Application { lhs: Box<ASTNode>, rhs: Box<ASTNode> },
    Atom(String),
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
