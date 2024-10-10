mod ast;
mod parser;
mod token;
mod tokenizer;
use parser::Parser;
use std::io::{stdin, stdout, Write};
use tokenizer::Tokenizer;

fn main() {
    print!("Enter an expression: ");
    stdout().flush().expect("Failed to flush stdout");
    let mut expr = String::new();
    stdin()
        .read_line(&mut expr)
        .expect("Failed to read user input");
    let expr = expr.trim();
    let mut tokenizer = Tokenizer::new(expr);
    let tokens = tokenizer.tokens();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    let result = ast.eval();
    println!("{expr} = {result}");
}
