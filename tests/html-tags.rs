use remend::remend;

#[test]
fn translated_literal_cases() {
    let cases: &[(&str, &str)] = &[
        ("Hello <div", "Hello"),
        ("Hello <custom", "Hello"),
        ("Hello <casecard", "Hello"),
        ("Text <MyComponent", "Text"),
        ("Hello </div", "Hello"),
        ("Hello </custom", "Hello"),
        ("<div>content</di", "<div>content"),
        ("Hello <div class=\"foo", "Hello"),
        ("Hello <div class=", "Hello"),
        ("Hello <a href=\"https://example.com", "Hello"),
        ("<custom data-id", ""),
        ("Hello <div>", "Hello <div>"),
        ("<div>content</div>", "<div>content</div>"),
        ("<br/>", "<br/>"),
        ("<img src='test'>", "<img src='test'>"),
        ("3 < 5", "3 < 5"),
        ("x < y", "x < y"),
        ("if a <", "if a <"),
        ("value <1", "value <1"),
        ("```\n<div\n```", "```\n<div\n```"),
        ("```html\n<custom", "```html\n<custom"),
        ("`<div`", "`<div`"),
        ("<div", ""),
        ("<custom", ""),
        ("</div", ""),
        ("Some text here\n\n<casecard", "Some text here"),
        ("<div>Hello</div> <span", "<div>Hello</div>"),
    ];

    for (input, expected) in cases {
        assert_eq!(remend(input), *expected, "input: {input:?}");
    }
}

#[test]
fn upstream_literal_cases() {
    let cases: &[(&str, &str)] = &[
        ("Hello <div", "Hello"),
        ("Hello <custom", "Hello"),
        ("Hello <casecard", "Hello"),
        ("Text <MyComponent", "Text"),
        ("Hello </div", "Hello"),
        ("Hello </custom", "Hello"),
        ("<div>content</di", "<div>content"),
        ("Hello <div class=\"foo", "Hello"),
        ("Hello <div class=", "Hello"),
        ("Hello <a href=\"https://example.com", "Hello"),
        ("<custom data-id", ""),
        ("Hello <div>", "Hello <div>"),
        ("<div>content</div>", "<div>content</div>"),
        ("<br/>", "<br/>"),
        ("<img src='test'>", "<img src='test'>"),
        ("3 < 5", "3 < 5"),
        ("x < y", "x < y"),
        ("if a <", "if a <"),
        ("value <1", "value <1"),
        ("```\n<div\n```", "```\n<div\n```"),
        ("```html\n<custom", "```html\n<custom"),
        ("`<div`", "`<div`"),
        ("<div", ""),
        ("<custom", ""),
        ("</div", ""),
        ("Some text here\n\n<casecard", "Some text here"),
        ("# Heading\n\nParagraph <custom", "# Heading\n\nParagraph"),
        ("<div>Hello</div> <span", "<div>Hello</div>"),
        (
            "<a target=\"_blank\" href=\"https://link.com\">word</a>",
            "<a target=\"_blank\" href=\"https://link.com\">word</a>",
        ),
        (
            "<a target=\"_blank\">link</a>",
            "<a target=\"_blank\">link</a>",
        ),
        (
            "<iframe src=\"x\" sandbox=\"allow_scripts\">",
            "<iframe src=\"x\" sandbox=\"allow_scripts\">",
        ),
    ];

    for (input, expected) in cases {
        assert_eq!(remend(input), *expected, "input: {input:?}");
    }
}

#[test]
fn html_matching_uses_the_first_incomplete_tag() {
    assert_eq!(remend("before <div attr=\"<custom"), "before");
}
