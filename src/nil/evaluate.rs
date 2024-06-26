use crate::nil::grammar::*;
use crate::nil::specialforms::SpecialForms;
use crate::nil::scope::Scope;
use Expression::*;

pub fn eval_ast(ast: Vec<ASTNode>) {
    let specialforms = SpecialForms::new();
    let mut scope = Scope::new();

    for node in ast {
        eval(node, &specialforms, &mut scope);
    }
}

fn eval(node: ASTNode, sp: &SpecialForms, scope: &mut Scope) -> f64 {
    match node {
            ASTNode::FunctionNode(fun) => {
                if fun.prototype.name == "" {
                    println!("run");
                    
                    eval_expression(&sp, fun.body);
                } else { //Named protype add to scope
                }
            },
            ASTNode::ExternNode(prot) => {}
    }

    return -1.0;
}
//eval args fn?
fn eval_expression(sp: &SpecialForms, expr: Expression) -> f64 {
    match expr {
        LiteralExpr(val) => val,
        VariableExpr(name) => {
           -1.0 
        },
        BinaryExpr(op, expr1, expr2) => {
            run(sp, op, vec!(eval_expression(&sp, *expr1), eval_expression(&sp, *expr2)))
        },
        CallExpr(name, args) => -1.0 //check specialforms then scope
    }
}

fn run(sp: &SpecialForms, fn_name: String, args: Vec<f64>) -> f64 {
    println!("name: {}, args: {:?}", fn_name, args);

    match sp.map.get(&fn_name) {
        Some(fun) => {
            match fun(args) {
                Ok(val) => val,
                Err(mes) => {error(mes)}
            }
        },
        None => {-1.0}//check scope
    }
    
    /*if sp.map.contains_key(&*fn_name) {
        match self.special_forms.get(&fn_name)(args) {
            Ok(val) => return val,
            Err(mes) => {
                println!("\x1b[91mError\x1b[0m \n {}", mes);
                panic!();
            }
        }
    }*/
    //-1.0
}

fn error(mes: String) -> ! {
    println!("\x1b[91mError\x1b[0m \n {}", mes);
    panic!();
}
