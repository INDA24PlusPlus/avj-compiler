use std::ops::Deref;

use crate::{lexer::Symbol, parser::NodeType};

fn validate_expression(tokens: &Vec<Symbol>) -> bool {
    if !matches!(tokens[0], Symbol::VARIABLE(_)) || !matches!(tokens[0], Symbol::VALUE(_)) {
        return false;
    }
    let mut prev_symbol: Option<Symbol> = None;
    for token in tokens.iter() {
        if prev_symbol.is_none() {
            prev_symbol = Some(token.clone());
        } else if prev_symbol.is_some() {
            let previous_symbol = prev_symbol.clone().unwrap();
            if matches!(token, Symbol::Operation(_))
                && matches!(previous_symbol, Symbol::Operation(_))
            {
                return false;
            } else if matches!(token, Symbol::VALUE(_))
                && !matches!(previous_symbol, Symbol::VALUE(_))
            {
                return false;
            } else if matches!(token, Symbol::VARIABLE(_))
                && !matches!(previous_symbol, Symbol::VARIABLE(_))
            {
                return false;
            }
            // also want to check if variable and value are besides each other
            else if matches!(token, Symbol::VALUE(_))
                && matches!(previous_symbol, Symbol::VARIABLE(_))
            {
                return false;
            } else if matches!(token, Symbol::VARIABLE(_))
                && matches!(previous_symbol, Symbol::VALUE(_))
            {
                return false;
            }
        }
    }
    return true;
}

// We basically want to check the ordering of keywords and tokens
// In this case that is what matters
// TODO: remove panics and just return an error
// TODO: not really checking that much right now but will add some more checks later
fn analyze_semantics(statement_type: &NodeType, tokens: &Vec<Symbol>) {
    if let NodeType::VARIABLEASSIGNMENT(variable) = statement_type {
        for index in 0..3 {
            let token_type = tokens[index as usize].clone();
            if index == 0 && !matches!(token_type, Symbol::VARIABLEASSIGN(_)) {
                panic!("Expected variable assignment at the start of the statement");
            } else if index == 1 && !matches!(token_type, Symbol::VARIABLE(_)) {
                panic!("Expected variable at the start of the statement");
            } else if index == 2 && !matches!(token_type, Symbol::ASSIGNMENTOP) {
                panic!("Expected assignment operator at the start of the statement");
            }
        }
        if !validate_expression(&tokens[2..].to_vec()) {
            panic!("Invalid expression");
        }
    } else if let NodeType::IFSTATEMENT(_) = statement_type {
        for (index, token) in tokens.iter().enumerate() {
            if index == 0 && !matches!(token, Symbol::IF) {
                panic!("Expected if at the start of the statement");
            } else if (index == 1 || index == 3)
                && (!matches!(token, Symbol::VARIABLE(_)) || !matches!(token, Symbol::VALUE(_)))
            {
                panic!("Expected variable or value after if");
            } else if index == 2 && !matches!(token, Symbol::Comparison(_)) {
                panic!("Expected comparison operator after variable or value");
            } else if index == 4 && !matches!(token, Symbol::RIGHTBRACE) {
                panic!("Expected right brace after comparison operator");
            }
        }
    } else if let NodeType::LOOP(_, _) = statement_type {
        for (index, token) in tokens.iter().enumerate() {
            if index == 0 && !matches!(token, Symbol::LOOP) {
                panic!("Expected loop at the start of the statement");
            } else if index == 1 && !matches!(token, Symbol::VARIABLEASSIGN(_)) {
                panic!("Expected variable assignment at the start of the statement");
            } else if index == 2 && !matches!(token, Symbol::VARIABLE(_)) {
                panic!("Expected variable at the start of the statement");
            } else if index == 3 && !matches!(token, Symbol::RIGHTBRACE) {
                panic!("Expected right brace after variable");
            }
        }
    }
}
