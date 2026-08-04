/// Protects a partial setext heading marker from being rendered prematurely.
pub(crate) fn handle_incomplete_setext_heading(text: &str) -> String {
    let Some(last_newline) = text.rfind('\n') else {
        return text.to_owned();
    };
    let last_line = &text[last_newline + 1..];
    let previous_content = &text[..last_newline];
    let trimmed = last_line.trim();

    if is_partial_setext_line(trimmed, last_line, '-')
        && previous_content
            .rsplit('\n')
            .next()
            .is_some_and(|line| !line.trim().is_empty())
    {
        return format!("{text}\u{200B}");
    }
    if is_partial_setext_line(trimmed, last_line, '=')
        && previous_content
            .rsplit('\n')
            .next()
            .is_some_and(|line| !line.trim().is_empty())
    {
        return format!("{text}\u{200B}");
    }
    text.to_owned()
}

fn is_partial_setext_line(trimmed: &str, original: &str, marker: char) -> bool {
    let marker_count = trimmed
        .chars()
        .filter(|&character| character == marker)
        .count();
    (marker_count == 1 || marker_count == 2)
        && trimmed.chars().all(|character| character == marker)
        && !original.chars().last().is_some_and(char::is_whitespace)
}
