use std::ops::Deref;

use crate::lexer::{Comparison, Operation, Symbol};

#[derive(Debug, Clone)]
enum NodeType {
    VARIABLEASSIGNMENT(String),
    IFSTATEMENT(Comparison),
    BINARYOPERATION(Operation),
    VALUE(i32),
    VARIABLE(String),
    LOOP(String, String),
    LOOPBODY,
    IFBODY,
}

#[derive(Debug, Clone)]
pub struct ASTNode {
    parent: Option<usize>,
    token: NodeType,
}

fn operator_presedence(op: Operation) -> Option<i32> {
    if op == Operation::ADD || op == Operation::SUBTRACT {
        return Some(1);
    } else if op == Operation::MULTIPLY || op == Operation::DIVIDE {
        return Some(2);
    }
    return None;
}

pub fn shunting_yard(tokens: Vec<Symbol>, parent_index: usize) -> Vec<Symbol> {
    let mut operator_stack = vec![];
    let mut output_queue: Vec<Symbol> = vec![];

    for token in tokens.iter() {
        match token {
            Symbol::VALUE(_) | Symbol::VARIABLE(_) => output_queue.push(token.clone()),
            Symbol::Operation(op) => {
                while let Some(Symbol::Operation(top_op)) = operator_stack.last() {
                    if operator_presedence(*top_op) >= operator_presedence(*op) {
                        output_queue.push(operator_stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                operator_stack.push(token.clone());
            }
            Symbol::LEFTPARENT => operator_stack.push(token.clone()),
            Symbol::RIGHTPARENT => {
                while let Some(top) = operator_stack.pop() {
                    if top == Symbol::LEFTPARENT {
                        break;
                    }
                    output_queue.push(top);
                }
            }
            _ => {}
        }
    }

    while let Some(op) = operator_stack.pop() {
        output_queue.push(op);
    }

    return output_queue;
}

fn build_tree_from_tokens(tokens: Vec<Symbol>) -> Vec<ASTNode> {
    let mut tree: Vec<ASTNode> = vec![];
    let mut leaf_stack: Vec<ASTNode> = vec![];
    for token in tokens.iter() {
        if let Symbol::Operation(op) = token {
            let mut left = leaf_stack.pop().unwrap();
            let mut right = leaf_stack.pop().unwrap();
            let node = ASTNode {
                parent: None,
                token: NodeType::BINARYOPERATION(op.clone()),
            };
            left.parent = Some(tree.len() + 2);
            right.parent = Some(tree.len() + 2);
            tree.push(left);
            tree.push(right);
            leaf_stack.push(node);
        } else if let Symbol::VALUE(value) = token {
            let node = ASTNode {
                parent: None,
                token: NodeType::VALUE(value.clone()),
            };
            leaf_stack.push(node);
        } else if let Symbol::VARIABLE(variable) = token {
            let node = ASTNode {
                parent: None,
                token: NodeType::VARIABLE(variable.clone()),
            };
            leaf_stack.push(node);
        }
    }

    tree.push(leaf_stack.pop().unwrap());
    return tree;
}

pub fn parse(tokens: Vec<Symbol>) -> Vec<ASTNode> {
    let mut tree: Vec<ASTNode> = vec![];
    // Not all tokens are really necessary for creating the AST
    let mut parent_node: Option<usize> = None;
    let mut expression_end: Option<usize> = None;
    let mut tokens = tokens.clone();

    let mut index = 0;

    while index < tokens.len() {
        let token = &tokens[index];
        if let Symbol::VARIABLEASSIGN(value) = token {
            let node = ASTNode {
                parent: None,
                token: NodeType::VARIABLEASSIGNMENT(value.clone()),
            };
            tree.push(node);
            parent_node = Some(index);
            // travel forward until we hit end of line
            for second_index in index..tokens.len() {
                if tokens[second_index] == Symbol::EOL {
                    expression_end = Some(second_index);
                    break;
                }
            }

            let expression_tokens = &tokens.clone()[index..=expression_end.unwrap()];
            let result = shunting_yard(expression_tokens.to_vec(), index);
            let mut t = build_tree_from_tokens(result);

            // find root node and set variable assignment as parent
            let root_node = t.iter().find(|node| node.parent.is_none());
            if let Some(root) = root_node {
                let root_index = t.iter().position(|node| node.parent.is_none()).unwrap();
                t[root_index].parent = Some(index);
            }

            index = expression_end.unwrap() + 1;

            tree.extend(t);
            expression_end = None;
            parent_node = None;

            // remove all tokens between index and expression_end
        } else if Symbol::LOOP == *token {
            // first travel forward until we hit first right brace
            for second_index in index..tokens.len() {
                if tokens[second_index] == Symbol::RIGHTBRACE {
                    expression_end = Some(second_index);
                    break;
                }
            }

            // everything in between index and expression_end is the loop definition
            let loop_tokens = &tokens.clone()[index..expression_end.unwrap()];

            let mut loop_iterations: String = String::from("");
            let mut loop_variable: String = String::from("");
            for token in loop_tokens.iter() {
                if let Symbol::VALUE(value) = token {
                    loop_iterations = value.to_string();
                } else if let Symbol::VARIABLE(variable) = token {
                    loop_iterations = variable.clone();
                } else if let Symbol::VARIABLEASSIGN(variable) = token {
                    loop_variable = variable.clone();
                }
            }

            let node = ASTNode {
                parent: None,
                token: NodeType::LOOP(loop_iterations, loop_variable),
            };
            tree.push(node);

            let loop_body_node = ASTNode {
                parent: Some(tree.len() - 1),
                token: NodeType::LOOPBODY,
            };
            tree.push(loop_body_node);

            index = expression_end.unwrap() + 1;
            expression_end = None;
        } else if Symbol::IF == tokens[index] {
            // find first left brace
            for second_index in index..tokens.len() {
                if tokens[second_index] == Symbol::LEFTBRACE {
                    expression_end = Some(second_index);
                    break;
                }
            }

            let if_tokens = &tokens[index..expression_end.unwrap()];
            let mut comparison_operator = None;
            for token in if_tokens.iter() {
                if let Symbol::Comparison(op) = token {
                    comparison_operator = Some(op.clone());
                    break;
                }
            }

            let if_index = tree.len();
            let node = ASTNode {
                parent: None,
                token: NodeType::IFSTATEMENT(comparison_operator.unwrap()),
            };
            tree.push(node);

            // want to add each side of the if operator comparison to the tree
            for token in if_tokens.iter() {
                if let Symbol::VARIABLE(variable) = token {
                    let node = ASTNode {
                        parent: Some(if_index),
                        token: NodeType::VARIABLE(variable.clone()),
                    };
                    tree.push(node);
                } else if let Symbol::VALUE(value) = token {
                    let node = ASTNode {
                        parent: Some(if_index),
                        token: NodeType::VALUE(value.clone()),
                    };
                    tree.push(node);
                }
            }

            let if_body_node = ASTNode {
                parent: Some(if_index),
                token: NodeType::IFBODY,
            };
            tree.push(if_body_node);
            index = expression_end.unwrap() + 1;
            expression_end = None;
        } else if let Symbol::RIGHTBRACE = *token {
            // find all the
            index += 1;
        } else {
            index += 1;
        }
    }

    println!("Tree: {:?}", tree);
    return tree;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::{Operation, Symbol};

    #[test]
    fn test_shunting_yard() {
        let tokens = vec![
            Symbol::VALUE(1),
            Symbol::Operation(Operation::ADD),
            Symbol::VALUE(2),
            Symbol::Operation(Operation::MULTIPLY),
            Symbol::VALUE(3),
        ];
        let result = shunting_yard(tokens, 0);
        assert_eq!(
            result,
            vec![
                Symbol::VALUE(1),
                Symbol::VALUE(2),
                Symbol::VALUE(3),
                Symbol::Operation(Operation::MULTIPLY),
                Symbol::Operation(Operation::ADD),
            ]
        );
    }

    #[test]
    fn test_shunting_yard_parentheses() {
        let tokens = vec![
            Symbol::LEFTPARENT,
            Symbol::VALUE(1),
            Symbol::Operation(Operation::ADD),
            Symbol::VALUE(2),
            Symbol::RIGHTPARENT,
            Symbol::Operation(Operation::MULTIPLY),
            Symbol::VALUE(3),
        ];
        let result = shunting_yard(tokens, 0);
        assert_eq!(
            result,
            vec![
                Symbol::VALUE(1),
                Symbol::VALUE(2),
                Symbol::Operation(Operation::ADD),
                Symbol::VALUE(3),
                Symbol::Operation(Operation::MULTIPLY)
            ]
        );
    }
}
