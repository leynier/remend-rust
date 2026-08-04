use remend::remend;

#[test]
fn translated_literal_cases() {
    let cases: &[(&str, &str)] = &[
        ("- __", "- __"),
        ("- **", "- **"),
        ("- __\n- **", "- __\n- **"),
        ("\n- __\n- **", "\n- __\n- **"),
        ("* __\n* **", "* __\n* **"),
        ("+ __\n+ **", "+ __\n+ **"),
        ("- __ text after", "- __ text after__"),
        ("- ** text after", "- ** text after**"),
        ("- ***", "- ***"),
        ("- *", "- *"),
        ("- _", "- _"),
        ("- ~~", "- ~~"),
        ("- `", "- `"),
        ("- **text\nmore text", "- **text\nmore text"),
    ];

    for (input, expected) in cases {
        assert_eq!(remend(input), *expected, "input: {input:?}");
    }
}

#[test]
fn upstream_literal_cases() {
    let cases: &[(&str, &str)] = &[
        (
            "- Item 1\n- Item 2 with **bol",
            "- Item 1\n- Item 2 with **bol**",
        ),
        ("- __", "- __"),
        ("- **", "- **"),
        ("- __\n- **", "- __\n- **"),
        ("\n- __\n- **", "\n- __\n- **"),
        ("* __\n* **", "* __\n* **"),
        ("+ __\n+ **", "+ __\n+ **"),
        ("- __ text after", "- __ text after__"),
        ("- ** text after", "- ** text after**"),
        ("- __\n- Normal item\n- **", "- __\n- Normal item\n- **"),
        ("- ***", "- ***"),
        ("- *", "- *"),
        ("- _", "- _"),
        ("- ~~", "- ~~"),
        ("- `", "- `"),
        ("- **text\nmore text", "- **text\nmore text"),
        ("* **content\n* Another item", "* **content\n* Another item"),
    ];

    for (input, expected) in cases {
        assert_eq!(remend(input), *expected, "input: {input:?}");
    }
}
