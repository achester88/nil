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
                    
                    return eval_expression(&sp, scope, fun.body);
                } else { //Named protype add to scope
                    scope.funs.insert(fun.prototype.name.clone(), fun);
                }
            },
            ASTNode::ExternNode(prot) => {}
    }

    return -1.0;
}
//eval args fn?
fn eval_expression(sp: &SpecialForms, scope: &mut Scope, expr: Expression) -> f64 {
    match expr {
        LiteralExpr(val) => val,
        VariableExpr(name) => {
            match scope.var.get(&name) {
                Some(val) => *val,
                None => error(format!("Undefined Varable: {}", name))
            } 
        },
        BinaryExpr(op, expr1, expr2) => {//split Binary Ops and built in fn?
            let lhs = eval_expression(&sp, scope, *expr1);
            let rhs = eval_expression(&sp, scope, *expr2);
            run(sp, scope, op, vec!(lhs, rhs))
        },
        CallExpr(name, args) => {
            let args_vals = args.into_iter().map(|expr| eval_expression(&sp, scope, expr)).collect();
            run(sp, scope, name, args_vals)
        } 
    }
}

fn run(sp: &SpecialForms, scope: &mut Scope, fn_name: String, args: Vec<f64>) -> f64 {
    println!("name: {}, args: {:?}", fn_name, args);

    match sp.map.get(&fn_name) {
        Some(fun) => {
            match fun(args) {
                Ok(val) => val,
                Err(mes) => error(mes)
            }
        },
        None => {
            match scope.funs.get(&fn_name) {
                Some(fun) => {
                    println!("Found: {:?}", fun);
                    //extend scope
                    let mut temp_scope = scope.clone(); //find a better slouition
                    for i in 0..args.len() { //check args count matches
                        temp_scope.var.insert(fun.prototype.args[i].to_string(), args[i]);
                    }
                    //eval
                    eval(
                        ASTNode::FunctionNode(Function {
                            prototype: Prototype {name: String::from(""), 
                            args: vec![]}, body: fun.body.clone()
                        }),
                        &sp,
                        &mut temp_scope
                    )
                },
                None => error(format!("Undefined Function: {}", fn_name))
            }
        }
    }

}

fn error(mes: String) -> ! {
    println!("\x1b[91mError\x1b[0m \n {}", mes);
    panic!();
}
