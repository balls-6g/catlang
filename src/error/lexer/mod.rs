pub mod char_unusable_error;
pub mod string_syntax_error;

use string_syntax_error::StringSyntaxError;

use crate::error::{debug_trait::CompilerError, lexer::char_unusable_error::CharUnusableError};

#[derive(Debug)]
pub enum LexerError {
    StringSyntaxError(StringSyntaxError),
    CharUnusableError(CharUnusableError),
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::StringSyntaxError(e) => write!(f, "{}", e),
            LexerError::CharUnusableError(e) => write!(f, "{}", e),
        }
    }
}

impl Clone for LexerError {
    fn clone(&self) -> Self {
        match self {
            LexerError::StringSyntaxError(e) => LexerError::StringSyntaxError(e.clone()),
            LexerError::CharUnusableError(e) => LexerError::CharUnusableError(e.clone()),
        }
    }
}

impl LexerError {
    fn source<E>(&self) -> Result<E, &'static str>
    where
        E: CompilerError + 'static,
    {
        match self {
            LexerError::StringSyntaxError(e) => {
                if std::any::TypeId::of::<E>() == std::any::TypeId::of::<StringSyntaxError>() {
                    let e_clone = e.clone();
                    let result =
                        unsafe { std::mem::transmute_copy::<StringSyntaxError, E>(&e_clone) };

                    Ok(result)
                } else {
                    Err("type mismatch")
                }
            }
            LexerError::CharUnusableError(e) => {
                if std::any::TypeId::of::<E>() == std::any::TypeId::of::<CharUnusableError>() {
                    let e_clone = e.clone();
                    let result =
                        unsafe { std::mem::transmute_copy::<CharUnusableError, E>(&e_clone) };

                    Ok(result)
                } else {
                    Err("type_mismatched")
                }
            }
        }
    }
}
