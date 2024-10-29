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

        match node {
            NodeType::VARIABLEASSIGNMENT(variable) => {
                let child_nodes = find_child_nodes(&stack, ast_index);
                // vi behöver veta vad variabeln heter och för värde den bör ha
                println!("{:?}", child_nodes.len());
                println!("{:?}", child_nodes);
                // börja enkelt genom att anta att det är bara är ett värde i child_nodes, sen kan vi kolla på uttryck och sånt
                // lite hacky att skapa ny variabel genom att addera 0 till det, men det funkar för tillfället
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
            NodeType::PRINT => {}
            NodeType::BINARYOPERATION(op) => {}
            NodeType::IFSTATEMENT(comparison) => {
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

                let qbe_if_string = format!(
                    "\n jnz {} {} {}, @ifbody, @ifend \n @ifbody \n",
                    a, comparison_string, b
                );

                code.push_str(&qbe_if_string);
                for (index, child_node) in child_nodes {
                    if matches!(child_node.token, NodeType::VARIABLE(_))
                        || matches!(child_node.token, NodeType::VALUE(_))
                    {
                        stack.remove(index);
                    }
                }
                stack.remove(0);
                in_if_body = true;
            }
            NodeType::LOOP(iterator_variable, count) => {
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
            _ => {}
        }
        println!("{}", code);
    }

    return code;
}
