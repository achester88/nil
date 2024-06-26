use std::collections::HashMap;

type Callback = fn(Vec<f64>) -> Result<f64, String>;
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

fn add(args: Vec<f64>) -> Result<f64, String> {
    Ok(args[0] + args[1])
}

fn sub(args: Vec<f64>) -> Result<f64, String> {
    Ok(args[0] - args[1])
}

fn mul(args: Vec<f64>) -> Result<f64, String> {
    Ok(args[0] * args[1])
}

fn div(args: Vec<f64>) -> Result<f64, String> {
    if args[1] == 0.0 {
        return Err(String::from("Div by zero"));
    }
    Ok(args[0] / args[1])
}

fn output(args: Vec<f64>) -> Result<f64, String> {
    let mut out: Vec<char> = vec![];

    for i in 0..args.len() {
        let str_val = args[i].to_string();
        out.append(&mut str_val.chars().collect());
        out.push(' ');
    }

    println!("{}", out.into_iter().collect::<String>());
    Ok(1.0)
}
