use crate::code_block_utils::is_inside_code_block;
use crate::inline_code_handler::is_whitespace_or_markers;
use crate::utils::{
    char_after, char_before, is_horizontal_rule, is_inside_complete_inline_code,
    is_within_html_tag, is_within_link_or_image_url, is_within_math_block, is_word_char,
};

fn should_skip_asterisk(
    text: &str,
    index: usize,
    previous: Option<char>,
    next: Option<char>,
) -> bool {
    if previous == Some('\\') || is_within_math_block(text, index) {
        return true;
    }
    if previous != Some('*') && next == Some('*') {
        let next_next = text[index + 1..].chars().nth(1);
        return next_next != Some('*');
    }
    if previous == Some('*') {
        return true;
    }
    if previous.is_some_and(is_word_char) && next.is_some_and(is_word_char) {
        return true;
    }
    let previous_is_whitespace = previous.is_none_or(is_emphasis_whitespace);
    let next_is_whitespace = next.is_none_or(is_emphasis_whitespace);
    previous_is_whitespace && next_is_whitespace
}

fn is_emphasis_whitespace(character: char) -> bool {
    matches!(character, ' ' | '\t' | '\n')
}

/// Counts single asterisks that can act as emphasis delimiters.
pub fn count_single_asterisks(text: &str) -> usize {
    let bytes = text.as_bytes();
    let mut count = 0;
    let mut in_code_block = false;
    let mut index = 0;
    while index < bytes.len() {
        if index + 2 < bytes.len() && &bytes[index..index + 3] == b"```" {
            in_code_block = !in_code_block;
            index += 3;
            continue;
        }
        if in_code_block || bytes[index] != b'*' {
            index += 1;
            continue;
        }
        if !should_skip_asterisk(
            text,
            index,
            char_before(text, index),
            char_after(text, index),
        ) {
            count += 1;
        }
        index += 1;
    }
    count
}

fn should_skip_underscore(
    text: &str,
    index: usize,
    previous: Option<char>,
    next: Option<char>,
) -> bool {
    previous == Some('\\')
        || is_within_math_block(text, index)
        || is_within_link_or_image_url(text, index)
        || is_within_html_tag(text, index)
        || previous == Some('_')
        || next == Some('_')
        || (previous.is_some_and(is_word_char) && next.is_some_and(is_word_char))
}

/// Counts single underscores that can act as emphasis delimiters.
pub fn count_single_underscores(text: &str) -> usize {
    let bytes = text.as_bytes();
    let mut count = 0;
    let mut in_code_block = false;
    let mut index = 0;
    while index < bytes.len() {
        if index + 2 < bytes.len() && &bytes[index..index + 3] == b"```" {
            in_code_block = !in_code_block;
            index += 3;
            continue;
        }
        if in_code_block || bytes[index] != b'_' {
            index += 1;
            continue;
        }
        if !should_skip_underscore(
            text,
            index,
            char_before(text, index),
            char_after(text, index),
        ) {
            count += 1;
        }
        index += 1;
    }
    count
}

/// Counts triple-asterisk markers outside fenced code blocks.
pub fn count_triple_asterisks(text: &str) -> usize {
    let bytes = text.as_bytes();
    let mut count = 0;
    let mut consecutive = 0;
    let mut in_code_block = false;
    let mut index = 0;
    while index < bytes.len() {
        if index + 2 < bytes.len() && &bytes[index..index + 3] == b"```" {
            count += consecutive / 3;
            count += consecutive / 3;
            consecutive = 0;
            in_code_block = !in_code_block;
            index += 3;
            continue;
        }
        if in_code_block {
            index += 1;
        } else if bytes[index] == b'*' {
            consecutive += 1;
            index += 1;
        } else {
            count += consecutive / 3;
            consecutive = 0;
            index += 1;
        }
    }
    count + consecutive / 3
}

fn count_double_asterisks(text: &str) -> usize {
    count_marker_pairs_outside_code(text, b'*')
}

fn count_double_underscores(text: &str) -> usize {
    count_marker_pairs_outside_code(text, b'_')
}

fn count_marker_pairs_outside_code(text: &str, marker: u8) -> usize {
    let bytes = text.as_bytes();
    let mut count = 0;
    let mut in_code_block = false;
    let mut index = 0;
    while index < bytes.len() {
        if index + 2 < bytes.len() && &bytes[index..index + 3] == b"```" {
            in_code_block = !in_code_block;
            index += 3;
            continue;
        }
        if !in_code_block
            && index + 1 < bytes.len()
            && bytes[index] == marker
            && bytes[index + 1] == marker
        {
            count += 1;
            index += 2;
        } else {
            index += 1;
        }
    }
    count
}

