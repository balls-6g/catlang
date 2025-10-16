use colored::Colorize;

use crate::error::debug_trait::{CompilerError, ErrorLevel};

#[derive(Clone)]
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

impl CompilerError for StringSyntaxError {
    fn fmt(&self) -> String {
        format!(
            "    {}[0001]: are you trying to writing a string without a ending?????, stop!\n        {} | {}\n{}^\n\n",
            "Error".red().bold(),
            self.line,
            match std::fs::read_to_string(&self.file) {
                Ok(f) => {
                    let middle = f.split('\n').collect::<Vec<&str>>();
                    let result = middle.get((self.line - 1) as usize).unwrap();
                    result.to_string()
                }
                Err(_) => panic!("!!!!!Error gaing failed"),
            },
            "Tips: YOU HAVE NO TIPS!!!!, GO BACK TO WORK!!!"
        ).to_string()
    }

    fn err_code(&self) -> u32 {
        0001
    }

    fn err_level(&self) -> ErrorLevel {
        ErrorLevel::CompilerPanicError
    }
}
