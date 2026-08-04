use remend::remend;

#[test]
fn translated_literal_cases() {
    let cases: &[(&str, &str)] = &[
        ("Text with **bold", "Text with **bold**"),
        ("**incomplete", "**incomplete**"),
        ("**xxx*", "**xxx**"),
        ("**bold text*", "**bold text**"),
        ("Text with **bold*", "Text with **bold**"),
        ("This is **bold text*", "This is **bold text**"),
    ];

    for (input, expected) in cases {
        assert_eq!(remend(input), *expected, "input: {input:?}");
    }
}

#[test]
fn upstream_literal_cases() {
    let cases: &[(&str, &str)] = &[
        ("Text with **bold", "Text with **bold**"),
        ("**incomplete", "**incomplete**"),
        ("**first** and **second", "**first** and **second**"),
        ("Here is some **bold tex", "Here is some **bold tex**"),
        ("**xxx*", "**xxx**"),
        ("**bold text*", "**bold text**"),
        ("Text with **bold*", "Text with **bold**"),
        ("This is **bold text*", "This is **bold text**"),
    ];

    for (input, expected) in cases {
        assert_eq!(remend(input), *expected, "input: {input:?}");
    }
}
