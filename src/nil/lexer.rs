use regex::Regex;

use crate::nil::token::{Token, TokenVal};
use crate::nil::errorhandler::Error;
use TokenVal::*;

pub fn tokenizer(input: String) -> Result<Vec<Token>, Error> {

    //return Err(Error::desc("test error", "This error was written as a test of NIL's debuging ablity"));

    let mut tokens = vec![];    
    
    let mut by_lines: Vec<&str> = input.split("\n").collect();
    
    by_lines.reverse();
    
    let mut l = by_lines.len();
    let mut in_segment = false; 
    
   for line in by_lines {
       println!("-------- start {} -----------", l);
       let mut ended = false;
       let start = match line.find("/*") {
           Some(i) => {
               println!("start: {}", i);
               in_segment = false;
               ended = true;
               i+2
           },
           None => 0
       };
 
       let end = match line.find("*/") {
           Some(i) => {
               println!("end: {}", i);
               in_segment = true;
               i
           },
           None => line.len()
       };

       //tokens.append(&mut tokenize_line(line));
       if in_segment || ended {
        println!("{}: {:?}", l, line[start..end].to_string());
        tokens.append(&mut tokenize_line(&line[start..end], l));
       }
       println!("-------- end {} -----------", l);
       l -= 1;
   } 

    Ok(tokens)
}

fn tokenize_line(line: &str, line_num: usize) -> Vec<Token> {
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
        let c = 0; //get pos of token from caputure
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

        temp.push(Token {value: token, pos: (line_num, c)});

    }

    return temp;
}
