use lexer::{tokenize, Symbol};
use parser::{draw_tree, parse, shunting_yard};
pub mod lexer;
pub mod parser;
fn main() {
    let code = "@a = 3 \n upprepa a @i { \n print i \n }";
    let if_code = "@a = 1 \n om a == 0 { \n print 1 \n }";
    let tokens = tokenize(if_code).unwrap();
    println!("{:?}", tokens);
    let ast = parse(tokens.clone());
    let if_tokens = tokenize(if_code).unwrap();
    let if_ast = parse(if_tokens);
    /* let if_statement = "@a = 1 \n om a == 0 då { \n print 1 \n }";
    let tokens = tokenize(if_statement).unwrap();
    let ast = parse(tokens); */
    draw_tree(ast);
}
