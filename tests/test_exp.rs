#![allow(unused)]

use rlisp::exp::*;
use rlisp::utils::exceptions::ParseError;

use std::collections::HashMap;

#[test]
fn test_tokenization() {

    assert_eq!(tokenize("(head '(1 2 3))".to_string()), 
        ["(", "head", "'", "(", "1", "2", "3", ")", ")"]);

    assert_eq!(tokenize("(head {1 2 3})".to_string()), 
        ["(", "head", "{", "1", "2", "3", "}", ")"]);

    assert_eq!(tokenize("((lambda (arg) (+ arg 1)) 5)".to_string()), 
        ["(", "(", "lambda", "(", "arg", ")", "(", "+", "arg", "1", ")", ")", "5", ")"]);
}

#[test]
fn test_parse_bool_token() {

    let boolean_tokens = HashMap::from([
        ("true", true),
        ("false", false),
    ]);

    for (token, value) in boolean_tokens {
        match parse_token(&token) {
           LispExp::Bool(boolean) => assert_eq!(boolean, value),
            _ => assert!(false)
        }
    }
}

#[test]
fn test_parse_number_token() {

    let number_tokens = HashMap::from([
        ("42", 42.0),
        ("4.5", 4.5),
        ("-13.564", -13.564),
    ]);

    for (token, value) in number_tokens {
        match parse_token(&token) {
            LispExp::Number(number) => assert_eq!(number, value),
            _ => assert!(false)
        }
    }
}

#[test]
fn test_parse_symbol_token() {

    let symbol_tokens = HashMap::from([
        ("+", "+".to_string()),
        ("(", "(".to_string()),
        (")", ")".to_string()),
        ("let", "let".to_string()),
        ("lambda", "lambda".to_string()),
    ]);
    
    for (token, value) in symbol_tokens {
        match parse_token(&token) {
            LispExp::Symbol(symbol) => assert_eq!(symbol, value),
            _ => assert!(false)
        }
    }
}


fn test_parse(input: &str, result: &str) {
    let tokens = tokenize(input.to_string());
    let (exp, _) = parse(&tokens[..]).unwrap();
    assert_eq!(exp.to_string() == result, true);
}

#[test]
fn test_parse_expression() {

    test_parse("(* (+ 4 1) 2)", "(* (+ 4 1) 2)");

    test_parse("'(1 2 3 4)", "(list 1 2 3 4)");

    test_parse("{1 2 3 4}", "(set 1 2 3 4)");

    test_parse("(define x {1 2 3})", "(define x (set 1 2 3))");
}


fn test_parse_exception(input: &str, exception: ParseError) {
    let tokens = tokenize(input.to_string());
    let result = parse(&tokens[..]);
    match result {
        Err(e) => assert_eq!(e, exception),
        Ok(_) => panic!("Test should not reach here!"),
    }
}

#[test]
fn test_parse_exceprions() {
    test_parse_exception("(+ 3 4", ParseError::MissingToken);

    test_parse_exception(")(+ 3 4", ParseError::UnexpectedToken(")".to_string()));

    test_parse_exception("'{+ 3 4}", ParseError::UnexpectedToken("{".to_string()));
}
