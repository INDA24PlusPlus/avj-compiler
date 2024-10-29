use std::fmt::format;

use crate::{lexer::Comparison, parser::NodeType};

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
