use remend::remend;

#[test]
fn translated_literal_cases() {
    let cases: &[(&str, &str)] = &[
        ("**bold and *italic", "**bold and *italic*"),
        ("*italic with **bold", "*italic with **bold***"),
        ("**bold with `code", "**bold with `code**`"),
        ("**bold with $x^2", "**bold with $x^2**"),
        ("**_text", "**_text_**"),
        ("_italic and **bold", "_italic and **bold**_"),
    ];

    for (input, expected) in cases {
        assert_eq!(remend(input), *expected, "input: {input:?}");
    }
}

#[test]
fn upstream_literal_cases() {
    let cases: &[(&str, &str)] = &[
        ("**bold and *italic", "**bold and *italic*"),
        (
            "Text with [link and **bold",
            "Text with [link and **bold](streamdown:incomplete-link)",
        ),
        ("*italic with **bold", "*italic with **bold***"),
        ("**bold with `code", "**bold with `code**`"),
        ("~~strike with **bold", "~~strike with **bold**~~"),
        ("**bold with $x^2", "**bold with $x^2**"),
        (
            "**bold *italic `code ~~strike",
            "**bold *italic `code ~~strike*`",
        ),
        ("**bold and *bold-italic***", "**bold and *bold-italic***"),
        (
            "combined **_bold and italic",
            "combined **_bold and italic_**",
        ),
        ("**_text", "**_text_**"),
        ("_italic and **bold", "_italic and **bold**_"),
    ];

    for (input, expected) in cases {
        assert_eq!(remend(input), *expected, "input: {input:?}");
    }
}
