use std::collections::HashMap;
use crate::nil::grammar::Value;

use crate::nil::grammar::{Expression, Prototype, Function};

#[derive(Clone)]
pub struct Scope {
    pub var: HashMap<String, Value>,
    pub funs: HashMap<String, Function>//fn prot body
}

impl Scope {
    pub fn new() -> Self {
        let var: HashMap<String, Value> = HashMap::new();
        let funs: HashMap<String, Function> = HashMap::new();

        Scope {var: var, funs: funs}
    }
}
