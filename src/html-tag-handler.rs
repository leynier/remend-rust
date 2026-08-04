use crate::code_block_utils::is_inside_code_block;

/// Removes an incomplete HTML tag at the end of streamed text.
pub(crate) fn handle_incomplete_html_tag(text: &str) -> String {
    let Some((start, _)) = text.match_indices('<').find(|(index, _)| {
        let suffix = &text[*index..];
        let first = suffix[1..].chars().next();
        !suffix.contains('>')
            && first.is_some_and(|character| character.is_ascii_alphabetic() || character == '/')
    }) else {
        return text.to_owned();
    };
    if is_inside_code_block(text, start) {
        return text.to_owned();
    }
    text[..start].trim_end().to_owned()
}
