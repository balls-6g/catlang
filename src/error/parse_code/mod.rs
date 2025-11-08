pub mod string_syntax_error;
pub mod char_unusable_error;

use string_syntax_error::StringSyntaxError;

use crate::error::{
    debug_trait::CompilerError, parse_code::char_unusable_error::CharUnusableError,
};

#[derive(Debug)]
pub enum ParseCodeError {
    StringSyntaxError(StringSyntaxError),
    CharUnusableError(CharUnusableError),
}

impl std::fmt::Display for ParseCodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseCodeError::StringSyntaxError(e) => write!(f, "{}", e),
            ParseCodeError::CharUnusableError(e) => write!(f, "{}", e)
        }
    }
}

impl Clone for ParseCodeError {
    fn clone(&self) -> Self {
        match self {
            ParseCodeError::StringSyntaxError(e) => ParseCodeError::StringSyntaxError(e.clone()),
            ParseCodeError::CharUnusableError(e) => ParseCodeError::CharUnusableError(e.clone())
        }
    }
}

impl ParseCodeError {
    fn source<E>(&self) -> Result<E, &'static str>
    where
        E: CompilerError +'static,
    {
        match self {
            ParseCodeError::StringSyntaxError(e) => {
                if std::any::TypeId::of::<E>() == std::any::TypeId::of::<StringSyntaxError>() {
                    let e_clone = e.clone();
                    let result = unsafe {
                        std::mem::transmute_copy::<StringSyntaxError, E>(&e_clone)
                    };

                    Ok(result)
                } else {
                    Err("type mismatch")
                }
            },
            ParseCodeError::CharUnusableError(e) => {
                if std::any::TypeId::of::<E>() == std::any::TypeId::of::<CharUnusableError>() {
                    let e_clone = e.clone();
                    let result = unsafe {
                        std::mem::transmute_copy::<CharUnusableError, E>(&e_clone)
                    };

                    Ok(result)
                } else {
                    Err("type_mismatched")
                }
            }
        }
    }
}
