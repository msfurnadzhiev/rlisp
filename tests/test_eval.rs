#![allow(unused)]

use rlisp::exp::*;
use rlisp::env::*;
use rlisp::eval::*;

#[test]
fn test_eval_if_statement() {

    let mut env = default_env();

    let tokens = tokenize("(if true 7 5)".to_string());
    let (exp, _) = parse(&tokens[..]).unwrap();
    let value = eval(exp, &mut env);

    if let LispEval::Number(n) = value.unwrap() {
        assert_eq!(n, f64::from(7));
    } else {
        assert!(false);
    }
}

#[test] 
fn test_define_variable() {

    let mut env = default_env();

    let tokens = tokenize("(define x 7.2)".to_string());
    let (exp, _) = parse(&tokens[..]).unwrap();
    eval(exp, &mut env);

    if let LispEval::Number(n) = env.get("x").unwrap() {
        assert_eq!(n, &f64::from(7.2));
    } else {
        assert!(false);
    }
}

#[test] 
fn test_call_function() {
    let mut env = default_env();

    let tokens = tokenize("(+ 3 4)".to_string());
    let (exp, _) = parse(&tokens[..]).unwrap();
    let value = eval(exp, &mut env);

    if let LispEval::Number(n) = value.unwrap() {
        assert_eq!(n, f64::from(7));
    } else {
        assert!(false);
    }
}

#[test] 
fn test_define_function() {

    let mut env = default_env();

    let tokens = tokenize("(fn plus a b (+ a b))".to_string());
    let (exp, _) = parse(&tokens[..]).unwrap();
    eval(exp, &mut env);

    let tokens = tokenize("(plus 3 4)".to_string());
    let (exp, _) = parse(&tokens[..]).unwrap();
    let value = eval(exp, &mut env);

    if let LispEval::Number(n) = value.unwrap() {
        assert_eq!(n, f64::from(7));
    } else {
        assert!(false);
    }
}

#[test] 
fn test_define_lambda_as_variable() {

    let mut env = default_env();

    let tokens = tokenize("(define plus (lambda a b (+ a b)))".to_string());
    let (exp, _) = parse(&tokens[..]).unwrap();
    eval(exp, &mut env);

    let tokens = tokenize("(plus 3 4)".to_string());
    let (exp, _) = parse(&tokens[..]).unwrap();
    let value = eval(exp, &mut env);

    if let LispEval::Number(n) = value.unwrap() {
        assert_eq!(n, f64::from(7));
    } else {
        assert!(false);
    }
}