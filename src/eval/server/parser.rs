use super::lexer::{BracketType, CalcError, Operator, Token};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum UnaryOperator {
    Plus,
    Minus,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Expr {
    NumberInt(i64),
    NumberFloat(f64),
    UnaryOp {
        op: UnaryOperator,
        operand: Box<Expr>,
    },
    BinaryOp {
        op: Operator,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

pub(crate) fn parse(tokens: &[Token]) -> Result<Expr, CalcError> {
    if tokens.is_empty() {
        return Err(CalcError::EmptyExpression);
    }

    let mut parser = Parser { tokens, pos: 0 };
    let expr = parser.parse_expression(0)?;
    if parser.pos != tokens.len() {
        return Err(CalcError::UnexpectedToken {
            position: parser.pos,
        });
    }
    Ok(expr)
}

struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl Parser<'_> {
    fn parse_expression(&mut self, min_binding_power: u8) -> Result<Expr, CalcError> {
        let mut left = self.parse_prefix()?;

        loop {
            let op = match self.tokens.get(self.pos) {
                Some(Token::Operator(op)) => *op,
                _ => break,
            };
            let (left_binding_power, right_binding_power) = match op {
                Operator::Plus | Operator::Minus => (1, 2),
                Operator::Multiply | Operator::Divide => (3, 4),
            };
            if left_binding_power < min_binding_power {
                break;
            }

            self.pos += 1;
            let right = self.parse_expression(right_binding_power)?;
            left = Expr::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_prefix(&mut self) -> Result<Expr, CalcError> {
        let token = self.tokens.get(self.pos).ok_or(CalcError::UnexpectedEnd)?;
        self.pos += 1;

        match token {
            Token::NumberInt(value) => Ok(Expr::NumberInt(*value)),
            Token::NumberFloat(value) => Ok(Expr::NumberFloat(*value)),
            Token::Operator(Operator::Plus) => self.parse_unary(UnaryOperator::Plus),
            Token::Operator(Operator::Minus) => self.parse_unary(UnaryOperator::Minus),
            Token::Bracket(BracketType::Opening) => {
                let expr = self.parse_expression(0)?;
                match self.tokens.get(self.pos) {
                    Some(Token::Bracket(BracketType::Closing)) => {
                        self.pos += 1;
                        Ok(expr)
                    }
                    _ => Err(CalcError::ExpectedClosingBracket { position: self.pos }),
                }
            }
            _ => Err(CalcError::UnexpectedToken {
                position: self.pos - 1,
            }),
        }
    }

    fn parse_unary(&mut self, op: UnaryOperator) -> Result<Expr, CalcError> {
        let operand = self.parse_expression(5)?;
        Ok(Expr::UnaryOp {
            op,
            operand: Box::new(operand),
        })
    }
}
