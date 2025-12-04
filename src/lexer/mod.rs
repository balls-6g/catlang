mod symbol;

use crate::context::Context;
use crate::error::debug_trait::CompilerError;
use crate::error::lexer::char_unusable_error::CharUnusableError;
use crate::error::lexer::{self};

// TODO: add char support
pub fn gane_string_pool<E: CompilerError>(
    code: &str,
    ctx: &mut Context<E>,
) -> Result<(), lexer::LexerError> {
    let mut string_buf = String::new();
    let mut string_started = false;
    let mut char_started = false;
    let mut comment = false;
    let mut char_buf: Option<char> = None;
    let mut last_s_line: u32 = 0;
    let mut last_s_col: u16 = 0;

    for (line_idx, line) in code.lines().enumerate() {
        let mut chars = line.chars().peekable();
        let mut col_idx = 0;

        while let Some(ch) = chars.next() {
            match ch {
                '"' => {
                    if comment {
                        // pass for comment
                        continue;
                    }

                    if char_started {
                        // if the charbuf isn't None means this isn't the first char already
                        if char_buf != None {
                            return Err(lexer::LexerError::CharUnusableError(
                                CharUnusableError::new(
                                    ctx.get_current_file(),
                                    line_idx as u32,
                                    col_idx as u16,
                                ),
                            ));
                        } else {
                            char_buf = Some('"');
                        }
                    }

                    last_s_line = line_idx as u32;
                    last_s_col = col_idx as u16;
                    if string_started {
                        ctx.string_pool_push(string_buf.clone());
                        string_buf.clear();
                        string_started = false;
                        continue;
                    } else {
                        string_started = true;
                        continue;
                    }
                }
                '\'' => {
                    if comment {
                        continue;
                    }

                    if string_started {
                        string_buf.push('\'');
                        continue;
                    }

                    if char_started {
                        if char_buf == None {
                            return Err(lexer::LexerError::CharUnusableError(
                                CharUnusableError::new(
                                    ctx.get_current_file(),
                                    line_idx as u32,
                                    col_idx as u16,
                                ),
                            ));
                        } else {
                            ctx.char_pool_push(char_buf.unwrap());
                            char_buf = None;
                            char_started = false;
                            continue;
                        }
                    } else {
                        char_started = true;
                        continue;
                    }
                }
                '\\' => {
                    if comment {
                        continue;
                    }

                    // Lookahead for escaped quote
                    if let Some(&next_ch) = chars.peek() {
                        if next_ch == '"' {
                            if string_started {
                                string_buf.push('"');
                                chars.next();
                                continue;
                            } else if char_started {
                                if char_buf != None {
                                    return Err(lexer::LexerError::CharUnusableError(
                                        CharUnusableError::new(
                                            ctx.get_current_file(),
                                            line_idx as u32,
                                            col_idx as u16,
                                        ),
                                    ));
                                }
                            }
                            col_idx += 1;
                        } else if next_ch == '\'' {
                            if string_started {
                                string_buf.push('\'');
                            }
                            if char_started {
                                if char_buf != None {
                                    return Err(lexer::LexerError::CharUnusableError(
                                        CharUnusableError::new(
                                            ctx.get_current_file(),
                                            line_idx as u32,
                                            col_idx as u16,
                                        ),
                                    ));
                                } else {
                                    chars.next();
                                    char_buf = Some('\'');
                                    continue;
                                }
                            }
                        } else {
                            string_buf.push('\\');
                        }
                    } else {
                        string_buf.push('\\');
                    }
                }
                '-' => {
                    if string_started {
                        string_buf.push('-');
                        continue;
                    } else if char_started {
                        char_buf = Some('-');
                    } else {
                        if let Some(&next_ch) = chars.peek() {
                            if next_ch == '-' && col_idx == 0 {
                                chars.next();
                                comment = true;
                            }
                        }
                    }
                }
                '\n' => {
                    comment = false;
                    continue;
                }
                _ => {
                    if comment {
                        continue;
                    } else if char_started {
                        char_buf = Some(ch)
                    } else if string_started {
                        string_buf.push(ch);
                    }
                }
            }
            col_idx += 1;
        }
    }

    if string_started || char_started {
        return Err(lexer::LexerError::StringSyntaxError(
            crate::error::lexer::string_syntax_error::StringSyntaxError::new(
                ctx.get_current_file(),
                last_s_line,
                last_s_col,
            ),
        ));
    }

    Ok(())
}

mod test {
    // mark: not passed
    #[test]
    fn test_gane_string_pool() {
        let mut context: crate::context::Context<
            crate::error::lexer::string_syntax_error::StringSyntaxError,
        > = crate::context::Context::new(
            "cat-test".to_string(),
            "test/test_gane_string_pool.cat".to_string(),
            1,
            1,
        );

        let code = r#"
            "hello world --fake comment\"" -- comment
            -- comment 2
            'c' -- a char
            '\'' -- another char
        "#;

        super::gane_string_pool(code, &mut context).unwrap_or_else(|e| {
            eprintln!("{}", e);
            std::process::exit(-1);
        });

        assert_eq!(
            context.get_string_pool(),
            vec!["hello world --fake comment\""]
        );
        assert_eq!(context.get_char_pool(), vec!['c', '\''])
    }
}
