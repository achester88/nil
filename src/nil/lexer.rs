use regex::Regex;

use crate::nil::token::Token;

pub fn tokenizer(input: String) -> Vec<Token> {
    let mut tokens = Vec::new();

    let code_re = Regex::new(r"\/\*([\s\S]*?)\*\/").unwrap();

    let code: Vec<String> = code_re.captures_iter(&input)
        .filter_map(|segments| segments.as_str().parse().ok())
        .collect();

    println!("{:#?}", code);

    return tokens;
}
