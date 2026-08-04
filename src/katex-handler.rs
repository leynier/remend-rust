fn is_triple_backtick(text: &str, index: usize) -> bool {
    let bytes = text.as_bytes();
    (index >= 2 && &bytes[index - 2..=index] == b"```")
        || (index >= 1 && index + 1 < bytes.len() && &bytes[index - 1..index + 2] == b"```")
        || (index + 2 < bytes.len() && &bytes[index..index + 3] == b"```")
}

fn count_dollar_pairs(text: &str) -> usize {
    let bytes = text.as_bytes();
    let mut count = 0;
    let mut in_inline_code = false;
    let mut index = 0;
    while index + 1 < bytes.len() {
        if bytes[index] == b'`' && !is_triple_backtick(text, index) {
            in_inline_code = !in_inline_code;
        }
        if !in_inline_code && &bytes[index..index + 2] == b"$$" {
            count += 1;
            index += 2;
        } else {
            index += 1;
        }
    }
    count
}

fn count_single_dollars(text: &str) -> usize {
    let bytes = text.as_bytes();
    let mut count = 0;
    let mut in_inline_code = false;
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'\\' {
            index += if index + 1 < bytes.len() { 2 } else { 1 };
            continue;
        }
        if bytes[index] == b'`' && !is_triple_backtick(text, index) {
            in_inline_code = !in_inline_code;
            index += 1;
            continue;
        }
        if !in_inline_code && bytes[index] == b'$' {
            if index + 1 < bytes.len() && bytes[index + 1] == b'$' {
                index += 2;
            } else {
                count += 1;
                index += 1;
            }
        } else {
            index += 1;
        }
    }
    count
}

pub(crate) fn handle_incomplete_block_katex(text: &str) -> String {
    if count_dollar_pairs(text) % 2 == 0 {
        return text.to_owned();
    }
    if text.ends_with('$') && !text.ends_with("$$") {
        return format!("{text}$");
    }
    let first_dollar = text.find("$$");
    let has_newline_after_start = first_dollar.is_some_and(|index| text[index..].contains('\n'));
    if has_newline_after_start && !text.ends_with('\n') {
        format!("{text}\n$$")
    } else {
        format!("{text}$$")
    }
}

pub(crate) fn handle_incomplete_inline_katex(text: &str) -> String {
    if count_single_dollars(text) % 2 == 1 {
        format!("{text}$")
    } else {
        text.to_owned()
    }
}
