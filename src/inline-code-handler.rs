use crate::code_block_utils::count_single_backticks;

pub(crate) fn handle_incomplete_inline_code(text: &str) -> String {
    if let Some(result) = handle_inline_triple_backticks(text) {
        return result;
    }

    let Some(marker_index) = text.rfind('`') else {
        return text.to_owned();
    };
    let content = &text[marker_index + 1..];
    if is_inside_incomplete_code_block(text)
        || content.is_empty()
        || is_whitespace_or_markers(content)
    {
        return text.to_owned();
    }
    if count_single_backticks(text) % 2 == 1 {
        format!("{text}`")
    } else {
        text.to_owned()
    }
}

fn handle_inline_triple_backticks(text: &str) -> Option<String> {
    if !text.starts_with("```") || text.contains('\n') {
        return None;
    }
    let rest = &text[3..];
    if rest.ends_with("``") && !rest.ends_with("```") && !rest[..rest.len() - 2].contains('`') {
        return Some(format!("{text}`"));
    }
    if rest.ends_with("```") && !rest[..rest.len() - 3].contains('`') {
        return Some(text.to_owned());
    }
    None
}

fn is_inside_incomplete_code_block(text: &str) -> bool {
    let mut count = 0;
    let mut search_start = 0;
    while let Some(relative) = text[search_start..].find("```") {
        count += 1;
        search_start += relative + 3;
    }
    count % 2 == 1
}

pub(crate) fn is_whitespace_or_markers(text: &str) -> bool {
    text.chars()
        .all(|character| character.is_whitespace() || matches!(character, '_' | '~' | '*' | '`'))
}
