use std::fmt;

use crate::utils::exceptions::ParseError;

#[derive(Debug, Clone)]
pub enum LispExp {
    Bool(bool),
    Number(f64),
    Symbol(String),
    List(Vec<LispExp>),
}

pub fn tokenize(expr: String) -> Vec<String> {
    expr
        .replace("(", " ( ")
        .replace(")", " ) ")
        .replace("{", " { ")
        .replace("}", " } ")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect()
} 


// Parses an individual token (atom) and creates a boolean, number, or symbol expression.
pub fn parse_token(token: &str) -> LispExp {
    if let Result::Ok(value) = token.parse::<bool>() {
        LispExp::Bool(value)
    } else if let Result::Ok(value) = token.parse::<f64>() {
        LispExp::Number(value)
    } else {
        LispExp::Symbol(token.to_string())
    }
}

// Parses a vector of string tokens and creates corresponding LispExp objects
pub fn parse(tokens: &[String]) -> Result<(LispExp, &[String]), ParseError>{

    let mut parsed_result: Vec<LispExp> = Vec::new();

    let (token, mut rest)  = tokens.split_first()
        .ok_or(
            ParseError::EmptyInput()
        )?;

    match token.as_str() {
        "'" => {
            let (next, _) = rest.split_first()
                    .ok_or(
                        ParseError::MissingToken
                    )?;
            if next == "(" {
                parsed_result.push(LispExp::Symbol("list".to_string()));
                let (exp,rest) = parse(&rest)?;
                if let LispExp::List(mut value) =  exp {
                    parsed_result.append(&mut value);
                    Ok((LispExp::List(parsed_result), rest))
                }
                else {
                    Err(ParseError::UnexpectedExpression())
                }
            }
            else {
                Err(ParseError::UnexpectedToken(next.to_string()))
            }
        },
        "(" => {
            loop {
                let (next, next_rest) = rest.split_first()
                    .ok_or(
                        ParseError::MissingToken
                    )?;
                if next == ")" {
                    return Ok((LispExp::List(parsed_result), next_rest))
                }
                else {
                    let (exp, next_rest) = parse(&rest)?;
                    parsed_result.push(exp);
                    rest = next_rest;
                }
            }
        },
        "{" => {
            parsed_result.push(LispExp::Symbol("set".to_string()));
            loop {
                let (next, next_rest) = rest.split_first()
                    .ok_or(
                        ParseError::MissingToken
                    )?;
                if next == "}" {
                    return Ok((LispExp::List(parsed_result), next_rest))
                }
                else {
                    let (exp, next_rest) = parse(&rest)?;
                    parsed_result.push(exp);
                    rest = next_rest;
                }
            }
        },
        ")" => {
            Err(ParseError::UnexpectedToken(")".to_string()))
        },
        "}" => {
            Err(ParseError::UnexpectedToken(")".to_string()))
        },
        _ => {
            Ok((parse_token(&token), rest))
        }
    }
}



impl fmt::Display for LispExp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str:String = match self {
            LispExp::Bool(b) => b.to_string(),
            LispExp::Number(n) => n.to_string(),
            LispExp::Symbol(s) => s.to_string(),
            LispExp::List(list) => {
                let items:Vec<String> = list.iter().map(
                    |item| item.to_string()
                ).collect();
                format!("({})", items.join(" "))
            },
        };
        write!(f, "{}", str)
    }
}

//Debug