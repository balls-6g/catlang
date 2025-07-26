use crate::error::parse_code;

fn extract_some(c: Option<u8>) -> u8 {
    match c {
        Some(v) => v,
        _ => std::process::exit(-1),
    }
}

pub fn gane_string_pool(code: String, pool: &mut Vec<String>) -> Result<(), ()> {
    // C....... you just can't beet me.... you just can't
    // .....your dirty namespace, ....your dirty ABI
    // your dirty macros......, dirty syntax....
    // your dirty community...
    // AHAHAHAHHAHAHAHAHA
    let codes: Vec<u8> = code.into();
    let mut string_buf = String::new();
    let mut string_started = false;

    for l in 0..codes.len() {
        if codes.get(l) != Some(&b'"') && codes.get(l) != Some(&b'\\') {
            if string_started {
                string_buf.push(extract_some(codes.get(l).copied()).into());
            } else {
                string_started = true
            }
            continue;
        }

        if l >= 1 && codes.get(l) == Some(&b'"') && codes.get(l - 1) != Some(&b'\\') {
            if string_started {
                pool.push(string_buf.clone());
            } else {
                string_started = true
            }
            continue;
        }

        if string_started {
            string_buf.push(extract_some(codes.get(l).copied()).into());
        }
    }

    if !string_started {
        return Err(());
    }

    Ok(())
}

mod test {
    use super::*;

    #[test]
    fn test_gane_string_pool() {
        let mut string_pool: Vec<String> = Vec::new();
        let code = r#"
        "Hello, world"
        "#
        .to_string();
        let _ = gane_string_pool(code, &mut string_pool);
        assert_eq!(string_pool, vec!["Hello, world"]);
    }
}
