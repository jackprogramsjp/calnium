use crate::lexer::Lexer;
use crate::nodes::Node;
use crate::tokens::Token;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Option<Token>,
    started: Option<String>,
}

macro_rules! raise_error {
    () => {
        return Err("Invalid syntax".to_string())
    };
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Self {
            lexer,
            current: None,
            started: None,
        };
        if let Err(e) = parser.advance() {
            parser.started = Some(e);
        }
        parser
    }

    fn advance(&mut self) -> Result<(), String> {
        match self.lexer.next() {
            Some(tokres) => match tokres {
                Ok(tok) => self.current = Some(tok),
                Err(e) => {
                    self.current = None;
                    return Err(e);
                }
            },
            None => self.current = None,
        }
        Ok(())
    }

    pub fn parse(&mut self) -> Result<Box<Node>, String> {
        if let Some(started) = self.started.clone() {
            return Err(started);
        }

        if self.current.is_none() {
            panic!("Empty tokens list; it is a bug, please make sure to have 1+ tokens");
        }

        let result = self.expr();

        if self.current.is_some() {
            raise_error!();
        }

        result
    }

    fn expr(&mut self) -> Result<Box<Node>, String> {
        let mut result = self.term()?;

        while let Some(current) = self.current {
            use Node::*;
            use Token::*;
            match current {
                Plus => {
                    self.advance()?;
                    result = Box::new(AddNode(result, self.term()?));
                }
                Minus => {
                    self.advance()?;
                    result = Box::new(SubtractNode(result, self.term()?));
                }
                _ => break,
            }
        }

        Ok(result)
    }

    fn term(&mut self) -> Result<Box<Node>, String> {
        let mut result = self.factor()?;

        while let Some(current) = self.current {
            use Node::*;
            use Token::*;
            match current {
                Multiply => {
                    self.advance()?;
                    result = Box::new(MultiplyNode(result, self.factor()?));
                }
                Divide => {
                    self.advance()?;
                    result = Box::new(DivideNode(result, self.factor()?));
                }
                _ => break,
            }
        }

        Ok(result)
    }

    fn factor(&mut self) -> Result<Box<Node>, String> {
        let token = match self.current {
            Some(t) => t,
            None => return Err("No existing token to factor".to_string()),
        };
        use Node::*;
        use Token::*;

        match token {
            Lparen => {
                self.advance()?;
                let result = self.expr()?;

                if self.current != Some(Rparen) {
                    raise_error!();
                }

                self.advance()?;
                Ok(result)
            }
            Number(f) => {
                self.advance()?;
                Ok(Box::new(NumberNode(f)))
            }
            Plus => {
                self.advance()?;
                Ok(Box::new(PlusNode(self.factor()?)))
            }
            Minus => {
                self.advance()?;
                Ok(Box::new(MinusNode(self.factor()?)))
            }
            _ => raise_error!(),
        }
    }
}
