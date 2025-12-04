use crate::{
    context::Context,
    error::{debug_trait::CompilerError, lexer::LexerError},
};

// TODO: Complete this
fn parse_symbol<E: CompilerError>(code: &str, ctx: Context<E>) -> Result<(), LexerError> {
    let mut comment = false;
    let mut string_started = false;

    Ok(())
}
