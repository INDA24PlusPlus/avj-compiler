use crate::{
    lexer::Symbol,
    lib::{
        helpers::{
            assignment_to_qbe, comparison_to_qbe, expression_to_qbe, extract_expression_from_tree,
            format_values_to_qbe,
        },
        template::loop_template,
    },
    parser::{draw_tree, find_child_nodes, ASTNode, NodeType},
};

#[derive(Clone)]
struct LoopInfo {
    iterator_variable: String,
    count: String,
}

fn remove_children(child_nodes: Vec<(usize, ASTNode)>, stack: &mut Vec<ASTNode>) {
    let mut indices_to_remove: Vec<usize> = child_nodes
        .iter()
        .filter(|(_, node)| {
            matches!(node.token, NodeType::VARIABLE(_))
                || matches!(node.token, NodeType::VALUE(_))
                || matches!(node.token, NodeType::BINARYOPERATION(_))
        })
        .map(|(index, _)| *index)
        .collect();

    // Sort in descending order so we remove from end first
    indices_to_remove.sort_by(|a, b| b.cmp(a));

    // Remove the nodes starting from highest index
    for index in indices_to_remove {
        stack.remove(index);
    }
}

pub fn generate_qbe_code(ast: &Vec<ASTNode>) -> String {
    let mut stack = ast.clone();

    let mut code = String::from("");

    let mut in_if_body = false;

    let mut loop_info: Option<LoopInfo> = None;

    let mut index = 0;

    let mut loop_counter = 0;
    let mut if_counter = 0;

    let mut print_counter = 0;
    while stack.len() > 0 {
        let node = stack.first().unwrap().token.clone();
        let ast_index = ast.len() - stack.len();
        if in_if_body && (stack.first().unwrap().parent.is_none() || matches!(node, NodeType::EOF))
        {
            code.push_str(&format!("\n @ifend_{} \n", if_counter));
            in_if_body = false;
            if_counter += 1;
        }
        if loop_info.is_some()
            && (stack.first().unwrap().parent.is_none() || matches!(node, NodeType::EOF))
        {
            if let Some(l) = &loop_info {
                let loop_definition_instructions = loop_template(
                    l.clone().iterator_variable,
                    l.clone().count,
                    loop_counter.to_string(),
                );
                code.push_str(&loop_definition_instructions);
                loop_info = None;
            }
        }

        match node {
            NodeType::VARIABLEASSIGNMENT(variable) => {
                let child_nodes = find_child_nodes(&stack, ast_index);
                // vi behöver veta vad variabeln heter och för värde den bör ha
                // börja enkelt genom att anta att det är bara är ett värde i child_nodes, sen kan vi kolla på uttryck och sånt
                let value = child_nodes.first().unwrap().1.token.clone();
                match value {
                    NodeType::VALUE(val) => {
                        code.push_str(&format!("%{} =w copy {}\n", variable, val));
                        // ta hand om uttryck senare
                    }
                    _ => {}
                }
                remove_children(child_nodes.clone(), &mut stack);
                stack.remove(0);
                index += child_nodes.len() + 1;
            }
            NodeType::REASSIGNMENT(variable) => {
                let expression_nodes: Vec<(usize, ASTNode)> =
                    extract_expression_from_tree(ast.clone(), ast_index);
                let child_nodes: Vec<(usize, ASTNode)> = expression_nodes
                    .iter()
                    .map(|(index, node)| (index.clone() - ast_index, node.clone()))
                    .collect();
                if child_nodes.len() == 1 {
                    let value = child_nodes.first().unwrap().1.token.clone();
                    match value {
                        NodeType::VALUE(val) => {
                            code.push_str(&format!("%{} =w copy {}\n", variable, val));
                            // ta hand om uttryck senare
                        }
                        NodeType::VARIABLE(var) => {
                            code.push_str(&format!("%{} =w add %{}, 0\n", variable, var));
                        }
                        _ => {}
                    }
                } else {
                    let qbe_instructions = assignment_to_qbe(
                        variable,
                        expression_nodes
                            .iter()
                            .map(|(_, node)| node.clone())
                            .collect(),
                        ast_index,
                    );

                    code.push_str(&qbe_instructions);
                }

                remove_children(child_nodes.clone(), &mut stack);
                stack.remove(0);
                index += child_nodes.len() + 1;
            }
            NodeType::PRINT => {
                let child_nodes = find_child_nodes(&stack, ast_index);
                // eftersom vi endast kan printa ett värde sparar vi det som en data variabel och sen printar ut det
                if child_nodes.is_empty() {
                    panic!("Print statement contains nothing")
                }
                let value_node = child_nodes.first().unwrap().clone().1;

                let fmt_string = match value_node.token {
                    NodeType::VALUE(value) => {
                        format!("\n call $printf(l $fmt, ..., w {})", value)
                    }
                    NodeType::VARIABLE(variable) => {
                        format!("\n call $printf(l $fmt, ..., w %{})", variable)
                    }
                    _ => String::from(""),
                };

                code.push_str(&fmt_string);
                stack.remove(0);
                stack.remove(0);

                index += 2;
                print_counter += 1;
            }
            NodeType::IFSTATEMENT(comparison) => {
                let comparison_string = comparison_to_qbe(comparison);
                let child_nodes = find_child_nodes(&stack, ast_index);
                // kolla två children som antingen är variable eller value
                let statement_parameters: Vec<(usize, ASTNode)> = child_nodes
                    .iter()
                    .filter(|(_, node)| {
                        matches!(node.token, NodeType::VARIABLE(_))
                            || matches!(node.token, NodeType::VALUE(_))
                    })
                    .map(|(index, node)| (index.clone(), node.clone()))
                    .collect();

                if statement_parameters.len() < 2 {
                    println!("Index: {:?}", ast_index);
                    panic!("Error here")
                }

                let a = format_values_to_qbe(statement_parameters[0].clone().1.token);
                let b = format_values_to_qbe(statement_parameters[1].clone().1.token);

                let variable_comparison =
                    format!("%a_{} =w {} {}, {}", if_counter, comparison_string, a, b);

                let qbe_if_string = format!(
                    "{} \n jnz %a_{}, @ifbody_{}, @ifend_{}",
                    variable_comparison, if_counter, if_counter, if_counter
                );

                code.push_str(&qbe_if_string);
                // Make sure removal is correct
                // Collect indices to remove first, then remove from largest to smallest
                remove_children(
                    Vec::from([
                        statement_parameters[0].clone(),
                        statement_parameters[1].clone(),
                    ]),
                    &mut stack,
                );
                stack.remove(0);

                index += &child_nodes.len() + 1;
            }
            NodeType::LOOPBODY => {
                code.push_str("\n @loop \n");
                stack.remove(0);
            }
            NodeType::LOOP(iterator_variable, count) => {
                // initializera iterator_variable till 0
                let variable_initialization_statement =
                    &format!("%{} =w copy 0", iterator_variable);

                code.push_str(&variable_initialization_statement);

                loop_info = Some(LoopInfo {
                    iterator_variable,
                    count,
                });
                stack.remove(0);
                index += 1;
                loop_counter += 1;
            }
            NodeType::IFBODY => {
                code.push_str(&format!("\n @ifbody_{}", if_counter));
                in_if_body = true;
                stack.remove(0);
                index += 1;
            }
            NodeType::EOF => {
                stack.remove(0);
                index += 1;
            }
            _ => {}
        }
    }

    code.push_str("\n data $fmt = {{ b \"%d\", b 0 }}");
    println!("Code start \n {} \n Code end", code);
    return code;
}
