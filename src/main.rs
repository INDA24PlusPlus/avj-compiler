use lexer::{tokenize, Symbol};
use parser::{parse, shunting_yard};
pub mod lexer;
pub mod parser;
fn main() {
    let code = "@a = 1 + 2 * 3";
    let tokens = tokenize(code).unwrap();
    let ast = parse(tokens.clone());
    /* let if_statement = "@a = 1 \n om a == 0 då { \n print 1 \n }";
    let tokens = tokenize(if_statement).unwrap();
    let ast = parse(tokens); */

    shunting_yard(tokens, 0);
}
