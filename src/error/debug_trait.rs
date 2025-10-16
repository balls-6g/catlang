pub trait CompilerError : Clone {
    fn fmt(&self) -> String;

    fn err_code(&self) -> u32;

    fn err_level(&self) -> ErrorLevel;
}

pub trait EnumCompilerError<E: CompilerError> {
    fn soruce(&self) -> E;
}

pub enum ErrorLevel {
    Error,
    CompilerPanicError,
    FixableError,
}
