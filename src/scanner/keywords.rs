use std::{collections::HashMap, sync::OnceLock};

use super::token;

pub static KEYWORDS: OnceLock<HashMap<&str, token::Type>> = OnceLock::new();

pub fn map() -> &'static HashMap<&'static str, token::Type> {
    KEYWORDS.get_or_init(|| {
        let mut map = HashMap::new();
        map.insert("and", token::Type::And);
        map.insert("class", token::Type::Class);
        map.insert("else", token::Type::Else);
        map.insert("false", token::Type::False);
        map.insert("for", token::Type::For);
        map.insert("fun", token::Type::Fun);
        map.insert("if", token::Type::If);
        map.insert("nil", token::Type::Nil);
        map.insert("or", token::Type::Or);
        map.insert("print", token::Type::Print);
        map.insert("return", token::Type::Return);
        map.insert("super", token::Type::Super);
        map.insert("this", token::Type::This);
        map.insert("true", token::Type::True);
        map.insert("var", token::Type::Var);
        map.insert("while", token::Type::While);
        map
    })
}
