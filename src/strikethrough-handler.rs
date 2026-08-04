use crate::code_block_utils::is_inside_code_block;
use crate::inline_code_handler::is_whitespace_or_markers;
use crate::utils::is_inside_complete_inline_code;

pub(crate) fn handle_incomplete_strikethrough(text: &str) -> String {
    if let Some(marker_index) = trailing_double_tilde_marker(text) {
        let content = &text[marker_index + 2..];
        if content.is_empty() || is_whitespace_or_markers(content) {
            return text.to_owned();
        }
        if is_inside_code_block(text, marker_index)
            || is_inside_complete_inline_code(text, marker_index)
        {
            return text.to_owned();
        }
        if count_double_tildes(text) % 2 == 1 {
            return format!("{text}~~");
        }
    } else if let Some(marker_index) = trailing_half_tilde_marker(text) {
        if is_inside_code_block(text, marker_index)
            || is_inside_complete_inline_code(text, marker_index)
        {
            return text.to_owned();
        }
        if count_double_tildes(text) % 2 == 1 {
            return format!("{text}~");
        }
    }
    text.to_owned()
}

fn trailing_double_tilde_marker(text: &str) -> Option<usize> {
    let index = text.rfind("~~")?;
    let content = &text[index + 2..];
    content
        .chars()
        .all(|character| character != '~')
        .then_some(index)
}

fn trailing_half_tilde_marker(text: &str) -> Option<usize> {
    let marker = text.find("~~")?;
    let content = &text[marker + 2..];
    if !content.ends_with('~') {
        return None;
    }
    let before_closing = &content[..content.len() - 1];
    if before_closing.is_empty() || before_closing.contains('~') {
        return None;
    }
    Some(marker)
}

fn count_double_tildes(text: &str) -> usize {
    text.match_indices("~~").count()
}
