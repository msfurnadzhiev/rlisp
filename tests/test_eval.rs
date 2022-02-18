use rlisp::exp::*;
use rlisp::env::*;
use rlisp::eval::*;
use rlisp::utils::*;

use std::collections::HashMap;

// #[test]
// fn test_eval_env_operation() {
//     todo!();
// }

// #[test]
// fn test_eval_if_statement() {
//     todo!();
// }

#[test] 
fn test_define_variable() {

    let tokens = tokenize("(define x 7.2)".to_string());
    let (exp, _) = LispExp::parse(&tokens[..]).unwrap();
    let mut env = LispEnv::default_env();
    eval(exp, &mut env);

    if let LispInter::Number(n) = env.get("x").unwrap() {
        assert_eq!(n, &f64::from(7.2));
    } else {
        assert!(false);
    }
}

// #[test]
// fn test_eval_fn() {
//     todo!();
// }