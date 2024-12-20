use std::{collections::HashMap, ops::Deref};

use crate::lexer::{Comparison, Operation, Symbol};

#[derive(Debug, Clone)]
pub enum NodeType {
    VARIABLEASSIGNMENT(String),
    REASSIGNMENT(String),
    IFSTATEMENT(Comparison),
    BINARYOPERATION(Operation),
    VALUE(i32),
    VARIABLE(String),
    LOOP(String, String),
    LOOPBODY,
    IFBODY,
    PRINT,
    EOF,
}

impl std::fmt::Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeType::VARIABLEASSIGNMENT(var) => write!(f, "Variable Assignment: {}", var),
            NodeType::REASSIGNMENT(var) => write!(f, "Reassignment: {}", var),
            NodeType::IFSTATEMENT(comp) => write!(f, "If Statement: {:?}", comp),
            NodeType::BINARYOPERATION(op) => write!(f, "Binary Operation: {:?}", op),
            NodeType::VALUE(val) => write!(f, "Value: {}", val),
            NodeType::VARIABLE(var) => write!(f, "Variable: {}", var),
            NodeType::LOOP(var, count) => write!(f, "Loop: {} times with iterator {}", count, var),
            NodeType::LOOPBODY => write!(f, "Loop Body"),
            NodeType::IFBODY => write!(f, "If Body"),
            NodeType::PRINT => write!(f, "Print"),
            NodeType::EOF => write!(f, "EOF"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ASTNode {
    pub parent: Option<usize>,
    pub token: NodeType,
}

pub fn find_child_nodes(tree: &Vec<ASTNode>, parent_index: usize) -> Vec<(usize, ASTNode)> {
    let mut child_nodes: Vec<(usize, ASTNode)> = vec![];
    for (index, node) in tree.iter().enumerate() {
        if node.parent == Some(parent_index) {
            child_nodes.push((index, node.clone()));
        }
    }
    return child_nodes;
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

fn build_tree_from_tokens(tokens: Vec<Symbol>, starting_index: usize) -> Vec<ASTNode> {
    let mut tree: Vec<ASTNode> = vec![];
    let mut leaf_stack: Vec<usize> = vec![];

    for token in tokens.iter() {
        match token {
            Symbol::Operation(op) => {
                let right_idx = leaf_stack.pop().unwrap();
                let left_idx = leaf_stack.pop().unwrap();

                let operation_node = ASTNode {
                    parent: None,
                    token: NodeType::BINARYOPERATION(op.clone()),
                };

                tree.push(operation_node);
                let current_index = tree.len() - 1;

                // Update parents of operands
                tree[left_idx].parent = Some(current_index);
                tree[right_idx].parent = Some(current_index);

                leaf_stack.push(current_index);
            }
            Symbol::VALUE(value) => {
                let node = ASTNode {
                    parent: None,
                    token: NodeType::VALUE(value.clone()),
                };
                tree.push(node);
                leaf_stack.push(tree.len() - 1);
            }
            Symbol::VARIABLE(variable) => {
                let node = ASTNode {
                    parent: None,
                    token: NodeType::VARIABLE(variable.clone()),
                };
                tree.push(node);
                leaf_stack.push(tree.len() - 1);
            }
            _ => {}
        }
    }

    if starting_index > 0 {
        for node in tree.iter_mut() {
            if let Some(parent) = node.parent {
                node.parent = Some(parent + starting_index + 1);
            } else if node.parent.is_none() {
                node.parent = Some(starting_index);
            }
        }
    }

    return tree;
}

pub fn parse(tokens: Vec<Symbol>) -> Vec<ASTNode> {
    let mut tree: Vec<ASTNode> = vec![];
    // Not all tokens are really necessary for creating the AST
    let mut parent_node: Vec<usize> = vec![];
    let mut expression_end: Option<usize> = None;
    let mut tokens = tokens.clone();

    let temp_tokens: Option<Vec<Symbol>> = None;

    let mut index = 0;

    while index < tokens.len() {
        let token = &tokens[index];
        if let Symbol::VARIABLEASSIGN(value) = token {
            let node = ASTNode {
                parent: None,
                token: NodeType::VARIABLEASSIGNMENT(value.clone()),
            };
            tree.push(node);
            parent_node.push(index);
            // travel forward until we hit end of line
            for second_index in index..tokens.len() {
                if tokens[second_index] == Symbol::EOL {
                    expression_end = Some(second_index);
                    break;
                }
            }

            let expression_tokens = &tokens.clone()[index..=expression_end.unwrap()];
            let result = shunting_yard(expression_tokens.to_vec(), index);
            let mut t = build_tree_from_tokens(result, tree.len() - 1);

            // find root node and set variable assignment as parent
            let root_node = t.iter().find(|node| node.parent.is_none());
            if let Some(root) = root_node {
                let root_index = t.iter().position(|node| node.parent.is_none()).unwrap();
                t[root_index].parent = Some(tree.len() - 1);
            }

            index = expression_end.unwrap() + 1;

            tree.extend(t);
            expression_end = None;
            parent_node.pop();

            // remove all tokens between index and expression_end
        } else if let Symbol::VARIABLE(var) = token {
            if tokens[index + 1] == Symbol::ASSIGNMENTOP {
                let node = ASTNode {
                    parent: parent_node.last().cloned(),
                    token: NodeType::REASSIGNMENT(var.clone()),
                };
                tree.push(node);
                parent_node.push(index);

                for second_index in index..tokens.len() {
                    if tokens[second_index] == Symbol::EOL {
                        expression_end = Some(second_index);
                        break;
                    }
                }

                let expression_tokens = &tokens.clone()[(index + 1)..=expression_end.unwrap()];
                let result = shunting_yard(expression_tokens.to_vec(), index);
                let mut t = build_tree_from_tokens(result, tree.len() - 1);

                let root_node = t.iter().find(|node| node.parent.is_none());
                if let Some(root) = root_node {
                    let root_index = t.iter().position(|node| node.parent.is_none()).unwrap();
                    t[root_index].parent = Some(tree.len() - 1);
                }

                index = expression_end.unwrap() + 1;

                tree.extend(t);
                expression_end = None;
                parent_node.pop();
            }
        } else if Symbol::LOOP == *token {
            // first travel forward until we hit first right brace
            for second_index in index..tokens.len() {
                if tokens[second_index] == Symbol::LEFTBRACE {
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
                parent: parent_node.last().cloned(),
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
            parent_node.push(tree.len() - 1);
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
                parent: parent_node.last().cloned(),
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
            parent_node.push(tree.len() - 1);
        } else if let Symbol::PRINT = *token {
            let node = ASTNode {
                parent: parent_node.last().cloned(),
                token: NodeType::PRINT,
            };

            // parse value / variable node
            let printed_value = &tokens.clone()[index + 1];

            if !matches!(printed_value, Symbol::VALUE(_))
                && !matches!(printed_value, Symbol::VARIABLE(_))
            {
                panic!("Expected value or variable but got: {:?}", printed_value)
            }

            tree.push(node);
            if let Symbol::VARIABLE(variable) = printed_value {
                let node = ASTNode {
                    parent: Some(tree.len() - 1),
                    token: NodeType::VARIABLE(variable.clone()),
                };
                tree.push(node);
            } else if let Symbol::VALUE(value) = printed_value {
                let node = ASTNode {
                    parent: Some(tree.len() - 1),
                    token: NodeType::VALUE(value.clone()),
                };
                tree.push(node);
            }

            index += 2;
        } else if let Symbol::RIGHTBRACE = *token {
            // we want everything in between this right brace and the previous left brace
            // then we want to rerun this function or while for that entire code block

            index += 1;
            parent_node.pop();
        } else {
            index += 1;
        }
    }

    let eof_node = ASTNode {
        parent: None,
        token: NodeType::EOF,
    };
    tree.push(eof_node);

    return tree;
}

pub fn draw_tree(tree: Vec<ASTNode>) {
    let mut tree_map: HashMap<Option<usize>, Vec<ASTNode>> = HashMap::new();
    for node in tree.iter() {
        tree_map
            .entry(node.parent)
            .or_insert(vec![])
            .push(node.clone());
    }
    // loop over the tree vec again and print the children of each node
    for (key, value) in tree_map.iter() {
        let parent = key.unwrap_or(0);
        let parent_string = if parent == 0 && key.is_none() {
            "ROOT"
        } else {
            &tree[parent].token.to_string()
        };
        println!("Parent: {:?}", parent_string);
        for child in value.iter() {
            print!("    Child: {:?}     ", child.token);
        }
        println!("");
    }
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
