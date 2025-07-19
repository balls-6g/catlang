pub trait CompilerError {
    fn fmt(&self) -> String;
}
