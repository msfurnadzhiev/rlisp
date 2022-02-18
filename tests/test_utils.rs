use rlisp::utils::*;

#[test]
fn test_tokenization() {

    assert_eq!(tokenize("(head '(1 2 3))".to_string()), 
        ["(", "head", "'", "(", "1", "2", "3", ")", ")"]);

    assert_eq!(tokenize("(head {1 2 3})".to_string()), 
        ["(", "head", "{", "1", "2", "3", "}", ")"]);

    assert_eq!(tokenize("((lambda (arg) (+ arg 1)) 5)".to_string()), 
        ["(", "(", "lambda", "(", "arg", ")", "(", "+", "arg", "1", ")", ")", "5", ")"]);
}