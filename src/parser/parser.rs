use std::iter::Peekable;

use crate::scanner::{Token, Type};

use super::expr::{BinaryOperator, Expr, UnaryOperator};

fn next_type_match<'a, I>(types: &Vec<Type>, tokens: &mut Peekable<I>) -> Option<Type>
where
    I: Iterator<Item = &'a Token>,
{
    let peek = tokens.peek();

    if let Some(peek) = peek {
        let peek = *peek;

        for r#type in types {
            if *r#type == peek.r#type {
                tokens.next();
                return Some(r#type.clone());
            }
        }
    }

    None
}

fn unary<'a, I>(tokens: &mut Peekable<I>) -> Expr
where
    I: Iterator<Item = &'a Token>,
{
    if let Some(r#type) = next_type_match(&vec![Type::Bang, Type::Minus], tokens) {
        let right = unary(tokens);
        let operator: UnaryOperator = r#type.into();

        return Expr::Unary(operator, Box::new(right));
    }

    primary(tokens)
}

fn next_token_type_match<'a, I>(r#type: &Type, tokens: &mut Peekable<I>) -> bool
where
    I: Iterator<Item = &'a Token>,
{
    if tokens.next_if(|token| token.r#type.eq(r#type)).is_some() {
        return true;
    }

    false
}

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

                if !next_token_type_match(&Type::RightParen, tokens) {
                    eprintln!("Error: Unmatched parentheses.");
                    std::process::exit(65);
                }

                Expr::Grouping(Box::new(expr))
            }
            Type::Print => {
                let expr = expression(tokens);
                Expr::Print(Box::new(expr))
            }
            _ => {
                eprintln!("Error: Unmatched parentheses.");
                std::process::exit(65);
            }
        }
    } else {
        eprintln!("Error: Unmatched parentheses.");
        std::process::exit(65);
    }
}

fn factor<'a, I>(tokens: &mut Peekable<I>) -> Expr
where
    I: Iterator<Item = &'a Token>,
{
    let mut expr = unary(tokens);

    while let Some(r#type) = next_type_match(&vec![Type::Star, Type::Slash], tokens) {
        let left = expr;
        let right = unary(tokens);

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

    while let Some(r#type) = next_type_match(&vec![Type::Minus, Type::Plus], tokens) {
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
        &vec![
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

    while let Some(r#type) = next_type_match(&vec![Type::EqualEqual, Type::BangEqual], tokens) {
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
    expression(&mut tokens.iter().peekable())
}
