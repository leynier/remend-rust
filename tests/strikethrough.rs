use remend::remend;

#[test]
fn translated_literal_cases() {
    let cases: &[(&str, &str)] = &[
        ("Text with ~~strike", "Text with ~~strike~~"),
        ("~~incomplete", "~~incomplete~~"),
        ("~~xxx~", "~~xxx~~"),
        ("~~strike text~", "~~strike text~~"),
        ("Text with ~~strike~", "Text with ~~strike~~"),
    ];

    for (input, expected) in cases {
        assert_eq!(remend(input), *expected, "input: {input:?}");
    }
}

#[test]
fn upstream_literal_cases() {
    let cases: &[(&str, &str)] = &[
        ("Text with ~~strike", "Text with ~~strike~~"),
        ("~~incomplete", "~~incomplete~~"),
        ("~~first~~ and ~~second", "~~first~~ and ~~second~~"),
        ("~~xxx~", "~~xxx~~"),
        ("~~strike text~", "~~strike text~~"),
        ("Text with ~~strike~", "Text with ~~strike~~"),
        ("This is ~~strikethrough~", "This is ~~strikethrough~~"),
    ];

    for (input, expected) in cases {
        assert_eq!(remend(input), *expected, "input: {input:?}");
    }
}
