use std::process;

#[derive(Clone)]
pub struct Error {
    message: String,
    desc: Option<String>,
    pos: Option<(usize, usize)>,
    point: bool,
}

impl Error {
    pub fn mes(mes: &str) -> Self {
        Error {message: mes.to_string(), desc: None, pos: None, point: false}
    }

    pub fn desc(mes: &str, desc: &str) -> Self {
        Error {message: mes.to_string(), desc: Some(desc.to_string()), pos: None, point: false}
    }

    pub fn at(mes: &str, desc: &str, pos: (usize, usize)) -> Self {
        Error {message: mes.to_string(), desc: Some(desc.to_string()), pos: Some(pos), point: false}
    }

    pub fn test() -> Self {
        Error::desc("test error", "This error was written as a test of NIL's debuging ablity")
    }
}

pub struct ErrorHandler {
    source: Vec<String>
}

impl ErrorHandler {
    
    pub fn new(source: String) -> Self {
        let by_lines = source.split("\n").map(|x| x.to_string()).collect();
        ErrorHandler {source: by_lines}
    }

    pub fn throw_err(&self, err: Error) -> ! {
        println!("\x1b[91mError\x1b[0m: {}", err.message);

        if err.desc.is_some() {
            println!("  {}", err.desc.unwrap());
        }

        if err.pos.is_some() {
            let (l, c) = err.pos.unwrap();
            println!("\n    {} |{}", l, self.source[l-1]);
            println!(  "    {}  {}^", " ".repeat(l.to_string().len()), " ".repeat(c));
        }

        process::exit(1);
    }
}  
