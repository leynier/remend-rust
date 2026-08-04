use crate::code_block_utils::is_inside_code_block;
use crate::index::LinkMode;
use crate::utils::{find_matching_closing_bracket, find_matching_opening_bracket};

pub(crate) fn handle_incomplete_links_and_images(text: &str, link_mode: LinkMode) -> String {
    if let Some(last_paren_index) = text.rfind("](") {
        if !is_inside_code_block(text, last_paren_index) {
            if let Some(result) = handle_incomplete_url(text, last_paren_index, link_mode) {
                return result;
            }
        }
    }

    let bytes = text.as_bytes();
    let mut index = bytes.len();
    while index > 0 {
        index -= 1;
        if bytes[index] == b'[' && !is_inside_code_block(text, index) {
            if let Some(result) = handle_incomplete_text(text, index, link_mode) {
                return result;
            }
        }
    }
    text.to_owned()
}

fn handle_incomplete_url(
    text: &str,
    last_paren_index: usize,
    link_mode: LinkMode,
) -> Option<String> {
    if text[last_paren_index + 2..].contains(')') {
        return None;
    }
    let open_bracket_index = find_matching_opening_bracket(text, last_paren_index)?;
    if is_inside_code_block(text, open_bracket_index) {
        return None;
    }

    let is_image = open_bracket_index > 0 && text.as_bytes()[open_bracket_index - 1] == b'!';
    let start_index = if is_image {
        open_bracket_index - 1
    } else {
        open_bracket_index
    };
    let before_link = &text[..start_index];
    if is_image {
        return Some(before_link.to_owned());
    }

    let link_text = &text[open_bracket_index + 1..last_paren_index];
    Some(match link_mode {
        LinkMode::Protocol => format!("{before_link}[{link_text}](streamdown:incomplete-link)"),
        LinkMode::TextOnly => format!("{before_link}{link_text}"),
    })
}

fn find_first_incomplete_bracket(text: &str, max_position: usize) -> usize {
    let bytes = text.as_bytes();
    let mut index = 0;
    while index < max_position {
        if bytes[index] == b'[' && !is_inside_code_block(text, index) {
            if index > 0 && bytes[index - 1] == b'!' {
                index += 1;
                continue;
            }
            match find_matching_closing_bracket(text, index) {
                None => return index,
                Some(closing_index) => {
                    if closing_index + 1 < text.len() && bytes[closing_index + 1] == b'(' {
                        if let Some(relative) = text[closing_index + 2..].find(')') {
                            index = closing_index + 2 + relative + 1;
                            continue;
                        }
                    }
                }
            }
        }
        index += 1;
    }
    max_position
}

fn handle_incomplete_text(text: &str, index: usize, link_mode: LinkMode) -> Option<String> {
    let bytes = text.as_bytes();
    let is_image = index > 0 && bytes[index - 1] == b'!';
    let open_index = if is_image { index - 1 } else { index };
    let after_open = &text[index + 1..];
    if !after_open.contains(']') {
        let before_link = &text[..open_index];
        return Some(if is_image {
            before_link.to_owned()
        } else {
            match link_mode {
                LinkMode::Protocol => format!("{text}](streamdown:incomplete-link)"),
                LinkMode::TextOnly => {
                    let first_incomplete = find_first_incomplete_bracket(text, index);
                    format!(
                        "{}{}",
                        &text[..first_incomplete],
                        &text[first_incomplete + 1..]
                    )
                }
            }
        });
    }

    if find_matching_closing_bracket(text, index).is_none() {
        let before_link = &text[..open_index];
        return Some(if is_image {
            before_link.to_owned()
        } else {
            match link_mode {
                LinkMode::Protocol => format!("{text}](streamdown:incomplete-link)"),
                LinkMode::TextOnly => {
                    let first_incomplete = find_first_incomplete_bracket(text, index);
                    format!(
                        "{}{}",
                        &text[..first_incomplete],
                        &text[first_incomplete + 1..]
                    )
                }
            }
        });
    }
    None
}
