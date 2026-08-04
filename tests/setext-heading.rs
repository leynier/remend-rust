use remend::{RemendOptions, remend, remend_with_options};

#[test]
fn partial_setext_markers_are_protected() {
    assert_eq!(remend("Heading\n-"), "Heading\n-\u{200b}");
    assert_eq!(remend("Heading\n--"), "Heading\n--\u{200b}");
    assert_eq!(remend("Heading\n="), "Heading\n=\u{200b}");
    assert_eq!(remend("Heading\n=="), "Heading\n==\u{200b}");
    assert_eq!(remend("Heading\n---"), "Heading\n---");
    assert_eq!(remend("Heading\n==="), "Heading\n===");
    assert_eq!(remend("-"), "-");
    assert_eq!(remend("\n-"), "\n-");
    assert_eq!(remend("Some text\n----"), "Some text\n----");
    assert_eq!(remend("Some text\n  -"), "Some text\n  -\u{200b}");
    assert_eq!(remend("Some text\n-x"), "Some text\n-x");
    assert_eq!(
        remend("Some text\n- Item 1\n- Item 2"),
        "Some text\n- Item 1\n- Item 2"
    );
    assert_eq!(remend("Some text\n- "), "Some text\n-\u{200b}");
    assert_eq!(
        remend("Line 1\nLine 2\nLine 3\n-"),
        "Line 1\nLine 2\nLine 3\n-\u{200b}"
    );
}

#[test]
fn setext_completion_can_be_disabled() {
    let options = RemendOptions {
        setext_headings: false,
        ..Default::default()
    };
    assert_eq!(remend_with_options("Heading\n-", &options), "Heading\n-");
}
