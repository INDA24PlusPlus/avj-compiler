use crate::{
    lexer::Symbol,
    lib::{
        helpers::{comparison_to_qbe, format_values_to_qbe},
        template::loop_template,
    },
    parser::{find_child_nodes, ASTNode, NodeType},
};
pub fn generate_qbe_code(ast: &Vec<ASTNode>) -> String {
    let mut stack = ast.clone();

    let mut code = String::from("");

    let mut in_if_body = false;

    while stack.len() > 0 {
        let node = stack.first().unwrap().token.clone();
        let ast_index = ast.len() - stack.len();
        println!("{:?}", node);

        match node {
            NodeType::VARIABLEASSIGNMENT(variable) => {
                if in_if_body && stack.first().unwrap().parent.is_none() {
                    code.push_str("\n @ifend \n");
                    in_if_body = false;
                }
                let child_nodes = find_child_nodes(&stack, ast_index);
                println!("AST index: {}", ast_index);
                // vi behöver veta vad variabeln heter och för värde den bör ha
                println!("{:?}", child_nodes.len());
                println!("{:?}", child_nodes);
                // börja enkelt genom att anta att det är bara är ett värde i child_nodes, sen kan vi kolla på uttryck och sånt
                let value = child_nodes.first().unwrap().1.token.clone();
                match value {
                    NodeType::VALUE(val) => {
                        code.push_str(&format!("%{} =w copy {}\n", variable, val));
                        // ta hand om uttryck senare
                    }
                    _ => {}
                }
                for (index, _) in child_nodes {
                    stack.remove(index);
                }
                stack.remove(0);
            }
            NodeType::PRINT => {
                if in_if_body && stack.first().unwrap().parent.is_none() {
                    code.push_str("\n @ifend \n");
                    in_if_body = false;
                }
                let child_nodes = find_child_nodes(&stack, ast_index);
                // eftersom vi endast kan printa ett värde sparar vi det som en data variabel och sen printar ut det
                if child_nodes.is_empty() {
                    panic!("Print statement contains nothing")
                }
                let value_node = child_nodes.first().unwrap().clone().1;

                let fmt_string = match value_node.token {
                    NodeType::VALUE(value) => {
                        format!("data $fmt = {{ b \"{}\", b 0 }}", value)
                    }
                    NodeType::VARIABLE(variable) => {
                        format!("data $fmt = {{ b \"{}\", b 0 }}", variable)
                    }
                    _ => String::from(""),
                };

                code.push_str(&fmt_string);
                stack.remove(0);
                stack.remove(0);
                // also want to remove all child nodes
            }
            NodeType::BINARYOPERATION(op) => {}
            NodeType::IFSTATEMENT(comparison) => {
                if in_if_body && stack.first().unwrap().parent.is_none() {
                    code.push_str("\n @ifend \n");
                    in_if_body = false;
                }
                let comparison_string = comparison_to_qbe(comparison);
                let child_nodes = find_child_nodes(&stack, ast_index);
                // kolla två children som antingen är variable eller value
                let statement_parameters: Vec<&ASTNode> = child_nodes
                    .iter()
                    .filter(|(_, node)| {
                        matches!(node.token, NodeType::VARIABLE(_))
                            || matches!(node.token, NodeType::VALUE(_))
                    })
                    .map(|(_, node)| node)
                    .collect();

                if statement_parameters.len() < 2 {
                    panic!("Error here")
                }

                let a = format_values_to_qbe(statement_parameters[0].clone().token);
                let b = format_values_to_qbe(statement_parameters[1].clone().token);

                println!("{} {}", a, b);

                let qbe_if_string =
                    format!("\n jnz {} {} {}, @ifbody, @ifend", a, comparison_string, b);

                code.push_str(&qbe_if_string);
                // Make sure removal is correct
                // Collect indices to remove first, then remove from largest to smallest
                let mut indices_to_remove: Vec<usize> = child_nodes
                    .iter()
                    .filter(|(_, node)| {
                        matches!(node.token, NodeType::VARIABLE(_))
                            || matches!(node.token, NodeType::VALUE(_))
                    })
                    .map(|(index, _)| *index)
                    .collect();

                // Sort in descending order so we remove from end first
                indices_to_remove.sort_by(|a, b| b.cmp(a));

                // Remove the nodes starting from highest index
                for index in indices_to_remove {
                    stack.remove(index);
                }
                stack.remove(0);
            }
            NodeType::LOOP(iterator_variable, count) => {
                if in_if_body && stack.first().unwrap().parent.is_none() {
                    code.push_str("\n @ifend \n");
                    in_if_body = false;
                }
                // initializera iterator_variable till 0
                let variable_initialization_statement =
                    &format!("%{} =w copy 0", iterator_variable);

                let mut loop_body_index: Option<usize> = None;

                for (index, child_node) in find_child_nodes(&stack, ast_index).iter().enumerate() {
                    if matches!(child_node.1.token, NodeType::LOOPBODY) {
                        loop_body_index = Some(index);
                    }
                }

                let loop_body_statements = find_child_nodes(&stack, loop_body_index.unwrap());

                // skicka in själva loop body koden också
                let loop_definition_instructions = loop_template(iterator_variable, count);
            }
            NodeType::IFBODY => {
                code.push_str("\n @ifbody \n");
                in_if_body = true;
                stack.remove(0);
            }
            _ => {}
        }
        println!("{}", code);
    }

    return code;
}
