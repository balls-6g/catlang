use crate::context::Context;
use crate::error::debug_trait::CompilerError;
use crate::error::parse_code;

fn extract_some<T>(v: Option<T>) -> T {
    match v {
        Some(value) => value,
        _ => std::process::exit(-1),
    }
}

pub fn gane_string_pool<E: CompilerError>(
    code: &str,
    ctx: &mut Context<E>,
) -> Result<(), crate::error::parse_code::ParseCodeError> {
    let mut string_buf = String::new();
    let mut string_started = false;
    let mut last_line = 0;
    let mut last_col = 0;

    for (line_idx, line) in code.lines().enumerate() {
        let mut chars = line.chars().peekable();
        let mut col_idx = 0;

        while let Some(ch) = chars.next() {
            match ch {
                '"' => {
                    let escaped = col_idx > 0 && line.chars().nth(col_idx - 1) == Some('\\');
                    if escaped {
                        string_buf.push('"');
                    } else {
                        if string_started {
                            ctx.string_pool_push(string_buf.clone());
                            string_buf.clear();
                            string_started = false;
                            last_line = line_idx;
                            last_col = col_idx;
                        } else {
                            string_started = true;
                        }
                    }
                }
                '\\' => {
                    // Lookahead for escaped quote
                    if let Some(&next_ch) = chars.peek() {
                        if next_ch == '"' {
                            chars.next(); // consume the quote
                            string_buf.push('"');
                            col_idx += 1;
                        } else {
                            string_buf.push('\\');
                        }
                    } else {
                        string_buf.push('\\');
                    }
                }
                _ => {
                    if string_started {
                        string_buf.push(ch);
                    }
                }
            }
            col_idx += 1;
        }
    }

    if string_started {
        return Err(parse_code::ParseCodeError::StringSyntaxError(
            crate::error::parse_code::string_syntaxerror::StringSyntaxError::new(
                ctx.get_current_file(),
                last_line.try_into().unwrap(),
                last_col.try_into().unwrap(),
            ),
        ));
    }

    Ok(())
}


mod test {
    use super::*;

    #[test]
    fn test_gane_string_pool() {
        let mut context: Context<crate::error::parse_code::string_syntaxerror::StringSyntaxError> = crate::context::Context::new(
            "cat-test".to_string(),
            "test/test_gane_string_pool.cat".to_string(),
            1,
            1,
        );

        let code = r#"
            "hello world"
        "#;

        gane_string_pool(code, &mut context).unwrap_or_else(|e| {
            eprintln!("{}", e);
            std::process::exit(-1);
        });
    }
}
