use remend::{LinkMode, RemendOptions, remend, remend_with_options};

#[test]
fn incomplete_links_and_images_follow_upstream_contract() {
    assert_eq!(remend("[text"), "[text](streamdown:incomplete-link)");
    assert_eq!(remend("[text](url"), "[text](streamdown:incomplete-link)");
    assert_eq!(remend("![alt"), "");
    assert_eq!(remend("![alt](url"), "");
    assert_eq!(remend("[text](url)"), "[text](url)");

    let options = RemendOptions {
        link_mode: LinkMode::TextOnly,
        ..Default::default()
    };
    assert_eq!(remend_with_options("[text", &options), "text");
    assert_eq!(remend_with_options("[text](url", &options), "text");
    assert_eq!(
        remend_with_options("[text] [incomplete", &options),
        "[text] incomplete"
    );
}

#[test]
fn nested_links_and_code_are_not_rewritten_as_links() {
    assert_eq!(
        remend("[outer [inner] text"),
        "[outer [inner] text](streamdown:incomplete-link)"
    );
    assert_eq!(remend("`[not a link`"), "`[not a link`");
    assert_eq!(remend("![outer [inner] text"), "");
}

#[test]
fn upstream_literal_cases() {
    let cases: &[(&str, &str)] = &[
        (
            "Text with [incomplete link",
            "Text with [incomplete link](streamdown:incomplete-link)",
        ),
        (
            "Text [partial",
            "Text [partial](streamdown:incomplete-link)",
        ),
        (
            "[outer [nested] text](incomplete",
            "[outer [nested] text](streamdown:incomplete-link)",
        ),
        (
            "[link with [inner] content](http://incomplete",
            "[link with [inner] content](streamdown:incomplete-link)",
        ),
        (
            "Text [foo [bar] baz](",
            "Text [foo [bar] baz](streamdown:incomplete-link)",
        ),
        (
            "Check out [this lin",
            "Check out [this lin](streamdown:incomplete-link)",
        ),
        (
            "Visit [our site](https://exa",
            "Visit [our site](streamdown:incomplete-link)",
        ),
        (
            "Text [outer [inner",
            "Text [outer [inner](streamdown:incomplete-link)",
        ),
        (
            "[foo [bar [baz",
            "[foo [bar [baz](streamdown:incomplete-link)",
        ),
        (
            "Text [outer [inner]",
            "Text [outer [inner]](streamdown:incomplete-link)",
        ),
        (
            "[link [nested] text",
            "[link [nested] text](streamdown:incomplete-link)",
        ),
    ];

    for (input, expected) in cases {
        assert_eq!(remend(input), *expected, "input: {input:?}");
    }
}
