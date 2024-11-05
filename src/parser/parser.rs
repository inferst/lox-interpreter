use std::iter::Peekable;

use crate::scanner::{Token, Type};

use super::expr::{BinaryOperator, Expr, UnaryOperator};

fn unary<'a, I>(tokens: &mut Peekable<I>) -> Expr
where
    I: Iterator<Item = &'a Token>,
{
    if let Some(r#type) = next_type_match(&[Type::Bang, Type::Minus], tokens) {
        let right = unary(tokens);
        let operator: UnaryOperator = r#type.into();

        return Expr::Unary(operator, Box::new(right));
    }

    primary(tokens)
}

fn next_type_match<'a, I>(types: &[Type], tokens: &mut Peekable<I>) -> Option<Type>
where
    I: Iterator<Item = &'a Token>,
{
    let token = tokens.next_if(|token| types.contains(&token.r#type));

    if let Some(token) = token {
        return Some(token.r#type);
    }

    None
}

#[allow(clippy::too_many_lines)]
fn primary<'a, I>(tokens: &mut Peekable<I>) -> Expr
where
    I: Iterator<Item = &'a Token>,
{
    if let Some(token) = tokens.next() {
        match token.r#type {
            Type::True => Expr::True,
            Type::False => Expr::False,
            Type::Nil => Expr::Nil,
            Type::Number => {
                let value = token.lexeme.parse::<f64>().unwrap();
                Expr::Number(value)
            }
            Type::String => {
                let literal = &token.literal;
                let string = literal.clone().unwrap();
                Expr::String(string.to_string())
            }
            Type::LeftParen => {
                let expr = expression(tokens);

                if next_type_match(&[Type::RightParen], tokens).is_none() {
                    eprintln!("Error: Unmatched parentheses.");
                    std::process::exit(65);
                }

                Expr::Grouping(Box::new(expr))
            }
            Type::LeftBrace => {
                if next_type_match(&[Type::RightBrace], tokens).is_some() {
                    return Expr::Nil;
                }

                let mut statements = vec![];

                while tokens.peek().is_some() {
                    statements.push(expression(tokens));

                    if let Some(token) = tokens.peek() {
                        if token.r#type == Type::RightBrace {
                            break;
                        }
                    }
                }

                if next_type_match(&[Type::RightBrace], tokens).is_none() {
                    eprintln!("Error: Unmatched braces.");
                    std::process::exit(65);
                }

                Expr::Statements(statements)
            }
            Type::Identifier => {
                let lexeme = &token.lexeme;

                let token = tokens.peek();
                if let Some(value) = token {
                    if value.r#type == Type::Equal {
                        tokens.next();
                        let expr = expression(tokens);
                        return Expr::Assignment(lexeme.clone(), Box::new(expr), false);
                    }

                    return Expr::Identifier(lexeme.clone());
                }

                Expr::Nil
            }
            Type::Var => {
                let token = tokens.next();
                let mut var = String::new();
                let mut expr = Expr::Nil;

                if let Some(value) = token {
                    if value.r#type == Type::Identifier {
                        var = String::from(&value.lexeme);
                    } else {
                        eprintln!("Error: Expected identifier.");
                        std::process::exit(65);
                    }
                }

                let token = tokens.next();
                if let Some(value) = token {
                    if value.r#type == Type::Equal {
                        expr = expression(tokens);
                    } else if value.r#type == Type::Semicolon {
                        expr = Expr::Semicolon;
                    } else {
                        eprintln!("Error: Expected '=' or ';'.");
                        std::process::exit(65);
                    }
                }

                Expr::Assignment(var, Box::new(expr), true)
            }
            Type::Print => {
                let expr = expression(tokens);

                if let Expr::Semicolon = expr {
                    eprintln!("Error: Print statement must have an expression.");
                    std::process::exit(65);
                }

                Expr::Print(Box::new(expr))
            }
            Type::Semicolon => Expr::Semicolon,
            Type::If => {
                let expr1 = expression(tokens);
                let expr2 = expression(tokens);

                if let Some(token) = tokens.peek() {
                    let mut token_type = token.r#type;

                    if token_type == Type::Semicolon {
                        tokens.next();

                        if let Some(token) = tokens.peek() {
                            token_type = token.r#type;
                        }
                    }

                    if token_type == Type::Else {
                        tokens.next();

                        let else_expr = expression(tokens);
                        return Expr::IfElse(
                            Box::new(expr1),
                            Box::new(expr2),
                            Some(Box::new(else_expr)),
                        );
                    }
                }

                Expr::IfElse(Box::new(expr1), Box::new(expr2), None)
            }
            Type::While => {
                let expr1 = expression(tokens);
                let expr2 = expression(tokens);

                Expr::While(Box::new(expr1), Box::new(expr2))
            }
            Type::For => {
                if let Some(token) = tokens.next() {
                    if token.r#type == Type::LeftParen {
                        let expr1 = expression(tokens);
                        let mut some_expr1 = None;

                        if let Expr::Semicolon = expr1 {
                            some_expr1 = None;
                        } else {
                            let expr = expression(tokens);
                            match expr {
                                Expr::Semicolon => {
                                    some_expr1 = Some(Box::new(expr1));
                                }
                                _ => {
                                    eprintln!("Error 1");
                                }
                            }
                        }

                        let expr2 = expression(tokens);

                        let expr3 = expression(tokens);
                        let mut some_expr3 = None;

                        if let Expr::Semicolon = expr3 {
                            if let Some(token) = tokens.peek() {
                                if token.r#type != Type::RightParen {
                                    let expr = expression(tokens);
                                    some_expr3 = Some(Box::new(expr));
                                }
                            }
                        } else {
                            some_expr3 = None;
                        }

                        if let Some(token) = tokens.next() {
                            if token.r#type != Type::RightParen {
                                eprintln!("Error 3");
                            }
                        }

                        let expr4 = expression(tokens);

                        return Expr::For(some_expr1, Box::new(expr2), some_expr3, Box::new(expr4));
                    }
                }

                Expr::Nil
            }
            _ => {
                eprintln!("Error: Unknown token type {:?}.", token.r#type);
                std::process::exit(65);
            }
        }
    } else {
        eprintln!("Error: Expected token.");
        std::process::exit(65);
    }
}

