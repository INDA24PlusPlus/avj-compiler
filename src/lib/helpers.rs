use std::fmt::format;

use crate::{
    lexer::Comparison,
    parser::{ASTNode, NodeType},
};

pub fn comparison_to_qbe(comp: Comparison) -> &'static str {
    match comp {
        Comparison::EQ => "ceq",
        Comparison::GEQ => "csge",
        Comparison::GT => "csgt",
        Comparison::LEQ => "csle",
        Comparison::LT => "cslt",
        Comparison::NEQ => "cne",
    }
}

pub fn format_values_to_qbe(value: NodeType) -> String {
    match value {
        NodeType::VALUE(val) => val.to_string(),
        NodeType::VARIABLE(var_name) => format!("%{}", var_name),
        _ => String::new(),
    }
}

pub fn assignment_to_qbe(
    variable: String,
    expression: Vec<ASTNode>,
    starting_index: usize,
) -> String {
    let (qbe_expression, variable_count) = expression_to_qbe(expression, starting_index);
    format!(
        "%{} \n %{} =w copy {}",
        qbe_expression, variable, variable_count
    )
}

pub fn expression_to_qbe(expression: Vec<ASTNode>, starting_index: usize) -> (String, usize) {
    // we want to take the tree and convert it into a list of qbe instructions
    // the operations at the bottom of the tree should be executed first
    // we want to traverse the tree in a post-order manner
    // we want to convert each node into a qbe instruction
    // we want to return a list of qbe instructions

    let mut qbe = String::new();

    let mut temp_counter = 0;
    let mut stack: Vec<String> = Vec::new();

    // Helper function to get next temporary variable name
    let mut get_next_temp = || {
        temp_counter += 1;
        format!("%t{}", temp_counter)
    };

    // Post-order traversal
    fn post_order(node: &ASTNode, nodes: &Vec<ASTNode>) -> Vec<usize> {
        let mut result = Vec::new();

        // Get child indices by finding nodes that have this node's index as parent
        let node_index = nodes.iter().position(|n| std::ptr::eq(n, node)).unwrap();
        let children: Vec<&ASTNode> = nodes
            .iter()
            .filter(|n| n.parent == Some(node_index))
            .collect();

        for child in children {
            result.extend(post_order(child, nodes));
        }
        result.push(node_index);
        result
    }

    // Get root node (node with no parent)
    let root = expression
        .iter()
        .find(|node| node.parent.is_none() || node.parent == Some(starting_index))
        .unwrap();
    let traversal = post_order(root, &expression);
    println!("Traversal: {:?}, Expression: {:?}", traversal, expression);

    // Process nodes in post-order
    for index in traversal {
        let node = &expression[index];
        match &node.token {
            NodeType::VALUE(val) => {
                stack.push(val.to_string());
            }
            NodeType::VARIABLE(var) => {
                stack.push(format!("%{}", var));
            }
            NodeType::BINARYOPERATION(op) => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                let temp = get_next_temp();

                qbe.push_str(&format!(
                    "    {} =w {} {}, {}\n",
                    temp,
                    op.to_string().to_lowercase(),
                    left,
                    right
                ));
                stack.push(temp);
            }
            _ => {}
        }
    }

    (qbe, temp_counter)
}
