pub mod string_syntaxerror;

use string_syntaxerror::StringSyntaxError;

use crate::error::{
    self,
    debug_trait::{CompilerError, EnumCompilerError, ErrorLevel},
};

pub enum ParseCodeError {
    StringSyntaxError(StringSyntaxError),
}

impl<E: CompilerError> EnumCompilerError<E> for ParseCodeError {
    fn soruce(&self) -> E {
        match self {
            ParseCodeError::StringSyntaxError(e) => e.clone()
        }
    }
}