fn should_skip_completion(text: &str, content: &str, marker_index: usize, marker: char) -> bool {
    if content.is_empty() || is_whitespace_or_markers(content) {
        return true;
    }
    let line_start = text[..marker_index]
        .rfind('\n')
        .map_or(0, |index| index + 1);
    let line_before_marker = &text[line_start..marker_index];
    if is_list_item_prefix(line_before_marker) && content.contains('\n') {
        return true;
    }
    is_horizontal_rule(text, marker_index, marker)
}

fn is_list_item_prefix(text: &str) -> bool {
    let mut chars = text.chars().peekable();
    while chars
        .peek()
        .is_some_and(|character| character.is_whitespace())
    {
        chars.next();
    }
    matches!(chars.next(), Some('-' | '*' | '+'))
        && chars.next().is_some_and(char::is_whitespace)
        && chars.all(char::is_whitespace)
}

/// Completes an incomplete bold marker.
pub fn handle_incomplete_bold(text: &str) -> String {
    let Some((marker_index, content)) = trailing_bold_match(text) else {
        return text.to_owned();
    };
    if is_inside_code_block(text, marker_index)
        || is_inside_complete_inline_code(text, marker_index)
        || should_skip_completion(text, content, marker_index, '*')
    {
        return text.to_owned();
    }
    if count_double_asterisks(text) % 2 == 1 {
        if content.ends_with('*') {
            format!("{text}*")
        } else {
            format!("{text}**")
        }
    } else {
        text.to_owned()
    }
}

fn trailing_bold_match(text: &str) -> Option<(usize, &str)> {
    let marker_index = text.rfind("**")?;
    let content = &text[marker_index + 2..];
    let content_without_optional_closer = content.strip_suffix('*').unwrap_or(content);
    (!content_without_optional_closer.contains('*')).then_some((marker_index, content))
}

/// Completes an incomplete double-underscore italic marker.
pub fn handle_incomplete_double_underscore_italic(text: &str) -> String {
    if let Some((marker_index, content)) = trailing_double_underscore_match(text) {
        if is_inside_code_block(text, marker_index)
            || is_inside_complete_inline_code(text, marker_index)
            || should_skip_completion(text, content, marker_index, '_')
        {
            return text.to_owned();
        }
        return if count_double_underscores(text) % 2 == 1 {
            format!("{text}__")
        } else {
            text.to_owned()
        };
    }

    let Some(marker_index) = text.rfind("__") else {
        return text.to_owned();
    };
    let content = &text[marker_index + 2..];
    if !content.ends_with('_') {
        return text.to_owned();
    }
    let body = &content[..content.len() - 1];
    if body.is_empty()
        || body.contains('_')
        || is_inside_code_block(text, marker_index)
        || is_inside_complete_inline_code(text, marker_index)
    {
        return text.to_owned();
    }
    if count_double_underscores(text) % 2 == 1 {
        format!("{text}_")
    } else {
        text.to_owned()
    }
}

fn trailing_double_underscore_match(text: &str) -> Option<(usize, &str)> {
    let marker_index = text.rfind("__")?;
    let content = &text[marker_index + 2..];
    (!content.contains('_')).then_some((marker_index, content))
}

fn find_first_single_asterisk(text: &str) -> Option<usize> {
    let bytes = text.as_bytes();
    let mut in_code_block = false;
    let mut index = 0;
    while index < bytes.len() {
        if index + 2 < bytes.len() && &bytes[index..index + 3] == b"```" {
            in_code_block = !in_code_block;
            index += 3;
            continue;
        }
        if !in_code_block && bytes[index] == b'*' {
            let previous = char_before(text, index);
            let next = char_after(text, index);
            if previous != Some('*')
                && next != Some('*')
                && previous != Some('\\')
                && !is_within_math_block(text, index)
            {
                let previous_is_whitespace = previous.is_none_or(is_emphasis_whitespace);
                let next_is_whitespace = next.is_none_or(is_emphasis_whitespace);
                if !((previous_is_whitespace && next_is_whitespace)
                    || (previous.is_some_and(is_word_char) && next.is_some_and(is_word_char)))
                {
                    return Some(index);
                }
            }
        }
        index += 1;
    }
    None
}

