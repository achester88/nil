use std::collections::HashMap;

use crate::nil::grammar::{Expression, Prototype};

pub struct Scope {
    var: HashMap<String, f64>,
    funs: HashMap<String, Prototype, Expression>//fn prot body
}
