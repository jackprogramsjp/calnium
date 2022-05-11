use crate::tokens::Token;
use std::str::Chars;

pub struct Lexer<'a> {
    text: Chars<'a>,
    current: Option<char>,
}

type TokenResult = Result<Token, String>;

impl<'a> Lexer<'a> {
    pub fn new(text: &'a str) -> Self {
        let mut res = Self {
            text: text.chars(),
            current: None,
        };
        res.advance();
        res
    }

    fn advance(&mut self) {
        self.current = self.text.next();
    }

    fn get_number(&mut self) -> TokenResult {
        let mut decimal_point_count = 0;
        let mut number = match self.current {
            Some(c) => c.to_string(),
            None => {
                panic!("Bug: the current character must be a number ('.' or a numerical digit)")
            }
        };
        self.advance();

        while let Some(current) = self.current {
            if current == '.' {
                decimal_point_count += 1;
                if decimal_point_count > 1 {
                    break;
                }
            } else if !is_number(current) {
                break;
            }

            number.push(current);
            self.advance();
        }

        if number.starts_with('.') {
            number = format!("0{}", number);
        }
        if number.ends_with('.') {
            number.push('0');
        }

        let result: f64 = match number.parse() {
            Ok(x) => x,
            Err(e) => return Err(e.to_string()),
        };

        Ok(Token::Number(result))
    }
}

#[inline]
fn is_number(c: char) -> bool {
    c.is_digit(10) || c == '.'
}

macro_rules! token_result {
    ($self:ident, $token:expr) => {{
        $self.advance();
        Some(Ok($token))
    }};
}

impl Iterator for Lexer<'_> {
    type Item = TokenResult;

    fn next(&mut self) -> Option<Self::Item> {
        use Token::*;
        match self.current {
            Some(' ') | Some('\t') | Some('\n') => {
                self.advance();
                self.next()
            }
            Some('+') => token_result!(self, Plus),
            Some('-') => token_result!(self, Minus),
            Some('*') => token_result!(self, Multiply),
            Some('/') => token_result!(self, Divide),
            Some('(') => token_result!(self, Lparen),
            Some(')') => token_result!(self, Rparen),
            Some(c) => {
                if is_number(c) {
                    Some(self.get_number())
                } else {
                    Some(Err(format!("Illegal character '{}'", c)))
                }
            }
            None => None,
        }
    }
}
