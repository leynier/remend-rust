use crate::code_block_utils::is_inside_code_block;
use crate::utils::is_word_char;

/// Escapes a single tilde between word characters.
pub(crate) fn handle_single_tilde_escape(text: &str) -> String {
    if !text.contains('~') {
        return text.to_owned();
    }

    let mut result = String::with_capacity(text.len());
    let mut last = 0;
    for (index, character) in text.char_indices() {
        if character != '~' {
            continue;
        }
        let previous = text[..index].chars().next_back();
        let next = text[index + 1..].chars().next();
        let is_single = previous != Some('~') && next != Some('~');
        if is_single
            && previous.is_some_and(is_word_char)
            && next.is_some_and(is_word_char)
            && !is_inside_code_block(text, index)
        {
            result.push_str(&text[last..index]);
            result.push_str("\\~");
            last = index + 1;
        }
    }
    result.push_str(&text[last..]);
    result
}
