use crate::code_block_utils::is_inside_code_block;

/// Escapes comparison operators in list items when they would otherwise be
/// parsed as blockquote markers.
pub(crate) fn handle_comparison_operators(text: &str) -> String {
    if !text.contains('>') {
        return text.to_owned();
    }

    let mut result = String::with_capacity(text.len());
    let mut line_start = 0;
    for line_end in text
        .match_indices('\n')
        .map(|(index, _)| index)
        .chain(std::iter::once(text.len()))
    {
        let line = &text[line_start..line_end];
        if let Some(operator_index) = comparison_operator_index(line) {
            if is_inside_code_block(text, line_start + operator_index) {
                result.push_str(line);
            } else {
                result.push_str(&line[..operator_index]);
                result.push_str("\\>");
                result.push_str(&line[operator_index + 1..]);
            }
        } else {
            result.push_str(line);
        }
        if line_end < text.len() {
            result.push('\n');
        }
        line_start = line_end + 1;
    }
    result
}

fn comparison_operator_index(line: &str) -> Option<usize> {
    let bytes = line.as_bytes();
    let mut index = 0;
    while index < bytes.len() && line[index..].chars().next()?.is_whitespace() {
        index += line[index..].chars().next()?.len_utf8();
    }
    if index >= bytes.len() {
        return None;
    }

    if matches!(bytes[index], b'-' | b'*' | b'+') {
        index += 1;
    } else {
        let digit_start = index;
        while index < bytes.len() && bytes[index].is_ascii_digit() {
            index += 1;
        }
        if index == digit_start || index >= bytes.len() || !matches!(bytes[index], b'.' | b')') {
            return None;
        }
        index += 1;
    }

    let space_start = index;
    while index < bytes.len() && line[index..].chars().next()?.is_whitespace() {
        index += line[index..].chars().next()?.len_utf8();
    }
    if index == space_start || index >= bytes.len() || bytes[index] != b'>' {
        return None;
    }
    let operator_index = index;
    index += 1;
    if index < bytes.len() && bytes[index] == b'=' {
        index += 1;
    }
    while index < bytes.len() && line[index..].chars().next()?.is_whitespace() {
        index += line[index..].chars().next()?.len_utf8();
    }
    if index < bytes.len() && bytes[index] == b'$' {
        index += 1;
    }
    if index < bytes.len() && bytes[index].is_ascii_digit() {
        Some(operator_index)
    } else {
        None
    }
}
