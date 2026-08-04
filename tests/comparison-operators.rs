use remend::remend;

#[test]
fn translated_literal_cases() {
    let cases: &[(&str, &str)] = &[
        ("- > 25: rich", "- \\> 25: rich"),
        ("* > 25: rich", "* \\> 25: rich"),
        ("+ > 25: rich", "+ \\> 25: rich"),
        ("1. > 25: rich", "1. \\> 25: rich"),
        ("2) > 10: high", "2) \\> 10: high"),
        ("  - > 25: rich", "  - \\> 25: rich"),
        ("    - > 5: expensive", "    - \\> 5: expensive"),
        ("- >= 10: high", "- \\>= 10: high"),
        ("- > $100: expensive", "- \\> $100: expensive"),
        ("> Some blockquote", "> Some blockquote"),
        ("> 25 is a number", "> 25 is a number"),
        ("- > Some quoted text", "- > Some quoted text"),
        (">25", ">25"),
        ("- >25: rich", "- \\>25: rich"),
    ];

    for (input, expected) in cases {
        assert_eq!(remend(input), *expected, "input: {input:?}");
    }
}

#[test]
fn comparison_operators_inside_inline_code_are_untouched() {
    assert_eq!(remend("`- > 25`"), "`- > 25`");
    assert_eq!(remend("`- > 25`\n- > 10"), "`- > 25`\n- \\> 10");
}

#[test]
fn upstream_literal_cases() {
    let cases: &[(&str, &str)] = &[
        ("- > 25: rich", "- \\> 25: rich"),
        ("* > 25: rich", "* \\> 25: rich"),
        ("+ > 25: rich", "+ \\> 25: rich"),
        ("1. > 25: rich", "1. \\> 25: rich"),
        ("2) > 10: high", "2) \\> 10: high"),
        ("  - > 25: rich", "  - \\> 25: rich"),
        ("    - > 5: expensive", "    - \\> 5: expensive"),
        ("- >= 10: high", "- \\>= 10: high"),
        ("- > $100: expensive", "- \\> $100: expensive"),
        ("> Some blockquote", "> Some blockquote"),
        ("> 25 is a number", "> 25 is a number"),
        ("- > Some quoted text", "- > Some quoted text"),
        ("- > Read more about this", "- > Read more about this"),
        (">25", ">25"),
        ("- >25: rich", "- \\>25: rich"),
    ];

    for (input, expected) in cases {
        assert_eq!(remend(input), *expected, "input: {input:?}");
    }
}
