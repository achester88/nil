use std::env;
use std::panic;

mod nil;
use crate::nil::lexer;
use crate::nil::parser;
use parser::ParserSettings;

fn main() {
    let args: Vec<String> = env::args().collect();

    panic::set_hook(Box::new(|_info| {
        // do nothing
    }));

    match args.len() {
        1 => {
            println!("\x1b[91mError\x1b[0m Missing Argument:\n <path>");
            panic!();
        }
        2 => {
            let path = &args[1];

            match path.as_ref() {
                "--help" => println!("USEAGE:\n nil <path>"),
                _ => {
                    let parts: &Vec<&str> = &path.as_str().split('.').collect();
                    if parts.len() == 1 {
                        println!("\x1b[91mError\x1b[0m Unkown Argument Passed");
                    panic!();
                    } else {
                        let ent = parts[parts.len()-1];
                        if ent != "nil" {
                            println!("\x1b[91mError\x1b[0m File has unexpected file extension of .{}", ent);
                            panic!();
                        }

                        let content: String;
                        match std::fs::read_to_string(&path) {
                            Ok(x) => content = x,
                            Err(_e) => {
                                println!("\x1b[91mError\x1b[0m Could Not Find File at Path: `{}`", path);
                                panic!();
                            }
                        }

                        //Start of Processing
                        let mut tokens = lexer::tokenizer(content);
                        println!("{:?}\n\n", &tokens);
                        let tree = parser::parser(&mut tokens, &mut ParserSettings::default());
                        println!("{:?}\n\n", &tree.unwrap());
                    }
                }
            }
        }
        _ => {
            println!("\x1b[91mError\x1b[0m Unkown Argument Passed");    
        }
    }
}
