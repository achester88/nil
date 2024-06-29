use std::collections::HashMap;
use crate::nil::grammar::Value;
use crate::nil::errorhandler::Error;

/*macro_rules! check_args {
    ( $parms:expr, $types:expr ) => {
        if $parms.len() != $types.len() {
            return Err(String::from("Incorrect Number of Args"));
        }
        for i in 0..$parms.len() {
            if $parms[i] != $types[i] {
                return Err(String::from("missing proper types"))
            }
        }
    };
}*/

macro_rules! get_num {
    ( $arg:expr ) => {
        match $arg {
            Value::Num(num) => num,
            _ => return Err(String::from("improper type of args"))//make clearer
        }
    };
}

type Callback = fn(Vec<Value>) -> Result<Value, String>;
//user def fn call speial function that calls eval

pub struct SpecialForms {
    pub map: HashMap<String, Callback>
}

impl SpecialForms {
    pub fn new() -> Self {
        let mut temp: HashMap<String, Callback> = HashMap::new();

        temp.insert(String::from("+"), add);
        temp.insert(String::from("-"), sub);
        temp.insert(String::from("*"), mul);
        temp.insert(String::from("/"), div);

        temp.insert(String::from("output"), output);

        SpecialForms {map: temp}
    }
}

//----------------------- Bulit-In Functions -----------------------

fn add(args: Vec<Value>) -> Result<Value, String> {
    Ok(Value::Num(get_num!(args[0]) + get_num!(args[1])))
}

fn sub(args: Vec<Value>) -> Result<Value, String> {
    Ok(Value::Num(get_num!(args[0]) - get_num!(args[1])))
}

fn mul(args: Vec<Value>) -> Result<Value, String> {
    Ok(Value::Num(get_num!(args[0]) * get_num!((args[1]))))
}

fn div(args: Vec<Value>) -> Result<Value, String> {
    //let parms = vec!(Value::Num, Value::Num);
    //let Value::Num(num1) = args[0];
    //check_args!(args, parms);
    if get_num!(args[1]) == 0.0 {
        return Err(String::from("Div by zero"));
    }
    Ok(Value::Num(get_num!(args[0]) / get_num!(args[1])))
}

fn output(args: Vec<Value>) -> Result<Value, String> {
    let mut out: Vec<char> = vec![];

    for i in 0..args.len() {
        //let str_val = args[i].to_string();
        let str_val = match &args[i] {
            Value::Num(num) => num.to_string(),
            Value::String(str) => str.clone(),
            Value::Bool(bool) => if *bool {String::from("True")} else {String::from("False")}
        };
        
        out.append(&mut str_val.chars().collect());
        out.push(' ');
    }

    println!("{}", out.into_iter().collect::<String>());
    Ok(Value::Bool(true))
}
