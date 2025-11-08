use crate::error::debug_trait::{CompilerError, ErrorLevel};

use colored::Colorize;

pub struct Context<E: CompilerError> {
    proj: String,                 // Current project path
    file: String,                 // Current file path
    line: u32,                    // Current line number
    col: u32,                     // Current line col number
    error_pool: Option<Vec<E>>,        // gaining error pool
    warning_pool: Option<String>, // TODO: add `Warning` trait implementation and complete the code
    string_pool: Vec<String>,     // ...
    char_pool: Vec<char>
}

impl<E: CompilerError> Context<E> {
    pub fn new(proj: String, file: String, line: u32, col: u32) -> Self {
        Self {
            proj,
            file,
            line,
            col,
            error_pool: None,
            warning_pool: None,
            string_pool: vec![],
            char_pool: vec![]
        }
    }

    pub fn borrow_mut_error_pool(&mut self) -> &mut Option<Vec<E>> {
        &mut self.error_pool
    }

    pub fn string_pool_push(&mut self, v: String) {
        self.string_pool.push(v);
    }

    pub fn char_pool_push(&mut self, c: char) {
        self.char_pool.push(c);
    }

    pub fn borrow_mut_warning_pool(&mut self) -> &mut Option<String> {
        &mut self.warning_pool
    }

    pub fn borrow_mut_string_pool(&mut self) -> &mut Vec<String> {
        &mut self.string_pool
    }

    pub fn borrow_mut_char_pool(&mut self) -> &mut Vec<char> {
        &mut self.char_pool
    }

    pub fn set_pos(&mut self, proj: String, file: String, line: u32, col: u32) {
        self.proj = proj;
        self.file = file;
        self.line = line;
        self.col = col;
    }

    pub fn compiler_panic_error_start(&self, err: E) {
        match err.err_level() {
            ErrorLevel::CompilerPanicError => {
                println!("{}", err.err_fmt())
            }
            _ => {
                eprintln!(
                    "{}", "=============================== COMPILER INSIDER PANIC ===============================\n".red().bold()
                );
                eprintln!(
                    "insider error ocurrs: A error that is not in `CompilerPanic` Error level calls `compiler_panic_error_start`"
                );
                eprintln!(
                    "if you are a compiler user, please report at {}",
                    "https://github.com/balls-6g/catlang/issues"
                        .bright_yellow()
                        .italic()
                );
                eprintln!(
                    "{}", "======================================================================================".red().bold()
                );

                std::process::exit(1);
            }
        }
    }

    pub fn get_current_file(&self) -> String {
        self.file.clone()
    }

    pub fn get_string_pool(&self) -> Vec<String> {
        self.string_pool.clone()
    }

    pub fn get_char_pool(&self) -> Vec<char> {
        self.char_pool.clone()
    }
}
