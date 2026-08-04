use remend::remend;

#[test]
fn translated_literal_cases() {
    let cases: &[(&str, &str)] = &[
        ("**bold _und", "**bold _und_**"),
        ("> Quote with **bold", "> Quote with **bold**"),
        ("Text **bold `code", "Text **bold `code**`"),
    ];

    for (input, expected) in cases {
        assert_eq!(remend(input), *expected, "input: {input:?}");
    }
}

#[test]
fn upstream_literal_cases() {
    let cases: &[(&str, &str)] = &[
        ("This is **bold with *ital", "This is **bold with *ital*"),
        ("**bold _und", "**bold _und_**"),
        (
            "# Main Title\n## Subtitle with **emph",
            "# Main Title\n## Subtitle with **emph**",
        ),
        ("> Quote with **bold", "> Quote with **bold**"),
        (
            "| Col1 | Col2 |\n|------|------|\n| **dat",
            "| Col1 | Col2 |\n|------|------|\n| **dat**",
        ),
        (
            "1. First item\n   - Nested with `code\n2. Second",
            "1. First item\n   - Nested with `code\n2. Second`",
        ),
        ("Text **bold `code", "Text **bold `code**`"),
    ];

    for (input, expected) in cases {
        assert_eq!(remend(input), *expected, "input: {input:?}");
    }
}
