use remend::remend;

#[test]
fn translated_literal_cases() {
    let cases: &[(&str, &str)] = &[
        ("***incomplete", "***incomplete***"),
        ("text ***", "text ***"),
        ("text ****", "text ****"),
        ("text *****", "text *****"),
        ("text ******", "text ******"),
        ("text***", "text***"),
        ("word****", "word****"),
        ("end******", "end******"),
        ("***start***end***", "***start***end***"),
        ("***text***", "***text***"),
        ("***incomplete", "***incomplete***"),
        ("***word text***", "***word text***"),
    ];

    for (input, expected) in cases {
        assert_eq!(remend(input), *expected, "input: {input:?}");
    }
}

#[test]
fn upstream_literal_cases() {
    let cases: &[(&str, &str)] = &[
        ("Text with ***bold-italic", "Text with ***bold-italic***"),
        ("***incomplete", "***incomplete***"),
        ("***first*** and ***second", "***first*** and ***second***"),
        ("*italic* **bold** ***both", "*italic* **bold** ***both***"),
        ("***Starting bold-italic", "***Starting bold-italic***"),
        ("***bold-italic with `code", "***bold-italic with `code***`"),
        ("text ***", "text ***"),
        ("text ****", "text ****"),
        ("text *****", "text *****"),
        ("text ******", "text ******"),
        ("text***", "text***"),
        ("word****", "word****"),
        ("end******", "end******"),
        ("***start***end***", "***start***end***"),
        ("***text***", "***text***"),
        ("***word text***", "***word text***"),
        (
            "Combined **bold and *italic*** text",
            "Combined **bold and *italic*** text",
        ),
        (
            "**bold and *italic*** more text",
            "**bold and *italic*** more text",
        ),
        (
            "test **bold and *italic*** end",
            "test **bold and *italic*** end",
        ),
        (
            "- Combined **bold and *italic*** text",
            "- Combined **bold and *italic*** text",
        ),
    ];

    for (input, expected) in cases {
        assert_eq!(remend(input), *expected, "input: {input:?}");
    }
}
