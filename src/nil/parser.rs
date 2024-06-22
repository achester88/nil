use crate::nil::token::Token;
use crate::nil::grammar;
use Token::*;
use grammar::*;
use ASTNode::*;

#[derive(Debug)]
pub struct Error {

}

pub enum ParserSettings {
    None
}

enum PartParsingResult<T> {
    Good(T),
    Bad(String)
}

fn error<T>(message: &str) -> PartParsingResult<T> {
    PartParsingResult::Bad(message.to_string())
}

fn get_result<T>(ppresult: PartParsingResult<T>) -> T {
    match ppresult {
        PartParsingResult::Good(value) => value,
        PartParsingResult::Bad(mes) => {
            println!("\x1b[91mError\x1b[0m \n {}", mes);
            panic!();
        }
    }
}

pub fn parser(tokens: &mut Vec<Token>, settings: &mut ParserSettings) -> Result<Vec<ASTNode>, Error> {
    let mut ast: Vec<ASTNode> = vec![];
    let mut hold: Vec<Token> = vec![]; //keeps tokens of the current line

    loop { 
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
        
        ast.push(get_result(result));

        if(tokens.len() == 0) {
            break
        }

        hold.push(tokens.remove(0));
        
    }
    
    Ok(ast)
}

fn parse_function(tokens: &mut Vec<Token>, settings: &mut ParserSettings) -> PartParsingResult<ASTNode> {
    tokens.remove(0); //Removes Def
    let body = parse_expr(tokens, settings);
    let prototype = parse_prototype(tokens, settings);
    PartParsingResult::Good(FunctionNode(Function{
        prototype: get_result(prototype),
        body: get_result(body)
    }))
}

fn parse_extern(tokens: &mut Vec<Token>, settings: &mut ParserSettings) -> PartParsingResult<ASTNode> {
    tokens.remove(0); //Removes Def token
    let prototype = parse_prototype(tokens, settings);
    PartParsingResult::Good(ExternNode(get_result(prototype)))
    
}
fn parse_expression(tokens: &mut Vec<Token>, hold: &Vec<Token>, settings: &mut ParserSettings) -> PartParsingResult<ASTNode> {
    todo!();
}

fn parse_prototype(tokens: &mut Vec<Token>, settings: &mut ParserSettings) -> PartParsingResult<Prototype> {
    //Find '{'
    if tokens.remove(0) != OpeningBrac {
        return error("expected '{' in function prototype");
    }
    //collect args
    //Find '}'
    let mut args = vec![];

    loop {
        match tokens.remove(0) {
            Ident(arg) => args.push(arg),
            ClosingBrac => break,
            _ => return error("expected '}' in function prototype")
        }
    }
    //Collect fn Name
    let name: String;
    match tokens.remove(0) {
        Ident(val) => name = val,
        _ => return error("expected function name in prototype")
    }

    PartParsingResult::Good(Prototype{
        name: name, 
        args: args
    })
}

fn parse_expr(tokens: &mut Vec<Token>, settings: &mut ParserSettings) -> PartParsingResult<Expression> {
    todo!();
}
