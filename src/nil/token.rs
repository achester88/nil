#[derive(PartialEq, Clone, Debug)]
pub enum Token {
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
