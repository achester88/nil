pub enum Token {
    Delimiter, //; char
    OpeningPars,
    ClosingPars,
    OpeningBrac,
    ClosingBrac,
    Comma,
    Ident(String),
    Number(f64),
    Operator(String)
}
