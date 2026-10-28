use std::{any, collections::HashMap, hash::Hash};

use crate::runtime;

pub struct SymbolTable {
    pub scope_stack: Vec<HashMap<String,Symbol>>
}

impl SymbolTable {
    pub fn new(scope_stack: Vec<HashMap<String,Symbol>>) -> Self {
        Self {
            scope_stack
        }
    }

    pub fn add_scope(&mut self, scope: HashMap<String,Symbol>) -> HashMap<String, Symbol> {
        self.scope_stack.push(scope.clone());
        scope
    }


}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub value: runtime::RuntimeValue,
    pub mutability: bool
}


impl Symbol {
    pub fn new() -> Self {
        Self { name:  String::from(""), value: runtime::RuntimeValue::Nil, mutability: false }
    }
}