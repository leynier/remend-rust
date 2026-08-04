use remend::remend;

#[test]
fn translated_literal_cases() {
    let cases: &[(&str, &str)] = &[
        ("Text ending with *", "Text ending with *"),
        ("Text ending with **", "Text ending with **"),
        ("****", "****"),
        ("``", "``"),
        ("**", "**"),
        ("__", "__"),
        ("***", "***"),
        ("*", "*"),
        ("_", "_"),
        ("~~", "~~"),
        ("`", "`"),
        ("** __", "** __"),
        ("\n** __\n", "\n** __\n"),
        ("* _ ~~ `", "* _ ~~ `"),
        ("** ", "**"),
        (" **", " **"),
        ("**text", "**text**"),
        ("__text", "__text__"),
        ("*text", "*text*"),
        ("_text", "_text_"),
        ("~~text", "~~text~~"),
        ("`text", "`text`"),
        ("*", "*"),
        ("**", "**"),
        ("`", "`"),
        ("text**", "text**"),
        ("text*", "text*"),
        ("text`", "text`"),
        ("text$", "text$"),
        ("text~~", "text~~"),
        ("text **bold", "text **bold**"),
        ("text\n**bold", "text\n**bold**"),
        ("text\t`code", "text\t`code`"),
        ("**émoji 🎉", "**émoji 🎉**"),
        ("`código", "`código`"),
        ("**&lt;tag&gt;", "**&lt;tag&gt;**"),
        ("`&amp;", "`&amp;`"),
        ("3 + 2 - 5 * 0 = ?", "3 + 2 - 5 * 0 = ?"),
        ("5 * 0", "5 * 0"),
        ("x * y", "x * y"),
        ("a * b = c", "a * b = c"),
        ("2 * 3 * 4", "2 * 3 * 4"),
        ("5 * 0 and *italic", "5 * 0 and *italic*"),
    ];

    for (input, expected) in cases {
        assert_eq!(remend(input), *expected, "input: {input:?}");
    }
}

#[test]
fn upstream_literal_cases() {
    let cases: &[(&str, &str)] = &[
        ("Text ending with *", "Text ending with *"),
        ("Text ending with **", "Text ending with **"),
        ("****", "****"),
        ("``", "``"),
        ("**", "**"),
        ("__", "__"),
        ("***", "***"),
        ("*", "*"),
        ("_", "_"),
        ("~~", "~~"),
        ("`", "`"),
        ("** __", "** __"),
        ("\n** __\n", "\n** __\n"),
        ("* _ ~~ `", "* _ ~~ `"),
        ("** ", "**"),
        (" **", " **"),
        ("  **  ", "  **  "),
        ("**text", "**text**"),
        ("__text", "__text__"),
        ("*text", "*text*"),
        ("_text", "_text_"),
        ("~~text", "~~text~~"),
        ("`text", "`text`"),
        ("text**", "text**"),
        ("text*", "text*"),
        ("text`", "text`"),
        ("text$", "text$"),
        ("text~~", "text~~"),
        ("text **bold", "text **bold**"),
        ("text\n**bold", "text\n**bold**"),
        ("text\t`code", "text\t`code`"),
        ("**émoji 🎉", "**émoji 🎉**"),
        ("`código", "`código`"),
        ("**&lt;tag&gt;", "**&lt;tag&gt;**"),
        ("`&amp;", "`&amp;`"),
        ("3 + 2 - 5 * 0 = ?", "3 + 2 - 5 * 0 = ?"),
        ("5 * 0", "5 * 0"),
        ("x * y", "x * y"),
        ("a * b = c", "a * b = c"),
        ("2 * 3 * 4", "2 * 3 * 4"),
        ("5 * 0 and *italic", "5 * 0 and *italic*"),
    ];

    for (input, expected) in cases {
        assert_eq!(remend(input), *expected, "input: {input:?}");
    }
}
