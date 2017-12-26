
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

                let mut name = ch.to_string();
                let input_len = self.input.len();
                while self.ind < input_len {
                    let ch = self.input[self.ind];
                    if !ch.is_alphabetic() {
                        break;
                    }

                    name.push(ch);
                    self.advance();
                }

                Some(Token::Atom(name))
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_test() {
        assert_eq!(1+1, 2);
    }

    #[test]
    fn test_equality() {
        assert_eq!(Token::Lambda, Token::Lambda);
    }
}