fn or<'a, I>(tokens: &mut Peekable<I>) -> Expr
where
    I: Iterator<Item = &'a Token>,
{
    let mut expr = and(tokens);

    while next_type_match(&[Type::Or], tokens).is_some() {
        let left = expr;
        let right = and(tokens);

        expr = Expr::Or(Box::new(left), Box::new(right));
    }

    expr
}

fn and<'a, I>(tokens: &mut Peekable<I>) -> Expr
where
    I: Iterator<Item = &'a Token>,
{
    let mut expr = unary(tokens);

    while next_type_match(&[Type::And], tokens).is_some() {
        let left = expr;
        let right = unary(tokens);

        expr = Expr::And(Box::new(left), Box::new(right));
    }

    expr
}

fn factor<'a, I>(tokens: &mut Peekable<I>) -> Expr
where
    I: Iterator<Item = &'a Token>,
{
    let mut expr = or(tokens);

    while let Some(r#type) = next_type_match(&[Type::Star, Type::Slash], tokens) {
        let left = expr;
        let right = or(tokens);

        let operator: BinaryOperator = r#type.into();

        expr = Expr::Binary(operator, Box::new(left), Box::new(right));
    }

    expr
}

fn term<'a, I>(tokens: &mut Peekable<I>) -> Expr
where
    I: Iterator<Item = &'a Token>,
{
    let mut expr = factor(tokens);

    while let Some(r#type) = next_type_match(&[Type::Minus, Type::Plus], tokens) {
        let left = expr;
        let right = factor(tokens);

        let operator: BinaryOperator = r#type.into();

        expr = Expr::Binary(operator, Box::new(left), Box::new(right));
    }

    expr
}

fn comparison<'a, I>(tokens: &mut Peekable<I>) -> Expr
where
    I: Iterator<Item = &'a Token>,
{
    let mut expr = term(tokens);

    while let Some(r#type) = next_type_match(
        &[
            Type::Greater,
            Type::GreaterEqual,
            Type::Less,
            Type::LessEqual,
        ],
        tokens,
    ) {
        let left = expr;
        let right = term(tokens);

        let operator: BinaryOperator = r#type.into();

        expr = Expr::Binary(operator, Box::new(left), Box::new(right));
    }

    expr
}

fn equality<'a, I>(tokens: &mut Peekable<I>) -> Expr
where
    I: Iterator<Item = &'a Token>,
{
    let mut expr = comparison(tokens);

    while let Some(r#type) = next_type_match(&[Type::EqualEqual, Type::BangEqual], tokens) {
        let left = expr;
        let right = comparison(tokens);

        let operator: BinaryOperator = r#type.into();

        expr = Expr::Binary(operator, Box::new(left), Box::new(right));
    }

    expr
}

fn expression<'a, I>(tokens: &mut Peekable<I>) -> Expr
where
    I: Iterator<Item = &'a Token>,
{
    equality(tokens)
}

pub fn parse_tokens(tokens: &[Token]) -> Expr {
    let mut statements = vec![];
    let mut tokens = tokens.iter().peekable();

    while tokens.peek().is_some() {
        statements.push(expression(&mut tokens));
    }

    if statements.len() == 1 {
        statements[0].clone()
    } else {
        Expr::Statements(statements)
    }
}
