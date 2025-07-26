pub trait CompilerError {
    fn fmt(&self) -> String;

    fn source(&self) -> Self;
}
