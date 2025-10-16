use colored::Colorize;

use crate::error::debug_trait::{CompilerError, ErrorLevel};

#[derive(Debug)]
pub struct StringSyntaxError {
    file: String,
    line: u32,
    col: u16,
}

impl StringSyntaxError {
    pub fn new(file: String, line: u32, col: u16) -> Self {
        Self { file, line, col }
    }
}

impl std::fmt::Display for StringSyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.err_fmt())
    }
}

impl Clone for StringSyntaxError {
    fn clone(&self) -> Self {
        Self::new(self.file.clone(), self.line.clone(), self.col.clone())
    }

    fn clone_from(&mut self, source: &Self) {
        self.file = source.file.clone();
        self.line = source.line.clone();
        self.col = source.col.clone();
    }
}

impl CompilerError for StringSyntaxError {
    fn err_fmt(&self) -> String {
        format!(
            "    {}[0001]: are you trying to writing a string without a ending?????, stop!\n        {} | {}\n{}^\n\n",
            "Error".red().bold(),
            self.line,
            match std::fs::read_to_string(&self.file) {
                Ok(f) => {
                    let middle = f.split('\n').collect::<Vec<&str>>();
                    let result: &str = middle.get::<usize>((self.line - 1) as usize).unwrap();
                    result.to_string()
                }
                Err(_) => panic!("!!!!!Error gaing failed"),
            },
            "Tips: YOU HAVE NO TIPS!!!!, GO BACK TO WORK!!!"
        ).to_string()
    }

    fn err_code(&self) -> u32 {
        1
    }

    fn err_level(&self) -> ErrorLevel {
        ErrorLevel::CompilerPanicError
    }
}