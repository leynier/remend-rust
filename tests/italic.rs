use remend::remend;

#[test]
fn translated_literal_cases() {
    let cases: &[(&str, &str)] = &[
        ("Text with __italic", "Text with __italic__"),
        ("__incomplete", "__incomplete__"),
        ("__xxx_", "__xxx__"),
        ("__bold text_", "__bold text__"),
        ("Text with __bold_", "Text with __bold__"),
        ("This is __bold text_", "This is __bold text__"),
        ("Text with *italic", "Text with *italic*"),
        ("*incomplete", "*incomplete*"),
        ("**bold** and *italic", "**bold** and *italic*"),
        ("234234*123", "234234*123"),
        ("hello*world", "hello*world"),
        ("test*123*test", "test*123*test"),
        ("abc*123", "abc*123"),
        ("123*abc", "123*abc"),
        ("This is *italic", "This is *italic*"),
        ("*word* and more text", "*word* and more text"),
        ("Text with _italic", "Text with _italic_"),
        ("_incomplete", "_incomplete_"),
        ("__bold__ and _italic", "__bold__ and _italic_"),
        ("\\_fully\\_escaped\\_", "\\_fully\\_escaped\\_"),
        ("café_price", "café_price"),
        ("naïve_approach", "naïve_approach"),
        ("some_variable_name", "some_variable_name"),
        ("test_123_value", "test_123_value"),
        ("Text with _italic\n", "Text with _italic_\n"),
        ("_incomplete\n\n", "_incomplete_\n\n"),
        ("Start _text\n", "Start _text_\n"),
    ];

    for (input, expected) in cases {
        assert_eq!(remend(input), *expected, "input: {input:?}");
    }
}

#[test]
fn upstream_literal_cases() {
    let cases: &[(&str, &str)] = &[
        ("Text with __italic", "Text with __italic__"),
        ("__incomplete", "__incomplete__"),
        ("__first__ and __second", "__first__ and __second__"),
        ("__xxx_", "__xxx__"),
        ("__bold text_", "__bold text__"),
        ("Text with __bold_", "Text with __bold__"),
        ("This is __bold text_", "This is __bold text__"),
        ("Text with *italic", "Text with *italic*"),
        ("*incomplete", "*incomplete*"),
        ("**bold** and *italic", "**bold** and *italic*"),
        ("234234*123", "234234*123"),
        ("hello*world", "hello*world"),
        ("test*123*test", "test*123*test"),
        (
            "*italic with some*var*name inside",
            "*italic with some*var*name inside*",
        ),
        (
            "test*var and *incomplete italic",
            "test*var and *incomplete italic*",
        ),
        (
            "\\*escaped asterisk and *italic",
            "\\*escaped asterisk and *italic*",
        ),
        ("*start \\* middle \\* end", "*start \\* middle \\* end*"),
        ("abc*123", "abc*123"),
        ("123*abc", "123*abc"),
        ("This is *italic", "This is *italic*"),
        ("*word* and more text", "*word* and more text"),
        ("Text with _italic", "Text with _italic_"),
        ("_incomplete", "_incomplete_"),
        ("__bold__ and _italic", "__bold__ and _italic_"),
        (
            "\\_escaped\\_ and _unescaped",
            "\\_escaped\\_ and _unescaped_",
        ),
        (
            "Start \\_escaped\\_ middle _incomplete",
            "Start \\_escaped\\_ middle _incomplete_",
        ),
        ("\\_fully\\_escaped\\_", "\\_fully\\_escaped\\_"),
        (
            "\\_escaped\\_ _complete_ pair",
            "\\_escaped\\_ _complete_ pair",
        ),
        ("café_price", "café_price"),
        ("naïve_approach", "naïve_approach"),
        ("some_variable_name", "some_variable_name"),
        ("test_123_value", "test_123_value"),
        ("_start with underscore", "_start with underscore_"),
        (
            "_italic with some_var_name inside",
            "_italic with some_var_name inside_",
        ),
        (
            "test_var and _incomplete italic",
            "test_var and _incomplete italic_",
        ),
        ("Text with _italic\n", "Text with _italic_\n"),
        ("_incomplete\n\n", "_incomplete_\n\n"),
        ("Start _text\n", "Start _text_\n"),
    ];

    for (input, expected) in cases {
        assert_eq!(remend(input), *expected, "input: {input:?}");
    }
}
