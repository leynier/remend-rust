/// Returns whether a byte position is inside fenced or inline code.
pub(crate) fn is_inside_code_block(text: &str, position: usize) -> bool {
    let bytes = text.as_bytes();
    if position > bytes.len() {
        return false;
    }

    let mut in_inline_code = false;
    let mut in_multiline_code = false;
    let mut index = 0;
    while index < position {
        if bytes[index] == b'\\' && index + 1 < bytes.len() && bytes[index + 1] == b'`' {
            index += 2;
            continue;
        }
        if index + 2 < bytes.len() && &bytes[index..index + 3] == b"```" {
            in_multiline_code = !in_multiline_code;
            index += 3;
            continue;
        }
        if !in_multiline_code && bytes[index] == b'`' {
            in_inline_code = !in_inline_code;
        }
        index += 1;
    }
    in_inline_code || in_multiline_code
}

pub(crate) fn is_part_of_triple_backtick(text: &str, index: usize) -> bool {
    let bytes = text.as_bytes();
    (index + 2 < bytes.len() && &bytes[index..index + 3] == b"```")
        || (index > 0 && index + 1 < bytes.len() && &bytes[index - 1..index + 2] == b"```")
        || (index > 1 && &bytes[index - 2..index + 1] == b"```")
}

pub(crate) fn count_single_backticks(text: &str) -> usize {
    let bytes = text.as_bytes();
    let mut count = 0;
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'\\' && index + 1 < bytes.len() && bytes[index + 1] == b'`' {
            index += 2;
            continue;
        }
        if bytes[index] == b'`' && !is_part_of_triple_backtick(text, index) {
            count += 1;
        }
        index += 1;
    }
    count
}

pub(crate) mod is_inside_code_block {
    pub fn is_within_complete_inline_code(text: &str, position: usize) -> bool {
        let bytes = text.as_bytes();
        if position > bytes.len() {
            return false;
        }

        let mut in_inline_code = false;
        let mut in_multiline_code = false;
        let mut inline_code_start = None;
        let mut index = 0;
        while index < bytes.len() {
            if bytes[index] == b'\\' && index + 1 < bytes.len() && bytes[index + 1] == b'`' {
                index += 2;
                continue;
            }
            if index + 2 < bytes.len() && &bytes[index..index + 3] == b"```" {
                in_multiline_code = !in_multiline_code;
                index += 3;
                continue;
            }
            if !in_multiline_code && bytes[index] == b'`' {
                if in_inline_code {
                    if inline_code_start.is_some_and(|start| start < position && position < index) {
                        return true;
                    }
                    in_inline_code = false;
                    inline_code_start = None;
                } else {
                    in_inline_code = true;
                    inline_code_start = Some(index);
                }
            }
            index += 1;
        }
        false
    }
}
