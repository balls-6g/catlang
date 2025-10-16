pub trait CompilerError: Clone + std::fmt::Debug + std::fmt::Display {
    fn err_fmt(&self) -> String;

    fn err_code(&self) -> u32;

    fn err_level(&self) -> ErrorLevel;
}

pub enum ErrorLevel {
    Error,
    CompilerPanicError,
    FixableError,
}
