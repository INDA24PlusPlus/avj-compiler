use lexer::{tokenize, Symbol};

pub mod lexer;
fn main() {
    let code = "@a = 1";
    let if_statement = "@a = 1 \n om a == 0 då { \n print 1 \n }";
    let tokens = tokenize(if_statement).unwrap();
    println!("{:?}", tokens);
}
