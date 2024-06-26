use crate::nil::grammar::*;
use Expression::*;

pub fn eval_ast(ast: Vec<ASTNode>) {
    for node in ast {
        eval(node);
    }
}

fn eval(node: ASTNode) -> i32 {
    match node {
            ASTNode::FunctionNode(fun) => {
                if fun.prototype.name == "" {
                    println!("run");
                    
                    eval_expression(fun.body);
                } else { //Named protype add to scope
                }
            },
            ASTNode::ExternNode(prot) => {}
    }

    return -1;
}
//eval args fn?
fn eval_expression(expr: Expression) {
    match expr {
        LiteralExpr(val) => {},
        VariableExpr(name) => {},
        BinaryExpr(op, expr1, expr2) => {
            run(op, eval(expr1), eval(expr2))
        },
        CallExpr(name, args) => {} //check specialforms then scope
    }
}
