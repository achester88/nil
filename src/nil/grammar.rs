#[derive(PartialEq, Clone, Debug)]
pub enum Value {
    Num(f64),
    Bool(bool),
    String(String)
}

#[derive(PartialEq, Clone, Debug)]
pub enum ASTNode {
    ExternNode(Prototype),
    FunctionNode(Function)
}

#[derive(PartialEq, Clone, Debug)]
pub struct Function {
    pub prototype: Prototype,
    pub body: Expression
}

#[derive(PartialEq, Clone, Debug)]
pub struct Prototype {
    pub name: String,
    pub args: Vec<String>
}

#[derive(PartialEq, Clone, Debug)]
pub enum Expression {
    LiteralExpr(Value),
    VariableExpr(String),
    BinaryExpr(String, Box<Expression>, Box<Expression>),
    //cond, then, else
    ConditionalExpr{cond_expr: Box<Expression>, then_expr: Box<Expression>, else_expr: Option<Box<Expression>>},
    CallExpr(String, Vec<Expression>)
}