#![allow(unused)]

use std::fmt;

use crate::exp::LispExp;
use crate::env::LispEnv;

use crate::utils::exceptions::EvalError;


// LispEval is interpretation of LispExp
#[derive(Clone)]
pub enum LispEval {
    Bool(bool),
    Number(f64),
    List(Vec<LispEval>),
    Func(fn(&LispEval) -> Result<LispEval, EvalError>),
    Lambda(LispLambda),
}

#[derive(Clone)]
pub struct LispLambda {
  params: Box<Vec<LispExp>>,
  body: Box<LispExp>,
}

pub fn eval(exp: LispExp, env: &mut LispEnv) -> Result<LispEval, EvalError> {

    match exp {
        LispExp::Bool(b) => Ok(LispEval::Bool(b)),
        LispExp::Number(n) => Ok(LispEval::Number(n)),
        LispExp::Symbol(s) => {
            let env_value = env.get(&s);       
            match env_value {
                Some(x) => Ok(x.clone()),
                None => Err(EvalError::UnknownSymbol(s.to_string()))
            } 
        },
        LispExp::List(list) => eval_list(list, env),
    }
}


fn eval_list(list: Vec<LispExp>, env: &mut LispEnv
) -> Result<LispEval, EvalError> {

    let (head, tail) = list.split_first()
        .ok_or(
            EvalError::InvalidListExpression()
        )?;
    match head {
        LispExp::List(first) => {
            match eval(LispExp::List(first.clone()), env) {
                Ok(_) => eval(LispExp::List(tail.to_vec()), env),
                Err(e) => Err(e)
            }
        },
        LispExp::Symbol(s) => {
            eval_symbol(s.clone(), tail, env)
        },
        _ => {
            Err(EvalError::InvalidListExpression())
        }
    }
}

fn eval_symbol(symbol: String, args: &[LispExp], env: &mut LispEnv
) -> Result<LispEval, EvalError> {

    match symbol.as_str() {

        "if" => if_statement(args, env),
        "define" => define_variable(args, env),
        "lambda" => define_lambda(args, env),
        "fn" => define_function(args, env),
        _ => {
            call_function(&symbol, args, env)
        }
    }
}


fn if_statement( args: &[LispExp], env: &mut LispEnv
) -> Result<LispEval, EvalError> {

    if args.len() != 3 {
        Err(EvalError::InvalidNumberOfArguments())
    }
    else {
        let if_exp:LispEval = eval(args[0].clone(), env)?;
        match if_exp {
            LispEval::Bool(res) => {
                if res {
                    Ok(eval(args[1].clone(), env)?)
                } else {
                    Ok(eval(args[2].clone(), env)?)
                }
            },
            _ => Err(EvalError::InvalidIfStatement())
        }
    }
}

fn define_variable(args: &[LispExp], env: &mut LispEnv
) -> Result<LispEval, EvalError> {

    if args.len() != 2 {
        return Err(EvalError::InvalidNumberOfArguments());
    } 
        
    let variable_name = args[0].to_string();
    let variable_value = eval(args[1].clone(), env)?;
        
    env.add(&variable_name, variable_value.clone());

    Ok(variable_value)
}

fn define_function(args: &[LispExp], env: &mut LispEnv
) -> Result<LispEval, EvalError> {

    if args.len() < 3 {
        Err(EvalError::InvalidNumberOfArguments())            
    } 
    else {

        let fn_name: String = args[0].to_string();

        // Collect argument definitions as a list of LispExp::Symbol
        println!("{}", args[1]);
        let params: Vec<LispExp> = args[1 .. args.len()-1].to_vec();

        // Add the function definition to the end
        let fn_def = args.last().unwrap().clone();

        // Create Lambda and insert into the current scope
        // let lambda_exp = define_lambda(&fn_def, env).unwrap();
        let lambda_exp = LispEval::Lambda(
            LispLambda {
                params: Box::new(params),
                body: Box::new(fn_def)
            }
        );

        env.add(&fn_name, lambda_exp);

        Ok(LispEval::Bool(true))
    }    
}

fn define_lambda(args: &[LispExp], env: &mut LispEnv
) -> Result<LispEval, EvalError> {

    if args.len() < 2 {
        Err(EvalError::InvalidNumberOfArguments())            
    } 
    else {

        // Collect argument definitions as a list of LispExp::Symbol
        let params: Vec<LispExp> = args[0 .. args.len()-1].to_vec();

        // Add the function definition to the end
        let fn_def = args.last().unwrap().clone();

        // Create Lambda and insert into the current scope
        // let lambda_exp = define_lambda(&fn_def, env).unwrap();
        let lambda_exp = LispEval::Lambda(
            LispLambda {
                params: Box::new(params),
                body: Box::new(fn_def)
            }
        );

        Ok(lambda_exp)
    }    
}


fn call_function(symbol: &str, args: &[LispExp], env: &mut LispEnv
) -> Result<LispEval, EvalError> {

    let env_fn = env.get(&symbol).ok_or(
        EvalError::NonDefineInThisScope
    )?.clone();

    
    match env_fn {
        LispEval::Func(func) => {
            let mut evaluated_args: Vec<LispEval> = vec![];
            for arg in args.iter() {
                evaluated_args.push(eval(arg.clone(), env)?);
            }
            func(&LispEval::List(evaluated_args))
        },
        LispEval::Lambda(lambda) => {
            if args.len() < 1 {
                Err(EvalError::InvalidNumberOfArguments())
            } 
            else {
                // Iterate over args and evalute each one
                let ev_args: Vec<LispEval> = args[0..args.len()].iter()
                                                .map(|a| eval(a.clone(), env)
                                                .unwrap())
                                                .collect();
                
                // Create new env to be the inherited sub-scope
                let mut sub_env = env.clone();
                
                // Set the args as a sub_env variables
                // Iterate over lambda from 0 .. len - 1 to get all args
                for i  in 0 .. lambda.params.len() {
                    let arg_def = lambda.params[i].to_string();
                    let arg_ev = ev_args.get(i).unwrap().clone();
                    sub_env.add(&arg_def, arg_ev);
                }

                // Get the lambda expression
                let fn_exp:LispExp = *lambda.body.clone();

                // Evalute lambda function call in new env and return the result
                Ok(eval(fn_exp, &mut sub_env)?)
            }
        }
        _ => {
            Err(EvalError::InvalidFunctionCall)
        }
    }
}


impl fmt::Display for LispEval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str:String = match self {
            LispEval::Bool(b) => LispExp::Bool(*b).to_string(),
            LispEval::Number(n) => LispExp::Number(*n).to_string(),
            LispEval::List(list) => {
                let items:Vec<String> = list.iter().map(
                    |item| item.to_string()
                ).collect();
                format!("({})", items.join(" "))
            },
            _ => format!("Interpretation cannot be displayed!")
        };
        write!(f, "{}", str)
    }
}
