use std::collections::VecDeque;

use colored::Colorize;

use crate::error::debug_trait::{CompilerError, ErrorLevel};
#[derive(Debug)]
pub struct CharUnusableError {
    file: String,
    line: u32,
    col: u16
}

impl CharUnusableError {
    pub fn new(file: String, line: u32, col: u16) -> Self {
        Self {
            file,
            line,
            col
        }
    }

}

impl std::fmt::Display for CharUnusableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.err_fmt())
    }
}


impl Clone for CharUnusableError {
    fn clone(&self) -> Self {
        Self::new(self.file.clone(), self.line.clone(), self.col.clone())
    }

    fn clone_from(&mut self, source: &Self) {
        self.file = source.file.clone();
        self.line = source.line.clone();
        self.col = source.col.clone();
    }
}

impl CompilerError for CharUnusableError {
    fn err_fmt(&self) -> String {
        format!(
            "   {}[0002]: Stop putting more than 1 charaters in the char!!!!!??\n        {} | {}\n{}^\n\n",
            "Error".bold().red(),
            self.line,
            match std::fs::read_to_string(&self.file) {
                Ok(f) => {
                    let middle = f.split('\n').collect::<Vec<&str>>();
                    let result: &str = middle.get::<usize>((self.line - 1) as usize).unwrap();
                    result.to_string()

                }
                Err(_) => panic!("!!!!!Error ganing failed"),
            },
            "Tips: try to use string instead of char to store more than one chracter of text \u{eda9}"
        ).to_string()
    }

    fn err_level(&self) -> ErrorLevel {
        ErrorLevel::CompilerPanicError
    }

    fn err_code(&self) -> u32 {
        2
    }
}
