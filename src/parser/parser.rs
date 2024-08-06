use std::iter::Peekable;

use crate::scanner::{Token, Type};

use super::expr::{BinaryOperator, Expr, UnaryOperator};

fn check_token_type(token: Option<&Token>, token_type: &Type) -> bool {
    if let Some(next) = token {
        if next.r#type == *token_type {
            return true;
        }
    }
    false
}

fn next_type_match<'a, I>(types: &Vec<Type>, tokens: &mut Peekable<I>) -> Option<Type>
where
    I: Iterator<Item = &'a Token>,
{
    let peek = tokens.peek();

    if let Some(peek) = peek {
        let peek = *peek;

        for ty in types {
            if *ty == peek.r#type {
                tokens.next();
                return Some(ty.clone());
            }
        }
    }

    None
}

pub fn unary<'a, I>(tokens: &mut Peekable<I>) -> Expr
where
    I: Iterator<Item = &'a Token>,
{
    if let Some(ty) = next_type_match(&vec![Type::Bang, Type::Minus], tokens) {
        let right = unary(tokens);
        let operator: UnaryOperator = ty.into();

        return Expr::Unary(operator, Box::new(right));
    }

    primary(tokens)
}

pub fn primary<'a, I>(tokens: &mut Peekable<I>) -> Expr
where
    I: Iterator<Item = &'a Token>,
{
    let token = tokens.next().unwrap();

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
            let expr = parse_tokens(tokens);
            let next = tokens.next();

            if !check_token_type(next, &Type::RightParen) {
                eprintln!("Error: Unmatched parentheses.");
                std::process::exit(65);
            }

            Expr::Grouping(Box::new(expr))
        }
        _ => {
            eprintln!("Error: Unknown token type.");
            std::process::exit(65);
        }
    }
}

pub fn parse_tokens<'a, I>(tokens: &mut Peekable<I>) -> Expr
where
    I: Iterator<Item = &'a Token>,
{
    let mut expr = unary(tokens);

    while let Some(ty) = next_type_match(&vec![Type::Star, Type::Slash], tokens) {
        let left = expr;
        let right = unary(tokens);

        let operator: BinaryOperator = ty.into();

        expr = Expr::Binary(operator, Box::new(left), Box::new(right));
    }

    expr
}