/// Completes an incomplete single-asterisk italic marker.
pub fn handle_incomplete_single_asterisk_italic(text: &str) -> String {
    let Some(_) = text.rfind('*') else {
        return text.to_owned();
    };
    let Some(first_marker) = find_first_single_asterisk(text) else {
        return text.to_owned();
    };
    if is_inside_code_block(text, first_marker)
        || is_inside_complete_inline_code(text, first_marker)
    {
        return text.to_owned();
    }
    let content = &text[first_marker + 1..];
    if content.is_empty() || is_whitespace_or_markers(content) {
        return text.to_owned();
    }
    if count_single_asterisks(text) % 2 == 1 {
        format!("{text}*")
    } else {
        text.to_owned()
    }
}

fn find_first_single_underscore(text: &str) -> Option<usize> {
    let bytes = text.as_bytes();
    let mut in_code_block = false;
    let mut index = 0;
    while index < bytes.len() {
        if index + 2 < bytes.len() && &bytes[index..index + 3] == b"```" {
            in_code_block = !in_code_block;
            index += 3;
            continue;
        }
        if !in_code_block && bytes[index] == b'_' {
            let previous = char_before(text, index);
            let next = char_after(text, index);
            if previous != Some('_')
                && next != Some('_')
                && previous != Some('\\')
                && !is_within_math_block(text, index)
                && !is_within_link_or_image_url(text, index)
                && !(previous.is_some_and(is_word_char) && next.is_some_and(is_word_char))
            {
                return Some(index);
            }
        }
        index += 1;
    }
    None
}

fn insert_closing_underscore(text: &str) -> String {
    let end = text.trim_end_matches('\n').len();
    if end < text.len() {
        format!("{}_{}", &text[..end], &text[end..])
    } else {
        format!("{text}_")
    }
}

fn handle_trailing_asterisks_for_underscore(text: &str) -> Option<String> {
    if !text.ends_with("**") {
        return None;
    }
    let without_trailing = &text[..text.len() - 2];
    if count_double_asterisks(without_trailing) % 2 != 1 {
        return None;
    }
    let first_double = without_trailing.find("**")?;
    let underscore = find_first_single_underscore(without_trailing)?;
    (first_double < underscore).then(|| format!("{without_trailing}_**"))
}

/// Completes an incomplete single-underscore italic marker.
pub fn handle_incomplete_single_underscore_italic(text: &str) -> String {
    let Some(_) = text.rfind('_') else {
        return text.to_owned();
    };
    let Some(first_marker) = find_first_single_underscore(text) else {
        return text.to_owned();
    };
    let content = &text[first_marker + 1..];
    if content.is_empty() || is_whitespace_or_markers(content) {
        return text.to_owned();
    }
    if is_inside_code_block(text, first_marker)
        || is_inside_complete_inline_code(text, first_marker)
    {
        return text.to_owned();
    }
    if count_single_underscores(text) % 2 == 1 {
        handle_trailing_asterisks_for_underscore(text)
            .unwrap_or_else(|| insert_closing_underscore(text))
    } else {
        text.to_owned()
    }
}

fn are_bold_italic_markers_balanced(text: &str) -> bool {
    count_double_asterisks(text) % 2 == 0 && count_single_asterisks(text) % 2 == 0
}

/// Completes an incomplete triple-asterisk marker.
pub fn handle_incomplete_bold_italic(text: &str) -> String {
    if text.len() >= 4 && text.chars().all(|character| character == '*') {
        return text.to_owned();
    }
    let Some((marker_index, content)) = trailing_bold_italic_match(text) else {
        return text.to_owned();
    };
    if content.is_empty()
        || is_whitespace_or_markers(content)
        || is_inside_code_block(text, marker_index)
        || is_inside_complete_inline_code(text, marker_index)
        || is_horizontal_rule(text, marker_index, '*')
    {
        return text.to_owned();
    }
    if count_triple_asterisks(text) % 2 == 1 {
        if are_bold_italic_markers_balanced(text) {
            text.to_owned()
        } else {
            format!("{text}***")
        }
    } else {
        text.to_owned()
    }
}

fn trailing_bold_italic_match(text: &str) -> Option<(usize, &str)> {
    let marker_index = text.rfind("***")?;
    let content = &text[marker_index + 3..];
    (!content.contains('*')).then_some((marker_index, content))
}
