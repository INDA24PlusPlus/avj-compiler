use crate::parser::{find_child_nodes, ASTNode, NodeType};
pub fn generate_qbe_code(ast: &Vec<ASTNode>) -> String {
    let mut stack = ast.clone();
    let mut code = String::from("");

    while stack.len() > 0 {
        let node = stack.first().unwrap().token.clone();
        match node {
            NodeType::VARIABLEASSIGNMENT(variable) => {
                let child_nodes = find_child_nodes(&stack, 0);
                // vi behöver veta vad variabeln heter och för värde den bör ha
                println!("{:?}", child_nodes.len());
                println!("{:?}", child_nodes);
            }
            NodeType::BINARYOPERATION(op) => {}
            _ => {}
        }
    }

    return code;
}
