use crate::nil::token::Token;
use crate::nil::grammar;
use Token::*;
use grammar::*;

#[derive(Debug)]
pub struct Error {

}

pub enum ParserSettings {
    None
}

enum PartParsingResult<T> {
    Good(T, Vec<Token>),
    NotComplete,
    Bad(String)
}

fn error<T>(message: &str) -> PartParsingResult<T> {
    PartParsingResult::Bad(message.to_string())
}

pub fn parser(tokens: &mut Vec<Token>, settings: &mut ParserSettings) -> Result<Vec<ASTNode>, Error> {
    let mut ast: Vec<ASTNode> = vec![];
    let mut hold: Vec<Token> = vec![]; //keeps tokens of the current line

    loop {
        if(tokens.len() == 0) {
            break
        }

        let cur_token = &tokens[0];

        let result = match cur_token {
            Def => parse_function(tokens, settings),
            Extern => parse_extern(tokens, settings),
            Delimiter => {
                hold = vec![];
                tokens.remove(0);
                continue
            },
            _ => parse_expression(tokens, &hold, settings)
        };

        hold.push(tokens.remove(0));
        //tokens.remove(0);

    } 

    Ok(ast)
}

fn parse_function(tokens: &mut Vec<Token>, settings: &mut ParserSettings) -> PartParsingResult<ASTNode> {
    todo!();
}

fn parse_extern(tokens: &mut Vec<Token>, settings: &mut ParserSettings) -> PartParsingResult<ASTNode> {
    todo!();
}
fn parse_expression(tokens: &mut Vec<Token>, hold: &Vec<Token>, settings: &mut ParserSettings) -> PartParsingResult<ASTNode> {
    todo!();
}

