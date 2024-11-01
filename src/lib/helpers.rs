use std::fmt::format;

use crate::{
    lexer::Comparison,
    parser::{find_child_nodes, ASTNode, NodeType},
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

pub fn extract_expression_from_tree(
    tree: Vec<ASTNode>,
    starting_index: usize,
) -> Vec<(usize, ASTNode)> {
    let mut expression_nodes = Vec::new();
    for (index, node) in tree.iter().enumerate() {
        if index > starting_index {
            println!("Node: {:?}", node);
            if !matches!(
                node.token,
                NodeType::BINARYOPERATION(_) | NodeType::VALUE(_) | NodeType::VARIABLE(_)
            ) {
                break;
            } else {
                expression_nodes.push((index, node.clone()));
            }
        }
    }

    expression_nodes
}

pub fn expression_to_qbe(expression: Vec<ASTNode>, starting_index: usize) -> (String, usize) {
    let mut qbe = String::new();

    let mut temp_counter = 0;
    let mut stack: Vec<String> = Vec::new();

    let mut get_next_temp = || {
        temp_counter += 1;
        format!("%t{}", temp_counter)
    };

    fn post_order(node: &ASTNode, nodes: &Vec<ASTNode>, starting_index: usize) -> Vec<usize> {
        let mut result = Vec::new();

        let node_index = nodes.iter().position(|n| std::ptr::eq(n, node)).unwrap();
        let children: Vec<&ASTNode> = nodes
            .iter()
            .filter(|n| n.parent == Some(node_index + starting_index + 1))
            .collect();

        for child in children {
            result.extend(post_order(child, nodes, starting_index));
        }
        result.push(node_index);
        result
    }

    let root = expression
        .iter()
        .find(|node| node.parent.is_none() || node.parent == Some(starting_index))
        .unwrap();
    let traversal = post_order(root, &expression, starting_index);

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
