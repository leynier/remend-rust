use remend::remend;

#[test]
fn translated_literal_cases() {
    let cases: &[(&str, &str)] = &[
        ("Text with `code", "Text with `code`"),
        ("`incomplete", "`incomplete`"),
        ("\\` *italic", "\\` *italic*"),
        ("`**bold`", "`**bold`"),
        ("`*italic`", "`*italic`"),
        ("`~~strikethrough`", "`~~strikethrough`"),
        ("**bold", "**bold**"),
        ("*italic", "*italic*"),
        ("~~strike", "~~strike~~"),
        ("`code` **bold", "`code` **bold**"),
    ];

    for (input, expected) in cases {
        assert_eq!(remend(input), *expected, "input: {input:?}");
    }
}

#[test]
fn upstream_literal_cases() {
    let cases: &[(&str, &str)] = &[
        ("Text with `code", "Text with `code`"),
        ("`incomplete", "`incomplete`"),
        ("```\nblock\n```\n`inline", "```\nblock\n```\n`inline`"),
        ("\\`not code\\` **bold", "\\`not code\\` **bold**"),
        ("\\` *italic", "\\` *italic*"),
        ("`**bold`", "`**bold`"),
        ("`*italic`", "`*italic`"),
        ("`~~strikethrough`", "`~~strikethrough`"),
        ("**bold", "**bold**"),
        ("*italic", "*italic*"),
        ("~~strike", "~~strike~~"),
        ("`code` **bold", "`code` **bold**"),
    ];

    for (input, expected) in cases {
        assert_eq!(remend(input), *expected, "input: {input:?}");
    }
}
