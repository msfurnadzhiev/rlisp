#[derive(Debug, PartialEq)]
pub enum ParseError {
  EmptyInput(),
  UnexpectedToken(String),
  UnexpectedExpression(),
  MissingToken,
}

#[derive(Debug)]
pub enum EvalError {
  DefaultError(),
  UnknownSymbol(String),
  UnexpectedExpression(),
  InvalidNumberOfArguments(),
  InvalidAgrumentType(),
  InvalidListExpression(),
  InvalidIfStatement(),
  InvalidFunctionCall,
  NonDefineInThisScope,
}

#[derive(Debug)]
pub enum ReaderError {
  UnexpectedSymbol(char),
  MissingSymbol(char)
}