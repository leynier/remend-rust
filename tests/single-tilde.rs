use remend::remend;

#[test]
fn translated_literal_cases() {
    let cases: &[(&str, &str)] = &[
        ("20~25°C", "20\\~25°C"),
        ("20~25°C。20~25°C", "20\\~25°C。20\\~25°C"),
        ("foo~bar", "foo\\~bar"),
        ("~~strikethrough~~", "~~strikethrough~~"),
        ("~hello", "~hello"),
        ("hello~", "hello~"),
        ("hello ~ world", "hello ~ world"),
        ("```\n20~25\n```", "```\n20~25\n```"),
        ("`20~25`", "`20~25`"),
        ("20~25 and ~~strike", "20\\~25 and ~~strike~~"),
    ];

    for (input, expected) in cases {
        assert_eq!(remend(input), *expected, "input: {input:?}");
    }
}

#[test]
fn upstream_literal_cases() {
    let cases: &[(&str, &str)] = &[
        ("20~25°C", "20\\~25°C"),
        ("20~25°C。20~25°C", "20\\~25°C。20\\~25°C"),
        ("foo~bar", "foo\\~bar"),
        ("~~strikethrough~~", "~~strikethrough~~"),
        ("~hello", "~hello"),
        ("hello~", "hello~"),
        ("hello ~ world", "hello ~ world"),
        ("```\n20~25\n```", "```\n20~25\n```"),
        ("`20~25`", "`20~25`"),
        ("20~25 and ~~strike", "20\\~25 and ~~strike~~"),
    ];

    for (input, expected) in cases {
        assert_eq!(remend(input), *expected, "input: {input:?}");
    }
}
