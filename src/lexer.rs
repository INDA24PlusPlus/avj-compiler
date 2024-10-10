use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operation {
    ADD,
    MULTIPLY,
    SUBTRACT,
    DIVIDE,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Comparison {
    EQ,
    GT,
    LT,
    GEQ,
    LEQ,
}
#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
    IF,
    THEN,
    ELSE,
    LEFTBRACE,
    RIGHTBRACE,
    VARIABLE(String),
    VARIABLEASSIGN(String),
    LOOP,
    ASSIGNMENTOP,
    Comparison(Comparison),
    Operation(Operation),
    VALUE(i32),
    PRINT,
    EOL,
    LEFTPARENT,
    RIGHTPARENT,
}

pub fn tokenize(code: &str) -> Result<Vec<Symbol>, &str> {
    let mut variable_map: HashMap<String, i32> = HashMap::new();
    let mut tokens: Vec<Symbol> = vec![];
    for line in code.lines() {
        for characters in line.split_whitespace() {
            if characters == "+" {
                tokens.push(Symbol::Operation(Operation::ADD));
            } else if characters == "*" {
                tokens.push(Symbol::Operation(Operation::MULTIPLY));
            } else if characters == "-" {
                tokens.push(Symbol::Operation(Operation::SUBTRACT));
            } else if characters == "/" {
                tokens.push(Symbol::Operation(Operation::DIVIDE));
            } else if characters == "==" {
                tokens.push(Symbol::Comparison(Comparison::EQ));
            } else if characters == ">=" {
                tokens.push(Symbol::Comparison(Comparison::GEQ));
            } else if characters == "=<" {
                tokens.push(Symbol::Comparison(Comparison::LEQ));
            } else if characters == ">" {
                tokens.push(Symbol::Comparison(Comparison::GT));
            } else if characters == "<" {
                tokens.push(Symbol::Comparison(Comparison::LT));
            } else if characters == "=" {
                tokens.push(Symbol::ASSIGNMENTOP);
            } else if characters == "{" {
                tokens.push(Symbol::LEFTBRACE);
            } else if characters == "}" {
                tokens.push(Symbol::RIGHTBRACE);
            } else if characters == "annars" {
                tokens.push(Symbol::ELSE);
            } else if characters == "om" {
                tokens.push(Symbol::IF);
            } else if characters == "upprepa" {
                tokens.push(Symbol::LOOP);
            } else if characters.contains("@") {
                let variable_name = characters.replace("@", "");
                tokens.push(Symbol::VARIABLEASSIGN(variable_name.to_string()));
                variable_map.insert(variable_name, 0);
            } else if characters.contains("print") {
                tokens.push(Symbol::PRINT);
            } else if characters.contains("då") {
                tokens.push(Symbol::THEN);
            } else if characters.contains("(") {
                tokens.push(Symbol::LEFTPARENT);
            } else if characters.contains(")") {
                tokens.push(Symbol::RIGHTPARENT);
            } else if characters.chars().all(|x| x.is_digit(10)) {
                tokens.push(Symbol::VALUE(characters.parse::<i32>().unwrap()));
            } else if variable_map.get(characters).is_some() {
                tokens.push(Symbol::VARIABLE(characters.to_string()));
            } else {
                return Err("Unknown symbol");
            }
        }
        tokens.push(Symbol::EOL);
    }
    return Ok(tokens);
}
