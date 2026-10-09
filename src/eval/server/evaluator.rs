use super::{
    lexer::{CalcError, Operator, tokenize},
    parser::{Expr, UnaryOperator, parse},
};

pub(crate) fn evaluate_expression(expression: &str) -> Result<f64, CalcError> {
    let tokens = tokenize(expression.to_owned())?;
    let ast = parse(&tokens)?;
    evaluate_ast(&ast)
}

fn evaluate_ast(expr: &Expr) -> Result<f64, CalcError> {
    let value = match expr {
        Expr::NumberInt(value) => *value as f64,
        Expr::NumberFloat(value) => *value,
        Expr::UnaryOp { op, operand } => {
            let operand = evaluate_ast(operand)?;
            match op {
                UnaryOperator::Plus => operand,
                UnaryOperator::Minus => -operand,
            }
        }
        Expr::BinaryOp { op, left, right } => {
            let left = evaluate_ast(left)?;
            let right = evaluate_ast(right)?;
            match op {
                Operator::Plus => left + right,
                Operator::Minus => left - right,
                Operator::Multiply => left * right,
                Operator::Divide => {
                    if right == 0.0 {
                        return Err(CalcError::DivisionByZero);
                    }
                    left / right
                }
            }
        }
    };

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::super::lexer::CalcError;
    use super::evaluate_expression;

    #[test]
    fn arithmetic() {
        for (expression, expected) in [
            ("42", 42.0),
            ("1.5", 1.5),
            ("7 + 3", 10.0),
            ("7 - 3", 4.0),
            ("7 * 3", 21.0),
            ("7 / 2", 3.5),
        ] {
            assert_eq!(
                evaluate_expression(expression),
                Ok(expected),
                "{expression}"
            );
        }
    }

    #[test]
    fn precedence() {
        for (expression, expected) in [
            ("2 + 3 * 4", 14.0),
            ("(2 + 3) * 4", 20.0),
            ("20 - 5 - 3", 12.0),
            ("24 / 4 / 2", 3.0),
            ("10 - 6 / 2 + 1", 8.0),
        ] {
            assert_eq!(
                evaluate_expression(expression),
                Ok(expected),
                "{expression}"
            );
        }
    }

    #[test]
    fn other() {
        for (expression, expected) in [
            ("-5", -5.0),
            ("+5", 5.0),
            ("--5", 5.0),
            ("2 * -3", -6.0),
            ("-(2 + 3) * 4", -20.0),
            (" \t(1 + 2)\n * 3 ", 9.0),
        ] {
            assert_eq!(
                evaluate_expression(expression),
                Ok(expected),
                "{expression}"
            );
        }
    }

    #[test]
    fn errors() {
        for (expression, expected) in [
            ("", CalcError::EmptyExpression),
            (" \t\n", CalcError::EmptyExpression),
            ("1 +", CalcError::UnexpectedEnd),
            ("(1 + 2", CalcError::ExpectedClosingBracket { position: 4 }),
            ("1 2", CalcError::UnexpectedToken { position: 1 }),
            (
                "1a",
                CalcError::UnexpectedCharacter {
                    position: 1,
                    character: 'a',
                },
            ),
            ("1.", CalcError::InvalidNumber { position: 0 }),
            ("10 / (3 - 3)", CalcError::DivisionByZero),
        ] {
            assert_eq!(
                evaluate_expression(expression),
                Err(expected),
                "{expression}"
            );
        }
    }
}
