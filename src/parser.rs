use crate::lex::lex;
use crate::lex::LexItem;
use crate::leases::Lease;
use crate::leases::Leases;
pub use crate::leases::LeasesMethods;
use crate::leases::parse_lease;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParserResult {
    pub leases: Leases
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigKeyword {
    Lease
}

impl ConfigKeyword {
    pub fn to_string(&self) -> String {
        match self {
            &ConfigKeyword::Lease => "lease".to_owned(),
        }
    }

    pub fn from(s: &str) -> Result<ConfigKeyword, String> {
        match s {
            "lease" => Ok(ConfigKeyword::Lease),
            _ => Err(format!("'{}' declaration is not supported", s)),
        }
    }
}


fn parse_config(tokens: Vec<LexItem>) -> Result<ParserResult, String> {
    let mut leases = Leases::new();
    let lease = Lease::new();

    let mut it = tokens.iter().peekable();

    while let Some(token) = it.peek() {
        match token {
            LexItem::Decl(ConfigKeyword::Lease) => {
                if lease != Lease::new() {
                    leases.push(lease.clone());
                }

                let mut lease = Lease::new();
                // ip-address
                it.next();
                lease.ip = it.peek().expect("IP address expected").to_string();

                // left curly brace
                it.next();
                assert_eq!(it.peek().unwrap().to_owned(), &LexItem::Paren('{'));

                // statements for the lease
                it.next();
                parse_lease(&mut lease, &mut it)?;

                // right curly brace
                if it.peek().is_none() || it.peek().unwrap().to_owned() != &LexItem::Paren('}') {
                    return Err(format!(
                        "Expected end of section with '}}', got '{:?}'",
                        it.peek(),
                    ));
                }

                leases.push(lease.clone());
                it.next();
            }
            _ => {
                return Err(format!("Unexpected {:?}", it.peek()));
            }
        }
    }

    Ok(ParserResult {
        leases: leases,
    })
}

pub fn parse(input: String) -> Result<ParserResult, String> {
    let tokens = lex(input).unwrap();
    return parse_config(tokens);
}
