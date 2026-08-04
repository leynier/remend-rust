use remend::remend;

#[test]
fn translated_literal_cases() {
    let cases: &[(&str, &str)] = &[
        ("---", "---"),
        ("----", "----"),
        ("-----", "-----"),
        ("***", "***"),
        ("****", "****"),
        ("*****", "*****"),
        ("___", "___"),
        ("____", "____"),
        ("_____", "_____"),
        ("- - -", "- - -"),
        ("* * *", "* * *"),
        ("_ _ _", "_ _ _"),
        ("-  -  -", "-  -  -"),
        ("*   *   *", "*   *   *"),
        ("_    _    _", "_    _    _"),
        ("Some text\n\n---", "Some text\n\n---"),
        ("Some text\n\n***", "Some text\n\n***"),
        ("Some text\n\n___", "Some text\n\n___"),
        ("---\n\nSome text", "---\n\nSome text"),
        ("***\n\nSome text", "***\n\nSome text"),
        ("___\n\nSome text", "___\n\nSome text"),
        ("Text with **bold", "Text with **bold**"),
        ("Text with --", "Text with --"),
        ("--", "--"),
        ("**", "**"),
        ("__", "__"),
        ("Text\n\n--", "Text\n\n--"),
        ("****", "****"),
        ("*****", "*****"),
        ("   ---", "   ---"),
        ("  ***", "  ***"),
        (" ___", " ___"),
        ("Text\n***", "Text\n***"),
    ];

    for (input, expected) in cases {
        assert_eq!(remend(input), *expected, "input: {input:?}");
    }
}

#[test]
fn upstream_literal_cases() {
    let cases: &[(&str, &str)] = &[
        ("---", "---"),
        ("----", "----"),
        ("-----", "-----"),
        ("***", "***"),
        ("****", "****"),
        ("*****", "*****"),
        ("___", "___"),
        ("____", "____"),
        ("_____", "_____"),
        ("- - -", "- - -"),
        ("* * *", "* * *"),
        ("_ _ _", "_ _ _"),
        ("-  -  -", "-  -  -"),
        ("*   *   *", "*   *   *"),
        ("_    _    _", "_    _    _"),
        (
            "Text before\n***\nText after",
            "Text before\n***\nText after",
        ),
        (
            "Text before\n___\nText after",
            "Text before\n___\nText after",
        ),
        ("Some text\n\n---", "Some text\n\n---"),
        ("Some text\n\n***", "Some text\n\n***"),
        ("Some text\n\n___", "Some text\n\n___"),
        ("---\n\nSome text", "---\n\nSome text"),
        ("***\n\nSome text", "***\n\nSome text"),
        ("___\n\nSome text", "___\n\nSome text"),
        (
            "Section 1\n\n---\n\nSection 2\n\n---\n\nSection 3",
            "Section 1\n\n---\n\nSection 2\n\n---\n\nSection 3",
        ),
        ("Text with **bold", "Text with **bold**"),
        ("Text with --", "Text with --"),
        ("--", "--"),
        ("**", "**"),
        ("__", "__"),
        ("Text\n\n--", "Text\n\n--"),
        ("   ---", "   ---"),
        ("  ***", "  ***"),
        (" ___", " ___"),
        (
            "This is not a --- horizontal rule",
            "This is not a --- horizontal rule",
        ),
        ("Text\n***", "Text\n***"),
    ];

    for (input, expected) in cases {
        assert_eq!(remend(input), *expected, "input: {input:?}");
    }
}
