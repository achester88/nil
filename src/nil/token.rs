#[derive(Debug)]
pub struct Token {
    pub value: TokenVal,
    pub pos: (usize, usize)
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
    Ident(String),
    Number(f64),
    Operator(String)
}
