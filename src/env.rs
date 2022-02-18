use std::collections::HashMap;

// use crate::exp::LispExp;
use crate::eval::LispEval;

use crate::utils::exceptions::EvalError;

#[derive(Clone)]
pub struct LispEnv {
    pub set: HashMap<String, LispEval>
}

impl LispEnv {

    pub fn new() -> Self {
        LispEnv {
            set: HashMap::new()
        }
    }

    pub fn get(&self, symbol: &str) -> Option<&LispEval> {
        self.set.get(symbol)
    }

    pub fn add(&mut self, symbol: &str, exp: LispEval) {
        self.set.insert(symbol.to_string(), exp);
    }

}

pub fn default_env() -> LispEnv {
    let mut env = LispEnv::new();

    // Basic math functions
    env.add("+", LispEval::Func(add));
    env.add("-", LispEval::Func(sub));
    env.add("*", LispEval::Func(mul));
    env.add("/", LispEval::Func(dev));
    env.add("%", LispEval::Func(modulus));

    // Basic comparison relations
    env.add("=", LispEval::Func(equals));
    env.add("<", LispEval::Func(less_than));
    env.add(">", LispEval::Func(more_than));
    env.add("<=", LispEval::Func(less_or_equal));
    env.add(">=", LispEval::Func(more_or_equal));

    // Math constants
    env.add("pi", LispEval::Number(core::f64::consts::PI));
    env.add("e",  LispEval::Number(core::f64::consts::E));

    // List operations
    env.add("list", LispEval::Func(to_list));
    env.add("head", LispEval::Func(list_head));
    // tail
    // concat

    // Set operations
    env.add("set", LispEval::Func(to_list));
    // some set operations

    // To deine some functional methods: map, apply, ...

    env
}


fn accumulate(args: &LispEval, op: fn(f64, f64)->f64
) -> Result<LispEval, EvalError> {
    
    let mut res = 0.0;
    if let LispEval::List(list) = args {
        for (i, arg) in list.iter().enumerate() {
            if let LispEval::Number(n) = arg {
                if i == 0 {
                    res = *n;
                } else {
                    res = op(res, *n);
                }
            }
        }
    }
    Ok(LispEval::Number(res))
}

fn add(args: &LispEval) -> Result<LispEval, EvalError>  {
    accumulate(args, |x,y| x+y)
}

fn sub(args: &LispEval) -> Result<LispEval, EvalError> {
    accumulate(args, |x,y| x-y)
}

fn mul(args: &LispEval) -> Result<LispEval, EvalError> {
    accumulate(args, |x,y| x*y)
}

fn dev(args: &LispEval) -> Result<LispEval, EvalError> {
    accumulate(args, |x,y| x/y)
}

fn modulus(args: &LispEval) -> Result<LispEval, EvalError> {
    accumulate(args, |x,y| x%y)
}


fn compare(args: &LispEval, rel: fn(f64, f64)->bool
) -> Result<LispEval, EvalError> 
{
    let mut prev = 0.0;
    let mut res = false;
    if let LispEval::List(list) = args {
        for (i, arg) in list.iter().enumerate() {
            if let LispEval::Number(n) = arg {
                if i > 0 {
                    res = rel(prev, *n);
                }
                prev = *n;
            }
        }
    }
    Ok(LispEval::Bool(res))
}

fn equals(args: &LispEval) -> Result<LispEval, EvalError> {
    compare(args, |x,y| x==y)
}

fn less_than(args: &LispEval) -> Result<LispEval, EvalError> {
    compare(args, |x,y| x<y)
}

fn more_than(args: &LispEval) -> Result<LispEval, EvalError> {
    compare(args, |x,y| x>y)
}

fn less_or_equal(args: &LispEval) -> Result<LispEval, EvalError> {
    compare(args, |x,y| x<=y)
}

fn more_or_equal(args: &LispEval) -> Result<LispEval, EvalError> {
    compare(args, |x,y| x>=y)
}

fn to_list(args: &LispEval) -> Result<LispEval, EvalError> {
    match args {
        LispEval::List(_) => Ok(args.clone()),
        _ => Err(EvalError::InvalidAgrumentType())
    }
}

fn list_head(args: &LispEval) -> Result<LispEval, EvalError> {
    if let LispEval::List(args_vec) = args {
       if args_vec.len() > 1 {
           return Err(EvalError::InvalidNumberOfArguments ());
       }
       match &args_vec[0] {
           LispEval::List(list) => Ok(list[0].clone()),
           _ => Err(EvalError::InvalidAgrumentType())
       }
    }
    else {
        Err(EvalError::UnexpectedExpression())
    }
}

// fn to_set(args: &LispEval) -> Result<LispEval, EvalError> {
//     match args {
//         LispEval::List(values) => {
//             Ok(LispEval::List(values.dedup()))
//         },
//         _ => Err(EvalError::InvalidAgrumentType())
//     }
// }