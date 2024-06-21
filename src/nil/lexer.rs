use regex::Regex;

use crate::nil::token::Token;
use Token::*;

pub fn tokenizer(input: String) -> Vec<Token> {
    let mut tokens = Vec::new();

    let code_re = Regex::new(r"\/\*([\s\S]*?)\*\/").unwrap();   //find /* code */
  
     let filtered = code_re
    .find_iter(&input)
    .filter_map(|segments| segments.as_str()[2..(segments.len()-2)].parse().ok())
    .collect::<Vec<String>>()
    .join("");

    let mut by_lines: Vec<&str> = filtered.split("\n").collect();

    by_lines.reverse();

    let code = by_lines
     .join("")
     .replace("\n", "")
     .replace("\r", "");

    let token_re = Regex::new(concat!(
        r"(?P<ident>\p{Alphabetic}\w*)|",
        r"(?P<number>\d+\.?\d*)|",
        r"(?P<delimiter>;)|",
        r"(?P<oppar>\()|",
        r"(?P<clpar>\))|",
        r"(?P<opbar>\{)|",
        r"(?P<clbar>\})|",
        r"(?P<operator>\S)"
    )).unwrap();
    
    for caputure in token_re.captures_iter(code.as_str()) {
        let token = if caputure.name("ident").is_some() {
            match caputure.name("ident").unwrap().as_str() {
                "def" => Def,
                "extern" => Extern,
                ident => Ident(ident.to_owned())
            }
        } else if caputure.name("number").is_some() {
            match caputure.name("number").unwrap().as_str().parse() {
            Ok(number) => Number(number),
            Err(_) => {
                println!("\x1b[91mError\x1b[0m Number Format Unrecognized");
                panic!()
            }
            }
        } else if caputure.name("delimiter").is_some() {
            Delimiter
        } else if caputure.name("oppar").is_some() {
            OpeningPars
        } else if caputure.name("clpar").is_some() {
            ClosingPars
        } else if caputure.name("opbar").is_some() {
            OpeningBrac
        } else if caputure.name("clbar").is_some() {
            ClosingBrac
        } else {
            let name = caputure.name("operator").unwrap();
            Operator(name.as_str().to_owned())
        };

        tokens.push(token);

    }

    return tokens;
}
