use std::collections::HashMap;

use crate::nil::grammar::{Expression, Prototype, Function};

pub struct Scope {
    pub var: HashMap<String, f64>,
    funs: HashMap<String, Function>//fn prot body
}

impl Scope {
    pub fn new() -> Self {
        let var: HashMap<String, f64> = HashMap::new();
        let funs: HashMap<String, Function> = HashMap::new();

        Scope {var: var, funs: funs}
    }
}
