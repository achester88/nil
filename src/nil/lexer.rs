use regex::Regex;

use crate::nil::token::Token;
use crate::nil::errorhandler::Error;
use Token::*;

pub fn tokenizer(input: String) -> Result<Vec<Token>, Error> {

    //return Err(Error::desc("test error", "This error was written as a test of NIL's debuging ablity"));

    let mut tokens = vec![];
    let segments: Vec<(Vec<&str>, Vec<u32>)> = vec![];

    let code_re = Regex::new(r"\/\*([\s\S]*?)\*\/").unwrap();   //find /* code */
    /*let filtered = code_re
    .find_iter(&input)//get line num from rexgex match 
    .filter_map(|segments| segments.as_str()[2..(segments.len()-2)].parse().ok())
    .collect::<Vec<String>>()
    .join("");
    */

    for caputure in code_re.captures_iter(&input) {
        //Here we go
        println!("{:#?}", caputure);
    }

    
    println!("{:?}", segments);
    //
    return Err(Error::test());

    /*
    let mut by_lines: Vec<&str> = filtered.split("\n").collect();
    
    by_lines.reverse();

    let code = by_lines
     .join("")
     .replace("\n", "")
     .replace("\r", ""); 
    
    let mut l = by_lines.len();
    
//Go line by line check for /* and */ and if in some use tokenize line
    
   for line in by_lines {
       tokens.append(&mut tokenize_line(line));
       l += 1;
   } 
    */
    Ok(tokens)
}

fn tokenize_line(line: &str) -> Vec<Token> {
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

    let mut temp: Vec<Token> = vec![];

    for caputure in token_re.captures_iter(line) {
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

        temp.push(token);

    }

    return temp;
}
