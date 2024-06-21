use regex::Regex;

use crate::nil::token::Token;
use Token::*;

pub fn tokenizer(input: String) -> Vec<Token> {
    let mut tokens = Vec::new();

    let code_re = Regex::new(r"\/\*([\s\S]*?)\*\/").unwrap();   //find /* code */
    
    let oneline = &input.replace("\n", "").replace("\r", "");

    let code = code_re.find_iter(oneline)
        .filter_map(|segments| segments.as_str()[2..(segments.len()-2)].parse().ok())
        .collect::<Vec<String>>()
        .join("");

    println!("{:#?}", code);
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
            let name = caputure.name("ident").unwrap();
            Ident(name.as_str().to_owned())
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
        } else {
            Delimiter
        };

        tokens.push(token);

    }

    return tokens;
}
