use crate::nil::grammer::Value;

#[derive(Debug)]
pub struct Token {
    pub value: TokenVal,
    pub pos: (usize, usize)
}

#[derive(PartialEq, Clone, Debug)]
pub enum TypeOf {
    Num    
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
    Else,
    Assignment,
    Ident(String),
    Value(Value),
    Operator(String),
    Type(TypeOf),
    Logical(String)
}
