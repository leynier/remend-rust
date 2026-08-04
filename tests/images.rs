use remend::remend;

#[test]
fn translated_literal_cases() {
    let cases: &[(&str, &str)] = &[
        ("Text with ![incomplete image", "Text with "),
        ("![partial", ""),
        ("See ![the diag", "See "),
        ("![logo](./assets/log", ""),
        ("Text ![outer [inner]", "Text "),
        ("![nested [brackets] text", ""),
        ("Start ![foo [bar] baz", "Start "),
    ];

    for (input, expected) in cases {
        assert_eq!(remend(input), *expected, "input: {input:?}");
    }
}

#[test]
fn upstream_literal_cases() {
    let cases: &[(&str, &str)] = &[
        ("Text with ![incomplete image", "Text with "),
        ("![partial", ""),
        ("See ![the diag", "See "),
        ("![logo](./assets/log", ""),
        ("Text ![outer [inner]", "Text "),
        ("![nested [brackets] text", ""),
        ("Start ![foo [bar] baz", "Start "),
    ];

    for (input, expected) in cases {
        assert_eq!(remend(input), *expected, "input: {input:?}");
    }
}
