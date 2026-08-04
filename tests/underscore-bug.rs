use remend::remend;

#[test]
fn translated_literal_cases() {
    let cases: &[(&str, &str)] = &[
        ("hello_world", "hello_world"),
        ("hello_world_test", "hello_world_test"),
        ("MAX_VALUE", "MAX_VALUE"),
        ("_italic text", "_italic text_"),
        ("This is _italic", "This is _italic_"),
        ("_italic\n", "_italic_\n"),
        ("word_", "word_"),
        ("_privateVariable", "_privateVariable_"),
    ];

    for (input, expected) in cases {
        assert_eq!(remend(input), *expected, "input: {input:?}");
    }
}
