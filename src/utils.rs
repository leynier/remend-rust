/// Returns whether a character is a letter, number, or underscore.
pub fn is_word_char(character: char) -> bool {
    character.is_alphanumeric() || character == '_'
}

/// Returns whether a byte position is inside a fenced code block.
pub fn is_within_code_block(text: &str, position: usize) -> bool {
    let bytes = text.as_bytes();
    if position > bytes.len() || !text.is_char_boundary(position) {
        return false;
    }

    let mut in_code_block = false;
    let mut index = 0;
    while index < position {
        if index + 2 < bytes.len() && &bytes[index..index + 3] == b"```" {
            in_code_block = !in_code_block;
            index += 3;
        } else {
            index += 1;
        }
    }
    in_code_block
}

/// Finds the matching opening bracket before `close_index`.
pub fn find_matching_opening_bracket(text: &str, close_index: usize) -> Option<usize> {
    if close_index > text.len() || !text.is_char_boundary(close_index) {
        return None;
    }

    let bytes = text.as_bytes();
    let mut depth = 1;
    let mut index = close_index;
    while index > 0 {
        index -= 1;
        match bytes[index] {
            b']' => depth += 1,
            b'[' => {
                depth -= 1;
                if depth == 0 {
                    return Some(index);
                }
            }
            _ => {}
        }
    }
    None
}

/// Finds the matching closing bracket after `open_index`.
pub fn find_matching_closing_bracket(text: &str, open_index: usize) -> Option<usize> {
    if open_index >= text.len() || !text.is_char_boundary(open_index) {
        return None;
    }

    let bytes = text.as_bytes();
    let mut depth = 1;
    for (index, byte) in bytes.iter().enumerate().skip(open_index + 1) {
        match *byte {
            b'[' => depth += 1,
            b']' => {
                depth -= 1;
                if depth == 0 {
                    return Some(index);
                }
            }
            _ => {}
        }
    }
    None
}

/// Returns whether a byte position is inside an inline or block math span.
pub fn is_within_math_block(text: &str, position: usize) -> bool {
    let bytes = text.as_bytes();
    if position > bytes.len() || !text.is_char_boundary(position) {
        return false;
    }

    let mut in_inline_math = false;
    let mut in_block_math = false;
    let mut index = 0;
    while index < position {
        if bytes[index] == b'\\' && index + 1 < bytes.len() && bytes[index + 1] == b'$' {
            index += 2;
            continue;
        }
        if bytes[index] == b'$' {
            if index + 1 < bytes.len() && bytes[index + 1] == b'$' {
                in_block_math = !in_block_math;
                in_inline_math = false;
                index += 2;
            } else {
                if !in_block_math {
                    in_inline_math = !in_inline_math;
                }
                index += 1;
            }
        } else {
            index += 1;
        }
    }
    in_inline_math || in_block_math
}

fn is_before_closing_paren(text: &str, position: usize) -> bool {
    if position > text.len() || !text.is_char_boundary(position) {
        return false;
    }
    for character in text[position..].chars() {
        if character == ')' {
            return true;
        }
        if character == '\n' {
            return false;
        }
    }
    false
}

/// Returns whether a byte position is in a link or image URL.
pub fn is_within_link_or_image_url(text: &str, position: usize) -> bool {
    if position > text.len() || !text.is_char_boundary(position) {
        return false;
    }

    let bytes = text.as_bytes();
    let mut index = position;
    while index > 0 {
        index -= 1;
        match bytes[index] {
            b')' => return false,
            b'(' => {
                if index > 0 && bytes[index - 1] == b']' {
                    return is_before_closing_paren(text, position);
                }
                return false;
            }
            b'\n' => return false,
            _ => {}
        }
    }
    false
}

/// Returns whether a byte position is inside an HTML tag.
pub fn is_within_html_tag(text: &str, position: usize) -> bool {
    if position > text.len() || !text.is_char_boundary(position) {
        return false;
    }

    let bytes = text.as_bytes();
    let mut index = position;
    while index > 0 {
        index -= 1;
        match bytes[index] {
            b'>' => return false,
            b'<' => {
                let Some(next) = text[index + 1..].chars().next() else {
                    return false;
                };
                return next.is_ascii_alphabetic() || next == '/';
            }
            b'\n' => return false,
            _ => {}
        }
    }
    false
}

/// Returns whether a marker occurrence is on a horizontal-rule-only line.
pub fn is_horizontal_rule(text: &str, marker_index: usize, marker: char) -> bool {
    if marker_index > text.len() || !text.is_char_boundary(marker_index) {
        return false;
    }

    let line_start = text[..marker_index]
        .rfind('\n')
        .map_or(0, |newline| newline + 1);
    let line_end = text[marker_index..]
        .find('\n')
        .map_or(text.len(), |newline| marker_index + newline);
    let line = &text[line_start..line_end];

    let mut marker_count = 0;
    for character in line.chars() {
        if character == marker {
            marker_count += 1;
        } else if character != ' ' && character != '\t' {
            return false;
        }
    }
    marker_count >= 3
}

pub(crate) fn char_before(text: &str, index: usize) -> Option<char> {
    text.get(..index)?.chars().next_back()
}

pub(crate) fn char_after(text: &str, index: usize) -> Option<char> {
    text.get(index + 1..)?.chars().next()
}

pub(crate) fn is_inside_complete_inline_code(text: &str, position: usize) -> bool {
    crate::code_block_utils::is_inside_code_block::is_within_complete_inline_code(text, position)
}
