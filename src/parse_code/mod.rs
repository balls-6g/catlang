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
    // C....... you just can't beet me.... you just can't
    // .....your dirty namespace, ....your dirty ABI
    // your dirty macros......, dirty syntax....
    // your dirty community...
    // AHAHAHAHHAHAHAHAHA
    // .....

    let codes: Vec<Vec<char>> = code
        .lines()
        .map(String::from)
        .collect::<Vec<_>>()
        .iter()
        .map(|v| v.chars().map(|s| s).collect::<Vec<_>>())
        .collect();
    let mut string_buf = String::new();
    let mut string_started = false;

    // this is the last postion that the '"' found
    let mut last_line = 0;
    let mut last_col = 0;

    // I can't stop nesting...
    for l in 0..codes.len() {
        for c in 0..codes.get(l).iter().len() {
            if codes.get(l).unwrap().get(c) != Some(&'"')
                && codes.get(l).unwrap().get(c) != Some(&'\\')
            {
                if string_started {
                    string_buf.push(extract_some(codes.get(l).unwrap().get(c).copied()));
                } else {
                    string_started = true
                }
                continue;
            }

            if l >= 1
                && codes.get(l).unwrap().get(c) == Some(&'"')
                && codes.get(l).unwrap().get(c - 1) != Some(&'\\')
            {
                if string_started {
                    ctx.string_pool_push(string_buf.clone());
                    string_started = false;
                    string_buf.clear();
                    last_line = l;
                    last_col = c;
                } else {
                    string_started = true
                }
                continue;
            }

            if l >= 1
                && codes.get(l).unwrap().get(c) == Some(&'"')
                && codes.get(l).unwrap().get(c - 1) == Some(&'\\')
            {
                string_buf.push('"');
                continue;
            }
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
    use crate::error::debug_trait::CompilerError;

    use super::*;

    #[test]
    fn test_gane_string_pool<E: CompilerError>() {
        let mut context: crate::context::Context<E: CompilerError> =
            Context::new("cat-test".to_string(), "app.cat".to_string(), 0, 0);
        let code = r#"
        Hello, world"
        "#;

        let _ = gane_string_pool(code, &mut context);
        assert_eq!(context.get_string_pool(), vec!["Hello, world"]);
    }
}
