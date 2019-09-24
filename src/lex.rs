use std::fmt;
use std::iter::Peekable;

use crate::leases::LeaseKeyword;
use crate::parser::ConfigKeyword;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexItem {
    Paren(char),
    Word(String),
    Opt(LeaseKeyword),
    Decl(ConfigKeyword),
}

impl fmt::Display for LexItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexItem::Paren(v) => v.fmt(f),
            LexItem::Word(v) => v.fmt(f),
            LexItem::Opt(v) => write!(f, "{}", v.to_string()),
            LexItem::Decl(v) => write!(f, "{}", v.to_string()),
        }
    }
}

pub fn lex(input: String) -> Result<Vec<LexItem>, String> {
    let mut result = Vec::new();

    let mut it = input.chars().peekable();
    while let Some(&c) = it.peek() {
        match c {
            '(' | ')' | '[' | ']' | '{' | '}' => {
                result.push(LexItem::Paren(c));
                it.next();
            }
            ' ' | '\n' => {
                it.next();
            }
            _ => {
                let w = get_word(&mut it);
                let kw = ConfigKeyword::from(&w);
                if kw.is_ok() {
                    result.push(LexItem::Decl(kw.unwrap()));
                } else {
                    let kw = LeaseKeyword::from(&w);
                    if kw.is_ok() {
                        result.push(LexItem::Opt(kw.unwrap()));
                    } else {
                        result.push(LexItem::Word(w));
                    }
                }
            }
        }
    }
    Ok(result)
}

fn get_word<T: Iterator<Item = char>>(iter: &mut Peekable<T>) -> String {
    let mut word = String::new();

    while let Some(&nc) = iter.peek() {
        if nc.is_whitespace() {
            break;
        }

        word.push(nc);
        iter.next();
    }
    word
}
