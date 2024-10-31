use codegen::generate_qbe_code;
use lexer::{tokenize, Operation, Symbol};
use lib::helpers::expression_to_qbe;
use parser::{draw_tree, parse, shunting_yard, ASTNode, NodeType};
pub mod codegen;
pub mod lexer;
pub mod lib;
pub mod parser;
pub mod semantics;
fn main() {
    let code = "@a = 3 \n upprepa a @i { \n print i \n }";
    let if_code = "@a = 1 \n om a == 0 { \n print 1 \n }";
    let fib = "@n = 3
om n == 0 {
    print 1
}

om n == 1 {
    print 3
}

@initial = 1
@second = 3
@result = 0

upprepa n @i {
    result = 3 * second - initial
    initial = second
    second = result
}

print result";
    let tokens = tokenize(fib).unwrap();
    let ast = parse(tokens.clone());
    /* let if_statement = "@a = 1 \n om a == 0 då { \n print 1 \n }";
    let tokens = tokenize(if_statement).unwrap();
    let ast = parse(tokens); */

    //draw_tree(ast.clone());
    generate_qbe_code(&ast);
}
