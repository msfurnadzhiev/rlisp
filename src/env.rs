use std::collections::HashMap;
use std::collections::HashSet;

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

    env.add("print", LispEval::Func(print));

    // Logical operators
    env.add("not", LispEval::Func(not));
    env.add("and", LispEval::Func(and));
    env.add("or", LispEval::Func(or));

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
    env.add("head", LispEval::Func(head));
    env.add("cons", LispEval::Func(cons));

    // Set operations
    env.add("set", LispEval::Func(to_set));
    env.add("union", LispEval::Func(union));
    env.add("inter", LispEval::Func(intersection));

    env
}


fn unary_logic_operator(args: &LispEval, op: fn(bool)->bool
) -> Result<LispEval, EvalError> {
    let mut res = false;
    if let LispEval::List(list) = args {
        if let LispEval::Bool(b) = list[0] {
            res = op(b)
        }
    }
    Ok(LispEval::Bool(res))
}

fn not(args: &LispEval) -> Result<LispEval, EvalError>  {
    unary_logic_operator(args, |x| !x)
}

fn binary_logic_operator(args: &LispEval, op: fn(bool, bool)->bool
) -> Result<LispEval, EvalError> {
    
    let mut res = false;
    if let LispEval::List(list) = args {
        for (i, arg) in list.iter().enumerate() {
            if let LispEval::Bool(b) = arg {
                if i == 0 {
                    res = *b;
                } else {
                    res = op(res, *b);
                }
            }
        }
    }
    Ok(LispEval::Bool(res))
}

fn and(args: &LispEval) -> Result<LispEval, EvalError>  {
    binary_logic_operator(args, |x,y| x&&y)
}

fn or(args: &LispEval) -> Result<LispEval, EvalError>  {
    binary_logic_operator(args, |x,y| x||y)
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

fn head(args: &LispEval) -> Result<LispEval, EvalError> {
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

fn cons(args: &LispEval) -> Result<LispEval, EvalError> {
    if let LispEval::List(args_vec) = args {
        if args_vec.len() != 2 {
            return Err(EvalError::InvalidNumberOfArguments ());
        }
        let mut result = Vec::new();
        result.push(args_vec[0].clone());

        if let LispEval::List(list) = &args_vec[1] {
            for item in list {
                result.push(item.clone());
            }
        }
        Ok(LispEval::List(result))
    }
    else {
        Err(EvalError::UnexpectedExpression())
    }
}

fn to_set(args: &LispEval) -> Result<LispEval, EvalError> {
    let mut set = HashSet::new();

    if let LispEval::List(list) = args {
        for item in list {
            set.insert(item.clone());
        }
    }  
    Ok(LispEval::Set(set))
}

fn print(args: &LispEval) -> Result<LispEval, EvalError> {
    if let LispEval::List(args_vec) = args {
        if args_vec.len() > 1 {
            return Err(EvalError::InvalidNumberOfArguments ());
        }
        println!("{}", &args_vec[0]);
        Ok(args_vec[0].clone())
     }
     else {
         Err(EvalError::UnexpectedExpression())
     }
}


fn union(args: &LispEval) -> Result<LispEval, EvalError> {
    let mut result = HashSet::<LispEval>::new();
    if let LispEval::List(args_vec) = args {
        for item in args_vec {
            if let LispEval::Set(set) = item {
                result.extend(set.iter().cloned());
            }
        }
        Ok(LispEval::Set(result))
     }
     else {
         Err(EvalError::UnexpectedExpression())
     }
}

fn intersection(args: &LispEval) -> Result<LispEval, EvalError> {
    let mut result = HashSet::<LispEval>::new();
    if let LispEval::List(args_vec) = args {
        for (i, item) in args_vec.iter().enumerate() {
            if let LispEval::Set(set) = item {
                if i == 0 {
                    result = set.clone();
                } else {
                    result = result.intersection(&set).cloned().collect();
                }
            }
        }
        Ok(LispEval::Set(result))
     }
     else {
        Err(EvalError::UnexpectedExpression())
     }
}