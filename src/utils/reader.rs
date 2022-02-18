use crate::eval::LispEval;
use crate::utils::exceptions::ReaderError;

use std::io::BufRead;

pub fn from_reader<B: BufRead>(reader: B) -> Result<Vec<String>, ReaderError> {

    let mut end_term = 0;

    let mut terms = Vec::new();
    let mut term = String::new();

    for line in reader.lines() {
        for c in line.expect("lines failed").chars() {
            term.push(c);
            if c == '(' {
                end_term += 1;
            }
            else if c == ')' {
                end_term -= 1;
                if end_term < 0 {
                    return Err(ReaderError::UnexpectedSymbol(')'));
                }
            }
            if end_term == 0 {
                terms.push(term);
                term = "".to_string();
            }
        }
    }

    if end_term > 0 {
        return Err(ReaderError::MissingSymbol(')'));
    }

    Ok(terms)
}