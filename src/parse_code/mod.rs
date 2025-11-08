use crate::context::Context;
use crate::error::debug_trait::CompilerError;
use crate::error::parse_code;
use crate::error::parse_code::char_unusable_error::CharUnusableError;

// TODO: add char support
pub fn gane_string_pool<E: CompilerError>(
    code: &str,
    ctx: &mut Context<E>,
) -> Result<(), parse_code::ParseCodeError> {
    let mut string_buf = String::new();
    let mut string_started = false;
    let mut char_started = false;
    let mut comment = false;
    let mut last_line = 0;
    let mut last_col = 0;
    let mut char_buf: Option<char> = None;

    for (line_idx, line) in code.lines().enumerate() {
        let mut chars = line.chars().peekable();
        let mut col_idx = 0;

        while let Some(ch) = chars.next() {
            match ch {
                '"' => {
                    if comment {
                        continue;
                    }

                    if char_started {
                        if char_buf != None {
                            return Err(
                                parse_code::ParseCodeError::CharUnusableError(
                                    CharUnusableError::new(
                                        ctx.get_current_file(),
                                        last_line,
                                        last_col
                                    )
                                )
                            )
                        } else {
                            char_buf = Some('"');
                        }
                    }

                    let escaped = col_idx > 0 && line.chars().nth(col_idx - 1) == Some('\\');
                    if escaped {
                        string_buf.push('"');
                    } else {
                        if string_started {
                            ctx.string_pool_push(string_buf.clone());
                            string_buf.clear();
                            string_started = false;
                            last_line = line_idx as u32;
                            last_col = col_idx as u16;
                        } else {
                            string_started = true;
                        }
                    }
                }
                '\'' => {
                    if comment {
                        continue;
                    }

                    if string_started {
                        string_buf.push('\'');
                        continue
                    }

                    if char_started {
                        if char_buf != None {
                            return Err(parse_code::ParseCodeError::CharUnusableError(
                                CharUnusableError::new(
                                        ctx.get_current_file(),
                                        last_line as u32,
                                        last_col as u16
                                    )
                                )
                            );
                        } else {
                            ctx.char_pool_push(ch);
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
                            chars.next(); // consume the quote
                            if string_started {
                                string_buf.push('"');
                            }
                            col_idx += 1;
                        } else if next_ch == '\'' {
                            if string_started {
                                string_buf.push('\'');
                            } if char_started {
                                if char_buf != None {
                                    return Err(
                                        parse_code::ParseCodeError::CharUnusableError(CharUnusableError::new(
                                            ctx.get_current_file(),
                                            last_line,
                                            last_col,
                                        ))
                                    )
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
                                comment = true;
                            }
                        }
                    }
                }
                _ => {
                    if comment {
                        continue;
                    }

                    if char_started {
                        char_buf = Some(ch)
                    }
                    
                    if string_started {
                        string_buf.push(ch);
                    }
                }
            }
            col_idx += 1;
            comment = false;
        }
    }

    if string_started || char_started {
        return Err(parse_code::ParseCodeError::StringSyntaxError(
            crate::error::parse_code::string_syntax_error::StringSyntaxError::new(
                ctx.get_current_file(),
                last_line.try_into().unwrap(),
                last_col.try_into().unwrap(),
            ),
        ));
    }

    Ok(())
}


mod test {
    use std::vec;

    // mark: passed
    #[test]
    fn test_gane_string_pool() {
        let mut context: crate::context::Context<crate::error::parse_code::string_syntax_error::StringSyntaxError> = crate::context::Context::new(
            "cat-test".to_string(),
            "test/test_gane_string_pool.cat".to_string(),
            1,
            1,
        );

        let code = r#"
            "hello world --fake comment\"" -- comment
            -- comment 2
            'c' -- a char
        "#;

        super::gane_string_pool(code, &mut context).unwrap_or_else(|e| {
            eprintln!("{}", e);
            std::process::exit(-1);
        });
        
        assert_eq!(context.get_string_pool(), vec!["hello world --fake comment\""]);
        assert_eq!(context.get_char_pool(), vec!['c'])
    }
}
