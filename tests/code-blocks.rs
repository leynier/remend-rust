use remend::remend;

#[test]
fn translated_literal_cases() {
    let cases: &[(&str, &str)] = &[("```\ncode here", "```\ncode here")];

    for (input, expected) in cases {
        assert_eq!(remend(input), *expected, "input: {input:?}");
    }
}

#[test]
fn upstream_literal_cases() {
    let cases: &[(&str, &str)] = &[
        ("```javascript\nconst x = 5;", "```javascript\nconst x = 5;"),
        ("```\ncode here", "```\ncode here"),
        ("```python\ndef hello():", "```python\ndef hello():"),
        (
            "Some text\n```js\nconsole.log",
            "Some text\n```js\nconsole.log",
        ),
    ];

    for (input, expected) in cases {
        assert_eq!(remend(input), *expected, "input: {input:?}");
    }
}
