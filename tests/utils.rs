use remend::{
    count_single_asterisks, count_single_underscores, count_triple_asterisks,
    find_matching_closing_bracket, find_matching_opening_bracket, is_horizontal_rule,
    is_within_code_block, is_within_html_tag, is_within_link_or_image_url, is_within_math_block,
    is_word_char,
};

#[test]
fn context_utilities_match_upstream_positions() {
    assert!(is_word_char('a'));
    assert!(is_word_char('é'));
    assert!(is_word_char('_'));
    assert!(!is_word_char('-'));

    assert!(is_within_code_block("before ```code", 14));
    assert!(!is_within_code_block("before ```code```", 17));
    assert!(is_within_math_block("$$x$y$$z", 5));
    assert!(is_within_link_or_image_url("[text](url)", 8));
    assert!(!is_within_link_or_image_url("[text](url) after", 13));
    assert!(is_within_html_tag("<div data_attr", 12));
    assert!(!is_within_html_tag("<div> text", 6));

    assert_eq!(
        find_matching_opening_bracket("[outer [inner] text]", 19),
        Some(0)
    );
    assert_eq!(find_matching_opening_bracket("[text]", 5), Some(0));
    assert_eq!(
        find_matching_opening_bracket("[outer [inner] text]", 13),
        Some(7)
    );
    assert_eq!(find_matching_opening_bracket("some text]", 9), None);
    assert_eq!(
        find_matching_closing_bracket("[outer [inner] text]", 0),
        Some(19)
    );
    assert_eq!(find_matching_closing_bracket("[text]", 0), Some(5));
    assert_eq!(
        find_matching_closing_bracket("[outer [inner] text]", 7),
        Some(13)
    );
    assert_eq!(find_matching_closing_bracket("[some text", 0), None);
    assert_eq!(find_matching_closing_bracket("é[text]", 2), Some(7));

    assert!(is_horizontal_rule("* * *", 0, '*'));
    assert!(is_horizontal_rule("*\t*\t*", 0, '*'));
    assert!(!is_horizontal_rule("text * * *", 5, '*'));

    assert_eq!(count_triple_asterisks("text***"), 1);
    assert_eq!(count_triple_asterisks("***```"), 2);
    assert_eq!(count_triple_asterisks("```\n***\n```"), 0);
    assert_eq!(count_single_asterisks("*italic*"), 2);
    assert_eq!(count_single_underscores("_italic_"), 2);
}

#[test]
fn utilities_reject_invalid_utf8_offsets() {
    let text = "é[text]";
    assert!(!is_within_code_block(text, 1));
    assert_eq!(find_matching_closing_bracket(text, 1), None);
    assert_eq!(find_matching_opening_bracket(text, 1), None);
    assert!(!is_within_html_tag(text, 1));
}
