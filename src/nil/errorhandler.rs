use std::process;

pub struct Error {
    message: String,
    desc: Option<String>,
    pos: Option<(u32, u32)>,
    point: bool,
}

impl Error {
    pub fn mes(mes: &str) -> Self {
        Error {message: mes.to_string(), desc: None, pos: None, point: false}
    }

    pub fn desc(mes: &str, desc: &str) -> Self {
        Error {message: mes.to_string(), desc: Some(desc.to_string()), pos: None, point: false}
    }

    pub fn test() -> Self {
        Error::desc("test error", "This error was written as a test of NIL's debuging ablity")
    }
}

pub struct ErrorHandler {
    source: String
}

impl ErrorHandler {
    
    pub fn new(source: String) -> Self {
        ErrorHandler {source: source}
    }

    pub fn throw_err(&self, err: Error) -> ! {
        println!("\x1b[91mError\x1b[0m: {}", err.message);

        if err.desc.is_some() {
            println!("  {}", err.desc.unwrap());
        }

        process::exit(1);
    }
}  
