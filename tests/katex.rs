use remend::remend;

#[test]
fn translated_literal_cases() {
    let cases: &[(&str, &str)] = &[
        ("Text with $$formula", "Text with $$formula$$"),
        ("$$incomplete", "$$incomplete$$"),
        ("$$x + y = z", "$$x + y = z$$"),
        ("$$formula$", "$$formula$$"),
        ("$$x = y$", "$$x = y$$"),
        ("$$\nx = 1\ny = 2", "$$\nx = 1\ny = 2\n$$"),
        ("Text with $formula", "Text with $formula"),
        ("$incomplete", "$incomplete"),
        ("$first$ and $second", "$first$ and $second"),
        ("$x + y = z", "$x + y = z"),
        ("$$$", "$$$$$"),
        ("$$$$", "$$$$"),
    ];

    for (input, expected) in cases {
        assert_eq!(remend(input), *expected, "input: {input:?}");
    }
}

#[test]
fn upstream_literal_cases() {
    let cases: &[(&str, &str)] = &[
        ("Text with $$formula", "Text with $$formula$$"),
        ("$$incomplete", "$$incomplete$$"),
        ("$$first$$ and $$second", "$$first$$ and $$second$$"),
        ("$$x + y = z", "$$x + y = z$$"),
        ("$$formula$", "$$formula$$"),
        ("$$x = y$", "$$x = y$$"),
        ("$$\nx = 1\ny = 2", "$$\nx = 1\ny = 2\n$$"),
        ("Text with $formula", "Text with $formula"),
        ("$incomplete", "$incomplete"),
        ("$first$ and $second", "$first$ and $second"),
        ("$$block$$ and $inline", "$$block$$ and $inline"),
        ("$x + y = z", "$x + y = z"),
        ("$$$", "$$$$$"),
        ("$$$$", "$$$$"),
    ];

    for (input, expected) in cases {
        assert_eq!(remend(input), *expected, "input: {input:?}");
    }
}
