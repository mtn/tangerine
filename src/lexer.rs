
#[derive(Debug, PartialEq)]
pub enum Token {
    Lambda,
    Dot,
    LParen,
    RParen,
    EOF,
    Atom(String),
}

pub struct Lexer {
    input: Vec<char>,
    ind: usize,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            input: input.chars().filter(|x| !x.is_whitespace()).collect(),
            ind : 0,
        }
    }

    fn advance(&mut self) {
        self.ind += 1;
    }

    fn get_token (&mut self) -> Option<Token> {
        if self.ind >= self.input.len() {
            return None
        }

        match self.input[self.ind] {
            'λ'| '\\' => {
                self.advance();
                Some(Token::Lambda)
            },
            '.' => {
                self.advance();
                Some(Token::Dot)
            },
            '(' => {
                self.advance();
                Some(Token::LParen)
            }
            ')' => {
                self.advance();
                Some(Token::RParen)
            }
            '\0' => {
                self.advance();
                Some(Token::EOF)
            }
            ch => {
                if !ch.is_alphabetic() && !ch.is_lowercase() {
                    panic!("Malformed identifier");
                }
                self.advance();

                Some(Token::Atom(ch.to_string()))
            }
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut v: Vec<Token> = Vec::new();

        while let Some(t) = self.get_token() {
            v.push(t);
        }
        v.push(Token::EOF);

        v
    }
}
