use std::collections::HashMap;

use crate::nil::token::Token;
use crate::nil::grammar;
use Token::*;
use grammar::*;
use ASTNode::*;
use Expression::*;

#[derive(Debug)]
pub struct Error {

}

pub struct ParserSettings {
    operator_precednece: HashMap<String, i32>
}

impl ParserSettings {
    pub fn default() -> Self {
        let mut op_prec = HashMap::new();
        op_prec.insert("<".to_string(), 10);
        op_prec.insert("+".to_string(), 20);
        op_prec.insert("-".to_string(), 20);
        op_prec.insert("*".to_string(), 40);
        ParserSettings{operator_precednece: op_prec}
    }
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

        if tokens.len() == 0 {
            break
        }

        hold.push(tokens.remove(0));
        
    }
    
    Ok(ast)
}

fn parse_function(tokens: &mut Vec<Token>, settings: &mut ParserSettings) -> PartParsingResult<ASTNode> {
    tokens.remove(0); //Removes Def
    let body = parse_expr(tokens, settings, &Vec::new());
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
    let expression = parse_expr(tokens, settings, hold);
    let prototype = Prototype{name: "".to_string(), args: vec![]};
    let lambda = Function{prototype: prototype, body: get_result(expression)};

    PartParsingResult::Good(FunctionNode(lambda))
}

fn parse_prototype(tokens: &mut Vec<Token>, _settings: &mut ParserSettings) -> PartParsingResult<Prototype> {
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

fn parse_primary_expr(tokens: &mut Vec<Token>, settings: &mut ParserSettings) -> PartParsingResult<Expression> {
    if &tokens[0] == &Delimiter {
        tokens.remove(0);
    }
    match &tokens[0] {
        Ident(name) => {
            //Only variable start with Ident
            let id_name = name.to_owned();
            tokens.remove(0);
            PartParsingResult::Good(VariableExpr(id_name.to_string()))
        },
        Number(_) => parse_literal_expr(tokens, settings),
        OpeningBrac => parse_call_expr(tokens, settings),
        OpeningPars => parse_parenthesis_expr(tokens, settings),
        _ => error(format!("error parsing primary expr with token: {:?}", tokens[0]).as_str())
    }
}

fn parse_call_expr(tokens : &mut Vec<Token>, settings : &mut ParserSettings) -> PartParsingResult<Expression> {
    tokens.remove(0);//removes OpeningBrac
    
    let name: String;
    match tokens.remove(0) {
        Ident(val) => name = val,
        _ => return error("expected function name in prototype")
    }

    if tokens.remove(0) != ClosingBrac {
        return error("expected function name in prototype")
    }

    let mut args = vec![];

    loop {
        if tokens.len() == 0 {
            break
        }
        match &tokens[0] {
            Delimiter => break, //Next line(end of args) starts with ';'
            _ => args.push(get_result(parse_expr(tokens, settings, &Vec::new())))
        }
    }

    PartParsingResult::Good(CallExpr(name, args))
}

fn parse_literal_expr(tokens : &mut Vec<Token>, _settings : &mut ParserSettings) -> PartParsingResult<Expression> {
    match tokens.remove(0) {
        Number(val) => PartParsingResult::Good(LiteralExpr(val)),
        _ => error("literal expected")
    }
}

fn parse_parenthesis_expr(tokens : &mut Vec<Token>, settings : &mut ParserSettings) -> PartParsingResult<Expression> {
    tokens.remove(0); //removes '('
    let expr = parse_expr(tokens, settings, &Vec::new());

    if tokens.remove(0) != ClosingPars {
        return error("expected ')'")
    }

    PartParsingResult::Good(get_result(expr))
}

fn parse_expr(tokens: &mut Vec<Token>, settings: &mut ParserSettings, _hold: &Vec<Token>) -> PartParsingResult<Expression> {
    let lhs = parse_primary_expr(tokens, settings);
    let expr = parse_binary_expr(tokens, settings, 0, &get_result(lhs));

    PartParsingResult::Good(get_result(expr))
}

fn parse_binary_expr(tokens: &mut Vec<Token>, settings: &mut ParserSettings, expr_precednce: i32, lhs: &Expression) -> PartParsingResult<Expression> {
    let mut result = lhs.clone();

    loop {
        
        let (operator, precednce) = match &tokens.first() {
            Some(&Operator(ref op)) => {
                match settings.operator_precednece.get(op) { //checks hashmap for op
                    Some(pr) if *pr >= expr_precednce => (op.clone(), *pr),
                    None => return error(&format!("unkonwn operator: {}", op)),
                    _ => break,
                }
            },
            _ => break
        };
        tokens.remove(0);
    
        let mut rhs = parse_primary_expr(tokens, settings);

        loop {
            let binary_rhs = match &tokens.first() {
                Some(&Operator(ref op)) => {
                    match settings.operator_precednece.get(op) {
                        Some(pr) if *pr > precednce => parse_binary_expr(tokens, settings, *pr, &get_result(rhs)),
                        None => return error(&format!("unkonwn operator: {}", op)),
                        _ => break
                    }
                },
                _ => break
            };

            rhs = binary_rhs;
        }

        result = BinaryExpr(operator, Box::new(result), Box::new(get_result(rhs)));

    }

    PartParsingResult::Good(result)
}
