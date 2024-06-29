use crate::nil::grammar::Value;

#[derive(Debug)]
pub struct Token {
    pub value: TokenVal,
    pub pos: (usize, usize)
}

#[derive(PartialEq, Clone, Debug)]
pub enum TypeOf {
    Num,
    Bool,
    String
}

#[derive(PartialEq, Clone, Debug)]
pub enum TokenVal {
    Delimiter, //; char
    OpeningPars,
    ClosingPars,
    OpeningBrac,
    ClosingBrac,
    Def,
    Extern,
    NIf,
    NWhile,
    Else,
    Assignment,
    Ident(String),
    Value(Value),
    Operator(String),
    Type(TypeOf),
    Logical(String)
}
